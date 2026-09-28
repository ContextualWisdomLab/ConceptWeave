//! Local atomic release issuance. The caller provisions a protected existing directory.

use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use conceptweave_client::{ReleaseMetadata, SemanticRelease, SemanticReleaseClient};
use conceptweave_domain::{PublicationState, TruthStatus};

use super::{
    GovernanceError, MAX_ARTIFACT_BYTES, PublishedModel, ReviewedAlignment, build_published,
    put_len, put_raw, put_text, sha256,
};

const MAX_RECORD_BYTES: usize = MAX_ARTIFACT_BYTES * 2 + 8;
static STAGE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A protected, local directory that issues each release ID once and never replaces it.
///
/// The directory must already exist and must allow only trusted writers. This adapter does not
/// distribute trusted client pins or authenticate stewards. It uses atomic hard links, so the
/// staging file and final release file must remain on the same filesystem.
#[derive(Debug, Clone)]
pub struct FilePublicationStore {
    root: PathBuf,
}

/// Durable publication or readback failed without issuing an unverified success.
#[derive(Debug)]
pub enum PublicationStoreError {
    /// The caller's configured directory is absent, unsafe, or not a directory.
    InvalidRoot,
    /// This release ID has already been issued, even if its content matches.
    DuplicateReleaseId,
    /// A record is incomplete or disagrees with the supplied trusted release.
    InvalidRecord,
    /// The caller's independently configured client did not admit the release.
    ReleaseNotAdmitted,
    /// The final link may exist, but directory durability could not be confirmed.
    UncertainCommit,
    /// The review or release contract failed before a store write.
    Governance(GovernanceError),
    /// A filesystem operation failed before a confirmed publication.
    Io(std::io::Error),
}

impl fmt::Display for PublicationStoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidRoot => "the publication directory is unavailable",
            Self::DuplicateReleaseId => "this release identity is already published",
            Self::InvalidRecord => "the stored release does not match the trusted release",
            Self::ReleaseNotAdmitted => "this release is not trusted for use",
            Self::UncertainCommit => "the publication outcome needs storage recovery review",
            Self::Governance(_) => "the governed release could not be created",
            Self::Io(_) => "the release could not be stored or read",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for PublicationStoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Governance(error) => Some(error),
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<GovernanceError> for PublicationStoreError {
    fn from(error: GovernanceError) -> Self {
        Self::Governance(error)
    }
}

impl From<std::io::Error> for PublicationStoreError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl FilePublicationStore {
    /// Opens an existing caller-protected directory; never creates a publication root implicitly.
    pub fn new(root: impl AsRef<Path>) -> Result<Self, PublicationStoreError> {
        let root = root
            .as_ref()
            .canonicalize()
            .map_err(|_| PublicationStoreError::InvalidRoot)?;
        let metadata = fs::metadata(&root).map_err(|_| PublicationStoreError::InvalidRoot)?;
        if !metadata.is_dir() {
            return Err(PublicationStoreError::InvalidRoot);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o077 != 0 {
                return Err(PublicationStoreError::InvalidRoot);
            }
            Ok(Self { root })
        }
        #[cfg(not(unix))]
        {
            Err(PublicationStoreError::InvalidRoot)
        }
    }

    /// Issues a reviewed release atomically and returns it only after durable storage succeeds.
    ///
    /// A second use of the same release ID fails, including an identical retry. If the directory
    /// sync fails after the final link, `UncertainCommit` requires a readback/recovery decision.
    pub fn publish(
        &self,
        reviewed: &ReviewedAlignment,
        metadata: ReleaseMetadata,
    ) -> Result<PublishedModel, PublicationStoreError> {
        let published = build_published(reviewed, metadata)?;
        self.issue_record(published.release(), published.artifact_bytes())?;
        Ok(published)
    }

    fn issue_record(
        &self,
        release: &SemanticRelease,
        artifact_bytes: &[u8],
    ) -> Result<(), PublicationStoreError> {
        if release.artifact_digest().as_str() != sha256(artifact_bytes)
            || artifact_bytes.len() > MAX_ARTIFACT_BYTES
            || release.truth_status() != TruthStatus::Authoritative
            || release.publication_state() != PublicationState::Published
        {
            return Err(PublicationStoreError::InvalidRecord);
        }
        let header = encode_header(release)?;
        let final_path = self.release_path(release.release_id());
        let (stage_path, mut stage) = self.create_stage()?;
        let write_result = (|| {
            stage.write_all(&u64::try_from(header.len()).unwrap().to_be_bytes())?;
            stage.write_all(&header)?;
            stage.write_all(artifact_bytes)?;
            let mut permissions = stage.metadata()?.permissions();
            permissions.set_readonly(true);
            stage.set_permissions(permissions)?;
            stage.sync_all()?;
            Ok::<(), std::io::Error>(())
        })();
        if let Err(error) = write_result {
            let _ = fs::remove_file(&stage_path);
            return Err(error.into());
        }
        let link_result = fs::hard_link(&stage_path, &final_path);
        let _ = fs::remove_file(&stage_path);
        match link_result {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                return Err(PublicationStoreError::DuplicateReleaseId);
            }
            Err(error) => return Err(error.into()),
        }
        File::open(&self.root)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| PublicationStoreError::UncertainCommit)?;
        Ok(())
    }

    /// Reads exact artifact bytes only after an independently pinned client admits the release.
    ///
    /// The stored manifest fields and artifact bytes must match the supplied release. A missing
    /// release ID returns `None`; malformed or changed records fail closed.
    pub fn read_verified(
        &self,
        client: &SemanticReleaseClient,
        release: &SemanticRelease,
    ) -> Result<Option<Vec<u8>>, PublicationStoreError> {
        client
            .validate_for_authoritative_use(release)
            .map_err(|_| PublicationStoreError::ReleaseNotAdmitted)?;
        let path = self.release_path(release.release_id());
        #[cfg(unix)]
        let opened = {
            use std::os::unix::fs::OpenOptionsExt;

            // A swapped symlink or FIFO must not redirect or stall readback.
            OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
                .open(path)
        };
        #[cfg(not(unix))]
        let opened = File::open(path);
        let file = match opened {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            #[cfg(unix)]
            Err(error) if error.raw_os_error() == Some(libc::ELOOP) => {
                return Err(PublicationStoreError::InvalidRecord);
            }
            Err(error) => return Err(error.into()),
        };
        let metadata = file.metadata()?;
        if !metadata.is_file() || metadata.len() > MAX_RECORD_BYTES as u64 {
            return Err(PublicationStoreError::InvalidRecord);
        }
        let mut record = Vec::new();
        file.take(MAX_RECORD_BYTES as u64 + 1)
            .read_to_end(&mut record)?;
        if record.len() < 8 || record.len() > MAX_RECORD_BYTES {
            return Err(PublicationStoreError::InvalidRecord);
        }
        let header_len = u64::from_be_bytes(record[..8].try_into().unwrap());
        let header_len =
            usize::try_from(header_len).map_err(|_| PublicationStoreError::InvalidRecord)?;
        let end = 8usize
            .checked_add(header_len)
            .ok_or(PublicationStoreError::InvalidRecord)?;
        if end > record.len() || record[8..end] != encode_header(release)? {
            return Err(PublicationStoreError::InvalidRecord);
        }
        let artifact = record[end..].to_vec();
        client
            .verify_detached_artifact(release, &artifact)
            .map_err(|_| PublicationStoreError::InvalidRecord)?;
        Ok(Some(artifact))
    }

    fn release_path(&self, release_id: &str) -> PathBuf {
        self.root
            .join(format!("{}.release", &sha256(release_id.as_bytes())[7..]))
    }

    fn create_stage(&self) -> Result<(PathBuf, File), PublicationStoreError> {
        for _ in 0..64 {
            let sequence = STAGE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = self
                .root
                .join(format!(".stage-{}-{sequence}", std::process::id()));
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            match options.open(&path) {
                Ok(file) => return Ok((path, file)),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.into()),
            }
        }
        Err(PublicationStoreError::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "publication staging names exhausted",
        )))
    }
}

fn encode_header(release: &SemanticRelease) -> Result<Vec<u8>, PublicationStoreError> {
    let mut header = Vec::new();
    put_text(&mut header, "conceptweave.immutable_release_record.v1")?;
    for value in [
        release.release_id(),
        release.contract_version(),
        release.ontology_version(),
        release.artifact_digest().as_str(),
        release.manifest_digest().as_str(),
    ] {
        put_text(&mut header, value)?;
    }
    put_raw(&mut header, &[3, 4])?;
    let mut provenance = release.provenance().iter().collect::<Vec<_>>();
    provenance.sort_by_key(|item| (item.source_id(), item.source_digest(), item.location()));
    put_len(&mut header, provenance.len())?;
    for evidence in provenance {
        for value in [
            evidence.source_id(),
            evidence.source_digest(),
            evidence.location(),
        ] {
            put_text(&mut header, value)?;
        }
    }
    let mut concept_ids = release.concept_ids().iter().collect::<Vec<_>>();
    concept_ids.sort();
    put_len(&mut header, concept_ids.len())?;
    for concept_id in concept_ids {
        put_text(&mut header, concept_id)?;
    }
    Ok(header)
}

#[cfg(all(test, unix))]
mod tests {
    use std::os::unix::fs::PermissionsExt;

    use conceptweave_client::{ReleaseDigest, TrustedReleaseManifest};
    use conceptweave_domain::EvidenceReference;

    use super::*;

    #[test]
    fn write_failure_removes_owned_staging_files() {
        if std::env::var("CW_PUBLICATION_WRITE_FAULT_CHILD").as_deref() != Ok("1") {
            let parent_root = std::env::temp_dir().join(format!(
                "cw-write-fault-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            fs::create_dir(&parent_root).unwrap();
            fs::set_permissions(&parent_root, fs::Permissions::from_mode(0o700)).unwrap();
            // Keep the quota above LLVM profile size and below the complete record size.
            let mut child = std::process::Command::new("/bin/sh")
                .args([
                    "-c",
                    "trap '' XFSZ; ulimit -f 8192; exec \"$1\" --exact \"$2\"",
                    "cw-write-fault",
                ])
                .arg(std::env::current_exe().unwrap())
                .arg("file_store::tests::write_failure_removes_owned_staging_files")
                .env("CW_PUBLICATION_WRITE_FAULT_CHILD", "1")
                .env(
                    "CW_PUBLICATION_WRITE_FAULT_ROOT",
                    parent_root.join("record_store"),
                )
                .spawn()
                .unwrap();
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
            loop {
                if let Some(status) = child.try_wait().unwrap() {
                    fs::remove_dir_all(&parent_root).unwrap();
                    assert!(status.success());
                    return;
                }
                if std::time::Instant::now() >= deadline {
                    child.kill().unwrap();
                    child.wait().unwrap();
                    fs::remove_dir_all(&parent_root).unwrap();
                    panic!("owned publication fault child timed out");
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
        let root = PathBuf::from(std::env::var_os("CW_PUBLICATION_WRITE_FAULT_ROOT").unwrap());
        fs::create_dir(&root).unwrap();
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
        let store = FilePublicationStore::new(&root).unwrap();
        let bytes = vec![7; 8 * 1024 * 1024];
        let release = SemanticRelease::new(
            ReleaseMetadata::new("fault-release", "1.0.0", "unit-ontology").unwrap(),
            TruthStatus::Authoritative,
            PublicationState::Published,
            ReleaseDigest::new(&sha256(&bytes)).unwrap(),
            vec![EvidenceReference::new("source", "digest", "location").unwrap()],
            vec!["unit.concept".into()],
        )
        .unwrap();
        let result = store.issue_record(&release, &bytes);
        let retained = fs::read_dir(&root).unwrap().count();
        fs::remove_dir_all(&root).unwrap();
        assert!(matches!(result, Err(PublicationStoreError::Io(error))
            if error.raw_os_error() == Some(libc::EFBIG)));
        assert_eq!(
            retained, 0,
            "failed writes must retain neither a record nor a stage"
        );
    }

    #[test]
    fn issuance_is_unique_and_readback_rejects_changed_bytes_without_a_database() {
        let root = std::env::temp_dir().join(format!(
            "cw-store-unit-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        assert!(matches!(
            FilePublicationStore::new(&root),
            Err(PublicationStoreError::InvalidRoot)
        ));
        assert!(!root.exists(), "opening an absent root must not create it");
        fs::write(&root, b"not a directory").unwrap();
        assert!(matches!(
            FilePublicationStore::new(&root),
            Err(PublicationStoreError::InvalidRoot)
        ));
        fs::remove_file(&root).unwrap();
        fs::create_dir(&root).unwrap();
        fs::set_permissions(&root, fs::Permissions::from_mode(0o777)).unwrap();
        assert!(matches!(
            FilePublicationStore::new(&root),
            Err(PublicationStoreError::InvalidRoot)
        ));
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
        let store = FilePublicationStore::new(&root).unwrap();
        let bytes = b"immutable model";
        let release = SemanticRelease::new(
            ReleaseMetadata::new("unit-release", "1.0.0", "unit-ontology").unwrap(),
            TruthStatus::Authoritative,
            PublicationState::Published,
            ReleaseDigest::new(&sha256(bytes)).unwrap(),
            vec![
                EvidenceReference::new("source-b", "digest", "location-b").unwrap(),
                EvidenceReference::new("source-a", "digest", "location-a").unwrap(),
            ],
            vec!["unit.concept.b".to_owned(), "unit.concept.a".to_owned()],
        )
        .unwrap();
        let pinned = SemanticReleaseClient::with_trusted_release_manifests(
            "1.0.0",
            vec![],
            vec![
                TrustedReleaseManifest::new(release.release_id(), release.manifest_digest())
                    .unwrap(),
            ],
        )
        .unwrap();
        let unpinned = SemanticReleaseClient::new("1.0.0").unwrap();
        assert!(matches!(
            store.read_verified(&unpinned, &release),
            Err(PublicationStoreError::ReleaseNotAdmitted)
        ));
        assert!(!store.release_path(release.release_id()).exists());
        assert!(store.read_verified(&pinned, &release).unwrap().is_none());
        assert!(matches!(
            store.issue_record(&release, b"changed model"),
            Err(PublicationStoreError::InvalidRecord)
        ));
        assert!(store.read_verified(&pinned, &release).unwrap().is_none());
        for truth in [
            TruthStatus::Observed,
            TruthStatus::Inferred,
            TruthStatus::Proposed,
            TruthStatus::Authoritative,
            TruthStatus::Superseded,
            TruthStatus::Rejected,
        ] {
            for state in [
                PublicationState::Draft,
                PublicationState::Proposed,
                PublicationState::Validated,
                PublicationState::Reviewed,
                PublicationState::Published,
                PublicationState::Superseded,
                PublicationState::Rejected,
            ] {
                if (truth, state) == (TruthStatus::Authoritative, PublicationState::Published) {
                    continue;
                }
                let unissued = SemanticRelease::new(
                    ReleaseMetadata::new("unit-release", "1.0.0", "unit-ontology").unwrap(),
                    truth,
                    state,
                    release.artifact_digest().clone(),
                    release.provenance().to_vec(),
                    release.concept_ids().to_vec(),
                )
                .unwrap();
                assert!(
                    matches!(
                        store.issue_record(&unissued, bytes),
                        Err(PublicationStoreError::InvalidRecord)
                    ),
                    "{truth:?}/{state:?}"
                );
                assert!(!store.release_path(release.release_id()).exists());
                assert_eq!(
                    fs::read_dir(&root).unwrap().count(),
                    0,
                    "denied issuance must not retain a stage"
                );
            }
        }
        store.issue_record(&release, bytes).unwrap();
        assert_eq!(
            store.read_verified(&pinned, &release).unwrap(),
            Some(bytes.to_vec())
        );
        let reordered = SemanticRelease::new(
            ReleaseMetadata::new("unit-release", "1.0.0", "unit-ontology").unwrap(),
            TruthStatus::Authoritative,
            PublicationState::Published,
            release.artifact_digest().clone(),
            release.provenance().iter().rev().cloned().collect(),
            release.concept_ids().iter().rev().cloned().collect(),
        )
        .unwrap();
        assert_eq!(release.manifest_digest(), reordered.manifest_digest());
        assert_eq!(
            store.read_verified(&pinned, &reordered).unwrap(),
            Some(bytes.to_vec())
        );
        assert!(matches!(
            store.issue_record(&release, bytes),
            Err(PublicationStoreError::DuplicateReleaseId)
        ));
        let mut corrupt = fs::read(store.release_path(release.release_id())).unwrap();
        *corrupt.last_mut().unwrap() ^= 1;
        fs::set_permissions(
            store.release_path(release.release_id()),
            fs::Permissions::from_mode(0o600),
        )
        .unwrap();
        fs::write(store.release_path(release.release_id()), corrupt).unwrap();
        assert!(matches!(
            store.read_verified(&pinned, &release),
            Err(PublicationStoreError::InvalidRecord)
        ));
        fs::remove_file(store.release_path(release.release_id())).unwrap();
        std::os::unix::fs::symlink("/dev/zero", store.release_path(release.release_id())).unwrap();
        assert!(matches!(
            store.read_verified(&pinned, &release),
            Err(PublicationStoreError::InvalidRecord)
        ));
        fs::remove_file(store.release_path(release.release_id())).unwrap();
        let record_path = store.release_path(release.release_id());
        assert!(
            std::process::Command::new("mkfifo")
                .arg(&record_path)
                .status()
                .unwrap()
                .success()
        );
        let (sender, receiver) = std::sync::mpsc::channel();
        let reader_store = store.clone();
        let reader_pin = pinned.clone();
        let reader_release = release.clone();
        let reader = std::thread::spawn(move || {
            sender
                .send(matches!(
                    reader_store.read_verified(&reader_pin, &reader_release),
                    Err(PublicationStoreError::InvalidRecord)
                ))
                .unwrap();
        });
        assert!(
            receiver
                .recv_timeout(std::time::Duration::from_secs(5))
                .unwrap()
        );
        reader.join().unwrap();
        fs::remove_file(&record_path).unwrap();
        fs::write(&record_path, [0; 7]).unwrap();
        assert!(matches!(
            store.read_verified(&pinned, &release),
            Err(PublicationStoreError::InvalidRecord)
        ));
        fs::write(&record_path, u64::MAX.to_be_bytes()).unwrap();
        assert!(matches!(
            store.read_verified(&pinned, &release),
            Err(PublicationStoreError::InvalidRecord)
        ));
        for declared_header_len in [0_u64, 1] {
            fs::write(&record_path, declared_header_len.to_be_bytes()).unwrap();
            assert!(matches!(
                store.read_verified(&pinned, &release),
                Err(PublicationStoreError::InvalidRecord)
            ));
        }
        OpenOptions::new()
            .write(true)
            .open(&record_path)
            .unwrap()
            .set_len(MAX_RECORD_BYTES as u64 + 1)
            .unwrap();
        assert!(matches!(
            store.read_verified(&pinned, &release),
            Err(PublicationStoreError::InvalidRecord)
        ));
        fs::remove_file(&record_path).unwrap();
        fs::create_dir(&record_path).unwrap();
        assert!(matches!(
            store.read_verified(&pinned, &release),
            Err(PublicationStoreError::InvalidRecord)
        ));
        let first_sequence = STAGE_SEQUENCE.load(Ordering::Relaxed);
        let occupied: Vec<_> = (first_sequence..first_sequence + 64)
            .map(|sequence| root.join(format!(".stage-{}-{sequence}", std::process::id())))
            .collect();
        for path in &occupied {
            fs::write(path, b"occupied stage").unwrap();
        }
        assert!(
            matches!(store.create_stage(), Err(PublicationStoreError::Io(error))
            if error.kind() == std::io::ErrorKind::AlreadyExists)
        );
        for path in &occupied {
            assert_eq!(fs::read(path).unwrap(), b"occupied stage");
        }
        let (fresh_path, fresh_file) = store.create_stage().unwrap();
        assert!(!occupied.contains(&fresh_path));
        drop(fresh_file);
        fs::remove_file(fresh_path).unwrap();
        fs::remove_dir_all(&root).unwrap();
        assert!(
            matches!(store.issue_record(&release, bytes), Err(PublicationStoreError::Io(error))
            if error.kind() == std::io::ErrorKind::NotFound)
        );
        assert!(!root.exists());
    }
}
