#![forbid(unsafe_code)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use conceptweave_zotero::read_local_snapshot;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

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
    let output = env::args()
        .nth(1)
        .ok_or("usage: conceptweave-zotero /tmp/OUTPUT.json")?;
    let output = validate_output_path(&output)?;
    let report = read_local_snapshot()?;
    if report.zotero_version.starts_with("9.") {
        eprintln!("Zotero 9 Local API is read-only; writing a local proposal report only");
    }
    let file = create_report_file(&output)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &report)?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(!output.exists() && !retained.exists());
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
        if output.exists() {
            fs::remove_file(&output).unwrap();
        }
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
