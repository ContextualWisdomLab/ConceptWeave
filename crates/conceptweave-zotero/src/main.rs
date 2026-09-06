#![forbid(unsafe_code)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use conceptweave_zotero::{build_steward_review_worksheet, read_local_snapshot};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

const USAGE: &str = "usage: conceptweave-zotero /tmp/REPORT.json | --worksheet /tmp/REPORT.json /tmp/WORKSHEET.json";

fn parse_output_request<I, S>(args: I) -> Result<(Option<String>, String), &'static str>
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
        (Some(report), worksheet)
    } else {
        (None, first)
    };
    if args.next().is_some() {
        return Err("unexpected extra argument");
    }
    Ok(request)
}

fn write_private_output(path: &Path, content: &[u8]) -> io::Result<()> {
    write_private_output_with(path, content, write_all_and_flush)
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn write_all_and_flush(writer: &mut BufWriter<File>, content: &[u8]) -> io::Result<()> {
    writer.write_all(content)?;
    writer.flush()
}

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
    let file_name = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "report output has no file name",
        )
    })?;
    let validated_path = resolved_parent.join(file_name);
    if fs::symlink_metadata(&validated_path).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "report output must not already exist or be a symlink",
        ));
    }
    Ok(validated_path)
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
/// Creates a private file; failure leaves an empty file rather than unlinking a raced path.
fn create_report_file_with(
    path: &Path,
    set_permissions: fn(&File) -> io::Result<()>,
) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;

    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    let file = options.mode(0o600).open(path)?;
    set_permissions(&file)?;
    Ok(file)
}

#[cfg_attr(coverage_nightly, coverage(off))]
/// Reads one Zotero snapshot and writes its sensitive local proposal report.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (report_output, output) = parse_output_request(env::args().skip(1))?;
    let output = validate_output_path(&output)?;
    let report_output = report_output
        .as_deref()
        .map(validate_output_path)
        .transpose()?;
    let report = read_local_snapshot()?;
    if report.zotero_version.starts_with("9.") {
        eprintln!("Zotero 9 Local API is read-only; writing local proposal output only");
    }
    if let Some(report_output) = report_output {
        let worksheet = build_steward_review_worksheet(&report)?;
        let report_content = serde_json::to_vec_pretty(&report)?;
        let worksheet_content = serde_json::to_vec_pretty(&worksheet)?;
        write_private_output(&report_output, &report_content)?;
        if let Err(error) = write_private_output(&output, &worksheet_content) {
            let _ = fs::remove_file(report_output);
            return Err(error.into());
        }
    } else {
        write_private_output(&output, &serde_json::to_vec_pretty(&report)?)?;
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
            Ok((Some(report.to_owned()), worksheet.to_owned()))
        );
        assert_eq!(
            parse_output_request(vec![report]),
            Ok((None, report.to_owned()))
        );
        assert_eq!(parse_output_request(Vec::<&str>::new()), Err(USAGE));
        assert!(parse_output_request(vec!["--worksheet"]).is_err());
        assert!(parse_output_request(vec!["--worksheet", report]).is_err());
        assert!(parse_output_request(vec![report, "extra"]).is_err());
        assert!(parse_output_request(vec!["--worksheet", report, report]).is_err());
    }

    #[test]
    fn write_failure_preserves_replacement_at_output_path() {
        let output = unique_temp_path("write-race");
        let retained = unique_temp_path("write-race-retained");
        assert!(!output.exists());
        assert!(!retained.exists());
        let error = write_private_output_with(&output, b"content", |_, _| {
            let output = unique_temp_path("write-race");
            fs::rename(&output, unique_temp_path("write-race-retained"))?;
            let mut replacement = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(output)?;
            replacement.write_all(b"unrelated replacement")?;
            Err(io::Error::new(io::ErrorKind::WriteZero, "injected failure"))
        })
        .unwrap_err();
        let replacement = fs::read(&output);
        let _ = fs::remove_file(output);
        fs::remove_file(retained).unwrap();
        assert_eq!(error.kind(), io::ErrorKind::WriteZero);
        assert_eq!(replacement.unwrap(), b"unrelated replacement");
    }

    #[test]
    fn write_failure_does_not_retry_buffered_bytes_on_drop() {
        let output = unique_temp_path("write-buffer");
        let retained = unique_temp_path("write-buffer-retained");
        assert!(!output.exists());
        assert!(!retained.exists());
        let error = write_private_output_with(&output, b"buffered content", |writer, content| {
            fs::rename(
                unique_temp_path("write-buffer"),
                unique_temp_path("write-buffer-retained"),
            )?;
            writer.write_all(content)?;
            Err(io::Error::new(io::ErrorKind::WriteZero, "injected failure"))
        })
        .unwrap_err();
        let retained_bytes = fs::read(&retained).unwrap();
        fs::remove_file(retained).unwrap();
        assert_eq!(error.kind(), io::ErrorKind::WriteZero);
        assert!(retained_bytes.is_empty());
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
                .parent()
                .unwrap()
                .canonicalize()
                .unwrap()
                .join(allowed.file_name().unwrap())
        );

        assert!(validate_output_path("relative.json").is_err());
        assert!(validate_output_path("/").is_err());
        let missing_name =
            validate_output_path(env::temp_dir().join("..").to_str().unwrap()).unwrap_err();
        assert_eq!(missing_name.kind(), io::ErrorKind::InvalidInput);
        assert_eq!(missing_name.to_string(), "report output has no file name");
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
        assert_eq!(fs::metadata(&rejected).unwrap().len(), 0);
        fs::remove_file(rejected).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn permission_failure_preserves_a_replacement_at_the_output_path() {
        use std::os::unix::fs::PermissionsExt;
        let output = unique_temp_path("permission-replaced");
        let retained = unique_temp_path("permission-original");
        assert!(!output.exists());
        assert!(!retained.exists());
        let error = create_report_file_with(&output, |file| {
            assert_eq!(file.metadata()?.permissions().mode() & 0o077, 0);
            fs::rename(
                unique_temp_path("permission-replaced"),
                unique_temp_path("permission-original"),
            )?;
            let mut replacement = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(unique_temp_path("permission-replaced"))?;
            replacement.write_all(b"unrelated replacement")?;
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "injected failure",
            ))
        })
        .unwrap_err();
        let preserved = fs::read(&output).ok();
        let _ = fs::remove_file(&output);
        fs::remove_file(retained).unwrap();
        assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(
            preserved.as_deref(),
            Some(b"unrelated replacement".as_slice())
        );
    }

    #[cfg(unix)]
    #[test]
    fn report_output_returns_the_checked_canonical_parent() {
        let output = Path::new("/tmp").join(format!(
            "conceptweave-zotero-{}-canonical.json",
            std::process::id()
        ));
        let expected = Path::new("/tmp")
            .canonicalize()
            .unwrap()
            .join(output.file_name().unwrap());
        assert_eq!(
            validate_output_path(output.to_str().unwrap()).unwrap(),
            expected
        );
    }
}
