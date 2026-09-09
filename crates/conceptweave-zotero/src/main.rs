#![forbid(unsafe_code)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use conceptweave_zotero::{ClassificationReport, ReadError, read_local_snapshot};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

fn allowed_output_parent_policy(
    system_temp: PathBuf,
    conventional_tmp: Option<PathBuf>,
) -> Vec<PathBuf> {
    let mut parents = vec![system_temp];
    if let Some(conventional_tmp) = conventional_tmp
        && !parents.contains(&conventional_tmp)
    {
        parents.push(conventional_tmp);
    }
    parents
}

fn conventional_tmp_parent(path: &Path) -> Option<PathBuf> {
    path.canonicalize().ok()
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn allowed_output_parents() -> Vec<PathBuf> {
    let system_temp = env::temp_dir()
        .canonicalize()
        .expect("system temporary directory must exist");
    // Host path discovery stays in this shim; admission/dedup policy is deterministic above.
    allowed_output_parent_policy(system_temp, conventional_tmp_parent(Path::new("/tmp")))
}

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
    let file_name = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "report output has no file name",
        )
    })?;
    let resolved_path = resolved_parent.join(file_name);
    if fs::symlink_metadata(&resolved_path).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "report output must not already exist or be a symlink",
        ));
    }
    Ok(resolved_path)
}

fn open_new_output(path: &Path) -> io::Result<fs::File> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path)
}

fn temporary_output_path(output: &Path) -> io::Result<PathBuf> {
    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    let file_name = output.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "report output has no file name",
        )
    })?;
    let nonce = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    Ok(output.with_file_name(format!(
        ".{}.{}.{}.tmp",
        file_name.to_string_lossy(),
        std::process::id(),
        nonce
    )))
}

fn write_report(
    output: &Path,
    report: &ClassificationReport,
) -> Result<(), Box<dyn std::error::Error>> {
    write_report_with(output, &mut open_new_output, &mut |writer| {
        serialize_report(writer, report)
    })
}

fn write_report_with(
    output: &Path,
    open_output: &mut dyn FnMut(&Path) -> io::Result<fs::File>,
    serialize: &mut dyn FnMut(&mut BufWriter<fs::File>) -> Result<(), Box<dyn std::error::Error>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let temporary = temporary_output_path(output)?;
    let file = open_output(&temporary)?;
    let mut writer = BufWriter::new(file);

    if let Err(error) = serialize(&mut writer) {
        drop(writer);
        let _ = fs::remove_file(&temporary);
        return Err(error);
    }
    drop(writer);

    if let Err(error) = fs::hard_link(&temporary, output) {
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    cleanup_published_report(&temporary).map_err(Into::into)
}

fn serialize_report(
    writer: &mut dyn Write,
    report: &ClassificationReport,
) -> Result<(), Box<dyn std::error::Error>> {
    serde_json::to_writer_pretty(&mut *writer, report)?;
    writer.flush()?;
    Ok(())
}

fn cleanup_published_report(temporary: &Path) -> io::Result<()> {
    fs::remove_file(temporary).map_err(|error| {
        let kind = error.kind();
        io::Error::new(
            kind,
            format!("report published but temporary cleanup failed: {error}"),
        )
    })
}

fn run_with(
    args: Vec<String>,
    read_snapshot: &mut dyn FnMut() -> Result<ClassificationReport, ReadError>,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = args
        .into_iter()
        .nth(1)
        .ok_or("usage: conceptweave-zotero /tmp/OUTPUT.json")?;
    let output = validate_output_path(&output)?;
    let report = read_snapshot()?;
    if report.zotero_version.starts_with("9.") {
        eprintln!("Zotero 9 Local API is read-only; writing a local proposal report only");
    }
    write_report(&output, &report)
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_with(env::args().collect(), &mut read_local_snapshot)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct WriteFailWriter;

    impl Write for WriteFailWriter {
        fn write(&mut self, _bytes: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("intentional write failure"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct FlushFailWriter;

    impl Write for FlushFailWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::other("intentional flush failure"))
        }
    }

    fn unique_temp_path(suffix: &str) -> PathBuf {
        env::temp_dir().join(format!(
            "conceptweave-zotero-{}-{suffix}.json",
            std::process::id()
        ))
    }

    #[test]
    fn output_path_helpers_keep_parent_policy_and_temp_names_bounded() {
        let system_temp = env::temp_dir();
        assert_eq!(
            allowed_output_parent_policy(system_temp.clone(), Some(system_temp.clone())),
            vec![system_temp.clone()]
        );
        let alternate = system_temp.join("conceptweave-zotero-alternate-parent");
        assert_eq!(
            allowed_output_parent_policy(system_temp.clone(), Some(alternate.clone())),
            vec![system_temp.clone(), alternate]
        );
        assert!(conventional_tmp_parent(Path::new("/definitely-missing-parent")).is_none());
        assert!(conventional_tmp_parent(&system_temp).is_some());
        assert!(temporary_output_path(Path::new("relative")).is_ok());
        assert!(temporary_output_path(Path::new("/")).is_err());
        let output = unique_temp_path("writer-seam");
        let _ = fs::remove_file(&output);
        let mut serialize = |_: &mut BufWriter<fs::File>| Ok(());
        write_report_with(&output, &mut open_new_output, &mut serialize).unwrap();
        fs::remove_file(&output).unwrap();
        assert!(write_report_with(Path::new("/"), &mut open_new_output, &mut serialize).is_err());
        let output = unique_temp_path("open-failure");
        assert!(
            write_report_with(
                &output,
                &mut |_| Err(io::Error::other("intentional open failure")),
                &mut serialize,
            )
            .is_err()
        );
    }

    fn sample_report(zotero_version: &str) -> ClassificationReport {
        ClassificationReport {
            zotero_version: zotero_version.to_owned(),
            api_version: Some(3),
            schema_version: Some(44),
            server_id: Some("test-server".to_owned()),
            library_version: 2,
            rule_revision: conceptweave_zotero::RULE_REVISION,
            observed_item_count: 0,
            classified_items: Vec::new(),
            unclassified_items: Vec::new(),
            pending_source_item_keys: Vec::new(),
            duplicate_candidates: Vec::new(),
        }
    }

    fn failing_snapshot() -> Result<ClassificationReport, ReadError> {
        Err(ReadError::Budget("test-reader"))
    }

    #[test]
    fn production_runner_publishes_a_complete_report() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let output = unique_temp_path(&format!("runner-success-{nonce}"));
        let _ = fs::remove_file(&output);
        let args = vec![
            "conceptweave-zotero".to_owned(),
            output.to_string_lossy().into_owned(),
        ];

        run_with(args, &mut || Ok(sample_report("10.0.1"))).unwrap();
        let saved: serde_json::Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        assert_eq!(saved["zotero_version"], "10.0.1");
        fs::remove_file(output).unwrap();
    }

    #[test]
    fn production_runner_preserves_the_zotero_9_read_only_path() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let output = unique_temp_path(&format!("runner-zotero9-{nonce}"));
        let _ = fs::remove_file(&output);
        let args = vec![
            "conceptweave-zotero".to_owned(),
            output.to_string_lossy().into_owned(),
        ];

        run_with(args, &mut || Ok(sample_report("9.0.6"))).unwrap();
        let saved: serde_json::Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        assert_eq!(saved["zotero_version"], "9.0.6");
        fs::remove_file(output).unwrap();
    }

    #[test]
    fn production_runner_rejects_missing_output_before_reading() {
        let result = run_with(
            vec!["conceptweave-zotero".to_owned()],
            &mut failing_snapshot,
        );

        assert!(result.unwrap_err().to_string().contains("usage:"));
    }

    #[test]
    fn production_runner_rejects_invalid_output_before_reading() {
        let result = run_with(
            vec!["conceptweave-zotero".to_owned(), "relative.json".to_owned()],
            &mut failing_snapshot,
        );

        assert!(result.unwrap_err().to_string().contains("absolute path"));
    }

    #[test]
    fn production_runner_propagates_snapshot_failure_without_publishing() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let output = unique_temp_path(&format!("runner-read-failure-{nonce}"));
        let _ = fs::remove_file(&output);
        let args = vec![
            "conceptweave-zotero".to_owned(),
            output.to_string_lossy().into_owned(),
        ];

        let result = run_with(args, &mut failing_snapshot);
        assert!(result.is_err());
        assert!(!output.exists());
    }

    #[test]
    fn production_runner_does_not_overwrite_a_path_created_during_the_read() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let output = unique_temp_path(&format!("runner-publication-race-{nonce}"));
        let _ = fs::remove_file(&output);
        let args = vec![
            "conceptweave-zotero".to_owned(),
            output.to_string_lossy().into_owned(),
        ];
        let output_during_read = output.clone();

        let result = run_with(args, &mut || {
            fs::write(&output_during_read, b"competitor").unwrap();
            Ok(sample_report("10.0.1"))
        });

        assert!(result.is_err());
        assert_eq!(fs::read(&output).unwrap(), b"competitor");
        fs::remove_file(output).unwrap();
    }

    #[test]
    fn failed_serialization_never_exposes_the_final_report_path() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let output = unique_temp_path(&format!("serialization-failure-{nonce}"));
        let _ = fs::remove_file(&output);

        assert!(
            write_report_with(&output, &mut open_new_output, &mut |_| Err(
                io::Error::other("intentional serialization failure").into()
            ),)
            .is_err()
        );
        assert!(!output.exists());
    }

    #[test]
    fn report_serialization_propagates_write_and_flush_failures() {
        let report = sample_report("10.0.1");
        let mut write_failure = WriteFailWriter;
        assert!(serialize_report(&mut write_failure, &report).is_err());
        assert!(write_failure.flush().is_ok());

        let mut writer = FlushFailWriter;
        assert!(serialize_report(&mut writer, &report).is_err());
    }

    #[test]
    fn report_write_removes_temporary_file_when_hard_link_fails() {
        let output_name = format!("conceptweave-zotero-{}-directory", std::process::id());
        let output = env::temp_dir().join(&output_name);
        let _ = fs::remove_dir(&output);
        fs::create_dir(&output).unwrap();
        assert!(write_report(&output, &sample_report("10.0.1")).is_err());
        assert!(output.is_dir());
        let temporary_prefix = format!(".{output_name}.{}.", std::process::id());
        assert!(
            !fs::read_dir(env::temp_dir())
                .unwrap()
                .flatten()
                .any(|entry| entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with(&temporary_prefix))
        );
        fs::remove_dir(output).unwrap();
    }

    #[test]
    fn published_report_cleanup_reports_the_temp_failure_without_path_rollback() {
        let output = unique_temp_path("published-cleanup-failure");
        let temporary = unique_temp_path("missing-cleanup-source");
        let _ = fs::remove_file(&output);
        let _ = fs::remove_file(&temporary);
        fs::write(&output, b"published").unwrap();
        let result = cleanup_published_report(&temporary);

        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("report published but temporary cleanup failed")
        );
        assert_eq!(fs::read(&output).unwrap(), b"published");
        fs::remove_file(output).unwrap();
    }

    #[test]
    fn published_report_cleanup_error_identifies_post_publication_state() {
        let temporary = unique_temp_path("missing-cleanup-kind");
        let _ = fs::remove_file(&temporary);
        let error = cleanup_published_report(&temporary).unwrap_err();

        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        assert!(
            error
                .to_string()
                .contains("report published but temporary cleanup failed")
        );
    }

    #[test]
    fn complete_report_is_published_once() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let output = unique_temp_path(&format!("atomic-success-{nonce}"));
        let _ = fs::remove_file(&output);
        let report = sample_report("10.0.1");

        write_report(&output, &report).unwrap();
        let saved: serde_json::Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        assert_eq!(saved["zotero_version"], "10.0.1");
        assert!(write_report(&output, &report).is_err());
        fs::remove_file(output).unwrap();
    }

    #[test]
    fn output_path_must_be_a_new_direct_temp_child() {
        let allowed = unique_temp_path("allowed");
        let _ = fs::remove_file(&allowed);
        let expected_allowed = allowed
            .parent()
            .unwrap()
            .canonicalize()
            .unwrap()
            .join(allowed.file_name().unwrap());
        assert_eq!(
            validate_output_path(allowed.to_str().unwrap()).unwrap(),
            expected_allowed
        );

        assert!(validate_output_path("relative.json").is_err());
        assert_eq!(
            validate_output_path("/").unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        let parent_component = env::temp_dir().join("..");
        assert_eq!(
            validate_output_path(parent_component.to_str().unwrap())
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidInput
        );
        let missing_parent = env::temp_dir().join("conceptweave-zotero-missing-directory");
        let missing_report = missing_parent.join("report.json");
        assert_ne!(
            validate_output_path(missing_report.to_str().unwrap())
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidInput
        );
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

        assert!(conventional_tmp_parent(Path::new("/tmp/does-not-exist")).is_none());

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

    #[test]
    fn output_parent_policy_covers_optional_and_deduplicated_conventional_tmp() {
        let system_temp = PathBuf::from("system-temp");
        let conventional_tmp = PathBuf::from("conventional-tmp");

        assert_eq!(
            allowed_output_parent_policy(system_temp.clone(), None),
            vec![system_temp.clone()]
        );
        assert_eq!(
            allowed_output_parent_policy(system_temp.clone(), Some(system_temp.clone())),
            vec![system_temp.clone()]
        );
        assert_eq!(
            allowed_output_parent_policy(system_temp.clone(), Some(conventional_tmp.clone())),
            vec![system_temp, conventional_tmp]
        );
    }

    #[cfg(windows)]
    #[test]
    fn allowed_output_parents_does_not_require_posix_tmp() {
        let system_temp = env::temp_dir().canonicalize().unwrap();
        assert!(allowed_output_parents().contains(&system_temp));
    }

    #[cfg(unix)]
    #[test]
    fn new_report_files_are_owner_only() {
        use std::os::unix::fs::PermissionsExt;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let output = unique_temp_path(&format!("private-{nonce}"));
        let _ = fs::remove_file(&output);
        let file = open_new_output(&output).unwrap();
        drop(file);

        let mode = fs::metadata(&output).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        fs::remove_file(output).unwrap();
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
    fn output_path_uses_the_validated_parent_not_a_swappable_symlink() {
        use std::os::unix::fs::symlink;
        use std::time::{SystemTime, UNIX_EPOCH};

        let allowed_parent = env::temp_dir().canonicalize().unwrap();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let alias = allowed_parent.join(format!(
            "conceptweave-zotero-{}-{nonce}-parent-link",
            std::process::id()
        ));
        symlink(&allowed_parent, &alias).unwrap();
        let leaf = format!(
            "conceptweave-zotero-{}-{nonce}-canonical.json",
            std::process::id()
        );
        let output = alias.join(&leaf);

        let validated = validate_output_path(output.to_str().unwrap()).unwrap();
        assert_eq!(validated, allowed_parent.join(&leaf));

        fs::remove_file(alias).unwrap();
    }
}
