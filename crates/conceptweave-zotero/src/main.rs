#![forbid(unsafe_code)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use conceptweave_zotero::{
    ClassificationReport, GoldenSetApproval, StewardDecisionPatch, StewardReviewWorksheet,
    apply_steward_decision_patch, assess_steward_review_progress, build_steward_review_worksheet,
    read_local_snapshot, reviewed_golden_set_from_worksheet,
};
use serde::de::DeserializeOwned;
use std::collections::BTreeSet;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

const USAGE: &str = "usage: conceptweave-zotero /tmp/REPORT.json | --worksheet /tmp/REPORT.json /tmp/WORKSHEET.json | --review-progress /tmp/REPORT.json /tmp/WORKSHEET.json /tmp/PROGRESS.json | --apply-decision-patch /tmp/REPORT.json /tmp/CURRENT_WORKSHEET.json /tmp/PATCH.json /tmp/UPDATED_WORKSHEET.json | --finalize /tmp/REPORT.json /tmp/WORKSHEET.json /tmp/APPROVAL.json /tmp/GOLDEN.json";
const MAX_ARTIFACT_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug, PartialEq, Eq)]
enum OutputRequest {
    Report(String),
    Worksheet {
        report: String,
        worksheet: String,
    },
    ReviewProgress {
        report: String,
        worksheet: String,
        output: String,
    },
    ApplyDecisionPatch {
        report: String,
        worksheet: String,
        patch: String,
        output: String,
    },
    Finalize {
        report: String,
        worksheet: String,
        approval: String,
        output: String,
    },
}

/// Parses one mutually exclusive report, worksheet, review-progress, or finalization request.
fn parse_output_request<I, S>(args: I) -> Result<OutputRequest, &'static str>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut args = args.into_iter().map(Into::into);
    let first = args.next().ok_or(USAGE)?;
    let request = if first == "--worksheet" {
        let report = args
            .next()
            .ok_or("--worksheet requires report and worksheet output paths")?;
        let worksheet = args
            .next()
            .ok_or("--worksheet requires report and worksheet output paths")?;
        if report == worksheet {
            return Err("report and worksheet output paths must differ");
        }
        OutputRequest::Worksheet { report, worksheet }
    } else if first == "--review-progress" {
        let report = args
            .next()
            .ok_or("--review-progress requires three artifact paths")?;
        let worksheet = args
            .next()
            .ok_or("--review-progress requires three artifact paths")?;
        let output = args
            .next()
            .ok_or("--review-progress requires three artifact paths")?;
        if BTreeSet::from([report.as_str(), worksheet.as_str(), output.as_str()]).len() != 3 {
            return Err("review progress artifact paths must differ");
        }
        OutputRequest::ReviewProgress {
            report,
            worksheet,
            output,
        }
    } else if first == "--apply-decision-patch" {
        let report = args
            .next()
            .ok_or("--apply-decision-patch requires four artifact paths")?;
        let worksheet = args
            .next()
            .ok_or("--apply-decision-patch requires four artifact paths")?;
        let patch = args
            .next()
            .ok_or("--apply-decision-patch requires four artifact paths")?;
        let output = args
            .next()
            .ok_or("--apply-decision-patch requires four artifact paths")?;
        if BTreeSet::from([
            report.as_str(),
            worksheet.as_str(),
            patch.as_str(),
            output.as_str(),
        ])
        .len()
            != 4
        {
            return Err("decision patch artifact paths must differ");
        }
        OutputRequest::ApplyDecisionPatch {
            report,
            worksheet,
            patch,
            output,
        }
    } else if first == "--finalize" {
        let report = args
            .next()
            .ok_or("--finalize requires four artifact paths")?;
        let worksheet = args
            .next()
            .ok_or("--finalize requires four artifact paths")?;
        let approval = args
            .next()
            .ok_or("--finalize requires four artifact paths")?;
        let output = args
            .next()
            .ok_or("--finalize requires four artifact paths")?;
        if BTreeSet::from([
            report.as_str(),
            worksheet.as_str(),
            approval.as_str(),
            output.as_str(),
        ])
        .len()
            != 4
        {
            return Err("finalization artifact paths must differ");
        }
        OutputRequest::Finalize {
            report,
            worksheet,
            approval,
            output,
        }
    } else {
        OutputRequest::Report(first)
    };
    if args.next().is_some() {
        return Err("unexpected extra argument");
    }
    Ok(request)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ArtifactIdentity {
    device: u64,
    inode: u64,
}

/// Opens, validates, bounds, and deserializes one owner-only review artifact.
fn read_private_json<T: DeserializeOwned>(raw: &str) -> io::Result<(T, ArtifactIdentity)> {
    let path = PathBuf::from(raw);
    if !path.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "review input must be an absolute path in the system temp directory",
        ));
    }
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "review input has no parent"))?;
    if !allowed_output_parents().contains(&parent.canonicalize()?) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "review input must be a direct child of the system temp directory",
        ));
    }
    let path_metadata = fs::symlink_metadata(&path)?;
    if path_metadata.file_type().is_symlink() || !path_metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "review input must be a regular file",
        ));
    }
    let (file, opened_metadata) = open_with_metadata(&path)?;
    #[cfg(not(unix))]
    {
        let _ = (path_metadata, opened_metadata, file);
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "private review input requires a Unix platform",
        ));
    }
    #[cfg(unix)]
    let identity = validate_opened_identity(&path_metadata, &opened_metadata)?;
    #[cfg(unix)]
    {
        let parsed = read_bounded_json(&mut { file }, opened_metadata.len())?;
        Ok((parsed, identity))
    }
}

/// Opens a review input once and returns metadata from the opened handle.
fn open_with_metadata(path: &Path) -> io::Result<(File, fs::Metadata)> {
    #[cfg(unix)]
    use std::os::unix::fs::OpenOptionsExt;

    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    options.custom_flags(libc::O_NOFOLLOW);
    let file = options.open(path)?;
    file.metadata().map(|metadata| (file, metadata))
}

#[cfg(unix)]
/// Proves the opened Unix file matches the checked path and owner-only contract.
fn validate_opened_identity(
    path_metadata: &fs::Metadata,
    opened_metadata: &fs::Metadata,
) -> io::Result<ArtifactIdentity> {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    if (opened_metadata.dev(), opened_metadata.ino()) != (path_metadata.dev(), path_metadata.ino())
        || opened_metadata.nlink() != 1
        || opened_metadata.permissions().mode() & 0o777 != 0o600
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "review input identity or permissions are unsafe",
        ));
    }
    Ok(ArtifactIdentity {
        device: opened_metadata.dev(),
        inode: opened_metadata.ino(),
    })
}

/// Reads JSON without allowing the input to exceed or grow past the artifact limit.
fn read_bounded_json<T: DeserializeOwned>(
    reader: &mut dyn Read,
    advertised_len: u64,
) -> io::Result<T> {
    if advertised_len > MAX_ARTIFACT_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "review input exceeds the artifact size limit",
        ));
    }
    let mut content = Vec::with_capacity(advertised_len as usize);
    reader
        .take(MAX_ARTIFACT_BYTES + 1)
        .read_to_end(&mut content)?;
    if content.len() as u64 > MAX_ARTIFACT_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "review input grew beyond the artifact size limit",
        ));
    }
    serde_json::from_slice(&content)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

/// Preserves an input error kind while naming the rejected artifact.
fn label_input(name: &str, error: io::Error) -> io::Error {
    io::Error::new(error.kind(), format!("{name}: {error}"))
}

/// Writes one create-new owner-only artifact and removes a failed partial write.
fn write_private_output(path: &Path, content: &[u8]) -> io::Result<()> {
    write_private_output_with(path, content, write_all_and_flush)
}

#[cfg_attr(coverage_nightly, coverage(off))]
/// Writes and flushes the complete serialized artifact.
fn write_all_and_flush(writer: &mut BufWriter<File>, content: &[u8]) -> io::Result<()> {
    writer.write_all(content)?;
    writer.flush()
}

/// Runs the private-output boundary with an injectable writer for failure testing.
fn write_private_output_with(
    path: &Path,
    content: &[u8],
    write: fn(&mut BufWriter<File>, &[u8]) -> io::Result<()>,
) -> io::Result<()> {
    let file = create_report_file(path)?;
    let mut writer = BufWriter::new(file);
    if let Err(error) = write(&mut writer, content) {
        drop(writer);
        let _ = fs::remove_file(path);
        return Err(error);
    }
    Ok(())
}

#[cfg_attr(coverage_nightly, coverage(off))]
/// Returns canonical directories in which a sensitive report may be created.
fn allowed_output_parents() -> Vec<PathBuf> {
    let mut parents = vec![
        env::temp_dir()
            .canonicalize()
            .expect("system temporary directory must exist"),
    ];
    #[cfg(unix)]
    parents.push(Path::new("/tmp").canonicalize().expect("/tmp must exist"));
    parents
}

/// Validates that a report path is a new direct child of an allowed temp directory.
fn validate_output_path(raw: &str) -> io::Result<PathBuf> {
    let path = PathBuf::from(raw);
    if !path.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "report output must be an absolute path in the system temp directory",
        ));
    }

    let allowed_parents = allowed_output_parents();
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "report output has no parent")
    })?;
    let resolved_parent = parent.canonicalize()?;
    if !allowed_parents.contains(&resolved_parent) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "report output must be a direct child of the system temp directory",
        ));
    }
    if fs::symlink_metadata(&path).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "report output must not already exist or be a symlink",
        ));
    }
    Ok(path)
}

/// Creates a new sensitive report file or fails closed on unsupported platforms.
fn create_report_file(path: &Path) -> io::Result<File> {
    #[cfg(not(unix))]
    return Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "private report creation requires a Unix platform",
    ));

    #[cfg(unix)]
    {
        create_report_file_with(path, set_owner_only_permissions)
    }
}

#[cfg(unix)]
/// Restores exact owner-only permissions after process umask application.
fn set_owner_only_permissions(file: &File) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    file.set_permissions(fs::Permissions::from_mode(0o600))
}

#[cfg(unix)]
/// Creates a private file and removes it if final permission enforcement fails.
fn create_report_file_with(
    path: &Path,
    set_permissions: fn(&File) -> io::Result<()>,
) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;

    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    let file = options.mode(0o600).open(path)?;
    if let Err(error) = set_permissions(&file) {
        drop(file);
        let _ = fs::remove_file(path);
        return Err(error);
    }
    Ok(file)
}

#[cfg_attr(coverage_nightly, coverage(off))]
/// Reads one Zotero snapshot and writes its sensitive local proposal report.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    match parse_output_request(env::args().skip(1))? {
        OutputRequest::Report(output) => {
            let output = validate_output_path(&output)?;
            let report = read_local_snapshot()?;
            if report.zotero_version.starts_with("9.") {
                eprintln!("Zotero 9 Local API is read-only; writing local proposal output only");
            }
            write_private_output(&output, &serde_json::to_vec_pretty(&report)?)?;
        }
        OutputRequest::Worksheet { report, worksheet } => {
            let report_output = validate_output_path(&report)?;
            let worksheet_output = validate_output_path(&worksheet)?;
            let report = read_local_snapshot()?;
            if report.zotero_version.starts_with("9.") {
                eprintln!("Zotero 9 Local API is read-only; writing local proposal output only");
            }
            let worksheet = build_steward_review_worksheet(&report)?;
            write_private_output(&report_output, &serde_json::to_vec_pretty(&report)?)?;
            if let Err(error) =
                write_private_output(&worksheet_output, &serde_json::to_vec_pretty(&worksheet)?)
            {
                let _ = fs::remove_file(report_output);
                return Err(error.into());
            }
        }
        OutputRequest::ReviewProgress {
            report,
            worksheet,
            output,
        } => {
            let output = validate_output_path(&output)?;
            let (report, report_identity): (ClassificationReport, _) =
                read_private_json(&report).map_err(|error| label_input("report", error))?;
            let (worksheet, worksheet_identity): (StewardReviewWorksheet, _) =
                read_private_json(&worksheet).map_err(|error| label_input("worksheet", error))?;
            if report_identity == worksheet_identity {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "review progress inputs must be distinct files",
                )
                .into());
            }
            let progress = assess_steward_review_progress(&report, &worksheet)?;
            write_private_output(&output, &serde_json::to_vec_pretty(&progress)?)?;
        }
        OutputRequest::ApplyDecisionPatch {
            report,
            worksheet,
            patch,
            output,
        } => {
            let output = validate_output_path(&output)?;
            let (report, report_identity): (ClassificationReport, _) =
                read_private_json(&report).map_err(|error| label_input("report", error))?;
            let (worksheet, worksheet_identity): (StewardReviewWorksheet, _) =
                read_private_json(&worksheet).map_err(|error| label_input("worksheet", error))?;
            let (patch, patch_identity): (StewardDecisionPatch, _) =
                read_private_json(&patch).map_err(|error| label_input("decision patch", error))?;
            if BTreeSet::from([report_identity, worksheet_identity, patch_identity]).len() != 3 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "decision patch inputs must be distinct files",
                )
                .into());
            }
            let updated = apply_steward_decision_patch(&report, &worksheet, &patch)?;
            let content = serde_json::to_vec_pretty(&updated)?;
            write_private_output(&output, &content)?;
        }
        OutputRequest::Finalize {
            report,
            worksheet,
            approval,
            output,
        } => {
            let output = validate_output_path(&output)?;
            let (report, report_identity): (ClassificationReport, _) =
                read_private_json(&report).map_err(|error| label_input("report", error))?;
            let (worksheet, worksheet_identity): (StewardReviewWorksheet, _) =
                read_private_json(&worksheet).map_err(|error| label_input("worksheet", error))?;
            let (approval, approval_identity): (GoldenSetApproval, _) =
                read_private_json(&approval).map_err(|error| label_input("approval", error))?;
            if BTreeSet::from([report_identity, worksheet_identity, approval_identity]).len() != 3 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "finalization inputs must be distinct files",
                )
                .into());
            }
            let golden = reviewed_golden_set_from_worksheet(&report, &worksheet, approval)?;
            write_private_output(&output, &serde_json::to_vec_pretty(&golden)?)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worksheet_mode_is_explicit_and_rejects_ambiguous_arguments() {
        let report = "/tmp/conceptweave-zotero-report.json";
        let worksheet = "/tmp/conceptweave-zotero-worksheet.json";
        assert_eq!(
            parse_output_request(vec!["--worksheet", report, worksheet]),
            Ok(OutputRequest::Worksheet {
                report: report.to_owned(),
                worksheet: worksheet.to_owned(),
            })
        );
        assert_eq!(
            parse_output_request(vec![report]),
            Ok(OutputRequest::Report(report.to_owned()))
        );
        assert_eq!(parse_output_request(Vec::<&str>::new()), Err(USAGE));
        assert!(parse_output_request(vec!["--worksheet"]).is_err());
        assert!(parse_output_request(vec!["--worksheet", report]).is_err());
        assert!(parse_output_request(vec![report, "extra"]).is_err());
        assert!(parse_output_request(vec!["--worksheet", report, report]).is_err());
    }

    #[test]
    fn finalization_mode_requires_four_distinct_artifact_paths() {
        let report = "/tmp/report.json";
        let worksheet = "/tmp/worksheet.json";
        let approval = "/tmp/approval.json";
        let output = "/tmp/golden.json";
        assert_eq!(
            parse_output_request(vec!["--finalize", report, worksheet, approval, output,]),
            Ok(OutputRequest::Finalize {
                report: report.to_owned(),
                worksheet: worksheet.to_owned(),
                approval: approval.to_owned(),
                output: output.to_owned(),
            })
        );
        assert!(parse_output_request(vec!["--finalize"]).is_err());
        assert!(parse_output_request(vec!["--finalize", report]).is_err());
        assert!(parse_output_request(vec!["--finalize", report, worksheet]).is_err());
        assert!(parse_output_request(vec!["--finalize", report, worksheet, approval]).is_err());
        assert!(
            parse_output_request(vec!["--finalize", report, worksheet, approval, report]).is_err()
        );
        assert!(
            parse_output_request(vec![
                "--finalize",
                report,
                worksheet,
                approval,
                output,
                "extra",
            ])
            .is_err()
        );
    }

    #[test]
    fn review_progress_mode_requires_three_distinct_artifact_paths() {
        let report = "/tmp/report.json";
        let worksheet = "/tmp/worksheet.json";
        let output = "/tmp/progress.json";
        assert_eq!(
            parse_output_request(vec!["--review-progress", report, worksheet, output]),
            Ok(OutputRequest::ReviewProgress {
                report: report.to_owned(),
                worksheet: worksheet.to_owned(),
                output: output.to_owned(),
            })
        );
        assert!(parse_output_request(vec!["--review-progress"]).is_err());
        assert!(parse_output_request(vec!["--review-progress", report]).is_err());
        assert!(parse_output_request(vec!["--review-progress", report, worksheet]).is_err());
        assert!(
            parse_output_request(vec!["--review-progress", report, worksheet, report]).is_err()
        );
        assert!(
            parse_output_request(vec![
                "--review-progress",
                report,
                worksheet,
                output,
                "extra"
            ])
            .is_err()
        );
    }

    #[test]
    fn apply_decision_patch_mode_requires_four_distinct_artifact_paths() {
        let report = "/tmp/report.json";
        let worksheet = "/tmp/worksheet.json";
        let patch = "/tmp/patch.json";
        let output = "/tmp/updated-worksheet.json";
        assert_eq!(
            parse_output_request(vec!["--apply-decision-patch", report, worksheet, patch, output]),
            Ok(OutputRequest::ApplyDecisionPatch {
                report: report.to_owned(),
                worksheet: worksheet.to_owned(),
                patch: patch.to_owned(),
                output: output.to_owned(),
            })
        );
        assert!(parse_output_request(vec!["--apply-decision-patch"]).is_err());
        assert!(parse_output_request(vec!["--apply-decision-patch", report]).is_err());
        assert!(parse_output_request(vec!["--apply-decision-patch", report, worksheet]).is_err());
        assert!(
            parse_output_request(vec!["--apply-decision-patch", report, worksheet, patch]).is_err()
        );
        assert!(
            parse_output_request(vec!["--apply-decision-patch", report, worksheet, patch, report])
                .is_err()
        );
        assert!(
            parse_output_request(vec!["--apply-decision-patch", report, worksheet, worksheet, output])
                .is_err()
        );
        assert!(
            parse_output_request(vec!["--apply-decision-patch", report, report, patch, output])
                .is_err()
        );
        assert!(
            parse_output_request(vec![
                "--apply-decision-patch",
                report,
                worksheet,
                patch,
                output,
                "extra",
            ])
            .is_err()
        );
    }

    #[cfg(unix)]
    #[test]
    fn private_json_input_is_owner_only_regular_bounded_and_valid() {
        use std::os::unix::fs::{PermissionsExt, symlink};

        let valid = unique_temp_path("valid-input");
        let _ = fs::remove_file(&valid);
        fs::write(&valid, br#"{"accepted":true}"#).unwrap();
        fs::set_permissions(&valid, fs::Permissions::from_mode(0o600)).unwrap();

        fs::set_permissions(&valid, fs::Permissions::from_mode(0o000)).unwrap();
        assert!(read_private_json::<serde_json::Value>(valid.to_str().unwrap()).is_err());
        fs::set_permissions(&valid, fs::Permissions::from_mode(0o600)).unwrap();
        let (parsed, _): (serde_json::Value, _) =
            read_private_json(valid.to_str().unwrap()).unwrap();
        assert_eq!(parsed["accepted"], true);
        assert!(read_private_json::<serde_json::Value>("relative.json").is_err());
        assert!(read_private_json::<serde_json::Value>("/").is_err());
        assert!(
            read_private_json::<serde_json::Value>(
                unique_temp_path("missing-input").to_str().unwrap()
            )
            .is_err()
        );
        assert!(
            read_private_json::<serde_json::Value>(
                "/tmp/conceptweave-zotero-missing-directory/input.json"
            )
            .is_err()
        );
        assert!(
            read_private_json::<serde_json::Value>(
                env::current_dir()
                    .unwrap()
                    .join("input.json")
                    .to_str()
                    .unwrap()
            )
            .is_err()
        );

        let directory = env::temp_dir().join(format!(
            "conceptweave-zotero-{}-input-directory",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir(&directory).unwrap();
        assert!(read_private_json::<serde_json::Value>(directory.to_str().unwrap()).is_err());
        fs::remove_dir(directory).unwrap();

        let link = unique_temp_path("input-link");
        let _ = fs::remove_file(&link);
        symlink(&valid, &link).unwrap();
        assert!(read_private_json::<serde_json::Value>(link.to_str().unwrap()).is_err());
        fs::remove_file(link).unwrap();

        fs::set_permissions(&valid, fs::Permissions::from_mode(0o644)).unwrap();
        assert!(read_private_json::<serde_json::Value>(valid.to_str().unwrap()).is_err());
        fs::set_permissions(&valid, fs::Permissions::from_mode(0o600)).unwrap();

        let hardlink = unique_temp_path("input-hardlink");
        let _ = fs::remove_file(&hardlink);
        fs::hard_link(&valid, &hardlink).unwrap();
        assert!(read_private_json::<serde_json::Value>(valid.to_str().unwrap()).is_err());
        fs::remove_file(hardlink).unwrap();

        fs::write(&valid, b"not-json").unwrap();
        assert!(read_private_json::<serde_json::Value>(valid.to_str().unwrap()).is_err());
        fs::remove_file(valid).unwrap();

        let oversized = unique_temp_path("oversized-input");
        let _ = fs::remove_file(&oversized);
        let file = File::create(&oversized).unwrap();
        file.set_len(MAX_ARTIFACT_BYTES + 1).unwrap();
        fs::set_permissions(&oversized, fs::Permissions::from_mode(0o600)).unwrap();
        assert!(read_private_json::<serde_json::Value>(oversized.to_str().unwrap()).is_err());
        fs::remove_file(oversized).unwrap();

        let identity_left = unique_temp_path("identity-left");
        let identity_right = unique_temp_path("identity-right");
        fs::write(&identity_left, b"{}").unwrap();
        fs::write(&identity_right, b"{}").unwrap();
        fs::set_permissions(&identity_left, fs::Permissions::from_mode(0o600)).unwrap();
        fs::set_permissions(&identity_right, fs::Permissions::from_mode(0o600)).unwrap();
        assert!(
            validate_opened_identity(
                &fs::metadata(&identity_left).unwrap(),
                &fs::metadata(&identity_right).unwrap()
            )
            .is_err()
        );
        fs::remove_file(identity_left).unwrap();
        fs::remove_file(identity_right).unwrap();
    }

    #[test]
    fn bounded_json_reader_and_input_labels_cover_failures() {
        struct FailingReader;
        impl Read for FailingReader {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::other("injected read failure"))
            }
        }

        let oversized =
            read_bounded_json::<serde_json::Value>(&mut io::empty(), MAX_ARTIFACT_BYTES + 1)
                .unwrap_err();
        assert_eq!(oversized.kind(), io::ErrorKind::InvalidData);

        let grown = read_bounded_json::<serde_json::Value>(
            &mut io::repeat(b' ').take(MAX_ARTIFACT_BYTES + 1),
            0,
        )
        .unwrap_err();
        assert_eq!(grown.kind(), io::ErrorKind::InvalidData);

        let read_failure =
            read_bounded_json::<serde_json::Value>(&mut FailingReader, 0).unwrap_err();
        assert_eq!(read_failure.kind(), io::ErrorKind::Other);

        let labeled = label_input(
            "worksheet",
            io::Error::new(io::ErrorKind::PermissionDenied, "unsafe"),
        );
        assert_eq!(labeled.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(labeled.to_string(), "worksheet: unsafe");
    }

    #[test]
    fn failed_private_output_is_removed_for_retry() {
        let output = unique_temp_path("failed-output");
        let _ = fs::remove_file(&output);
        let error = write_private_output_with(&output, b"content", |_, _| {
            Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "injected write failure",
            ))
        })
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::WriteZero);
        assert!(!output.exists());

        write_private_output(&output, b"complete").unwrap();
        assert_eq!(fs::read(&output).unwrap(), b"complete");
        assert!(write_private_output(&output, b"replacement").is_err());
        fs::remove_file(output).unwrap();
    }

    fn unique_temp_path(suffix: &str) -> PathBuf {
        env::temp_dir().join(format!(
            "conceptweave-zotero-{}-{suffix}.json",
            std::process::id()
        ))
    }

    #[test]
    fn output_path_must_be_a_new_direct_temp_child() {
        let allowed = unique_temp_path("allowed");
        let _ = fs::remove_file(&allowed);
        assert_eq!(
            validate_output_path(allowed.to_str().unwrap()).unwrap(),
            allowed
        );

        assert!(validate_output_path("relative.json").is_err());
        assert!(validate_output_path("/").is_err());
        assert!(validate_output_path("/tmp/missing-directory/report.json").is_err());
        assert!(
            validate_output_path(
                env::current_dir()
                    .unwrap()
                    .join("report.json")
                    .to_str()
                    .unwrap()
            )
            .is_err()
        );

        #[cfg(unix)]
        {
            let conventional = Path::new("/tmp").join(format!(
                "conceptweave-zotero-{}-conventional.json",
                std::process::id()
            ));
            let _ = fs::remove_file(&conventional);
            assert!(validate_output_path(conventional.to_str().unwrap()).is_ok());
        }

        #[cfg(not(unix))]
        assert!(validate_output_path("/tmp/conceptweave-zotero.json").is_err());

        let nested_dir =
            env::temp_dir().join(format!("conceptweave-zotero-{}-nested", std::process::id()));
        fs::create_dir_all(&nested_dir).unwrap();
        assert!(validate_output_path(nested_dir.join("report.json").to_str().unwrap()).is_err());
        fs::remove_dir_all(nested_dir).unwrap();

        let existing = unique_temp_path("existing");
        fs::write(&existing, b"existing").unwrap();
        assert!(validate_output_path(existing.to_str().unwrap()).is_err());
        fs::remove_file(existing).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn output_path_rejects_symlinks_before_open() {
        use std::os::unix::fs::symlink;

        let target = unique_temp_path("target");
        let link = unique_temp_path("link");
        let _ = fs::remove_file(&target);
        let _ = fs::remove_file(&link);
        fs::write(&target, b"target").unwrap();
        symlink(&target, &link).unwrap();
        assert!(validate_output_path(link.to_str().unwrap()).is_err());
        fs::remove_file(link).unwrap();
        fs::remove_file(target).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn report_output_is_owner_readable_and_writable_only() {
        use std::os::unix::fs::PermissionsExt;

        let output = unique_temp_path("private");
        let _ = fs::remove_file(&output);
        let file = create_report_file(&output).unwrap();
        let mode = file.metadata().unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        drop(file);
        fs::remove_file(output).unwrap();

        let existing = unique_temp_path("private-existing");
        let _ = fs::remove_file(&existing);
        fs::write(&existing, b"existing").unwrap();
        assert!(create_report_file(&existing).is_err());
        fs::remove_file(existing).unwrap();

        let rejected = unique_temp_path("private-permission-error");
        let _ = fs::remove_file(&rejected);
        let error = create_report_file_with(&rejected, |_| {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "injected permission failure",
            ))
        })
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        assert!(!rejected.exists());
    }
}
