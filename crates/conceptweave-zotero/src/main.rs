#![forbid(unsafe_code)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use conceptweave_zotero::read_local_snapshot;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

#[cfg_attr(coverage_nightly, coverage(off))]
fn allowed_output_parents() -> Vec<PathBuf> {
    let system_temp = env::temp_dir()
        .canonicalize()
        .expect("system temporary directory must exist");
    let mut parents = vec![system_temp];
    if let Ok(conventional_tmp) = Path::new("/tmp").canonicalize() {
        if !parents.contains(&conventional_tmp) {
            parents.push(conventional_tmp);
        }
    }
    parents
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
        io::Error::new(io::ErrorKind::InvalidInput, "report output has no file name")
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

fn write_report<T: serde::Serialize>(
    output: &Path,
    report: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = open_new_output(output)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, report)?;
    writer.flush()?;
    Ok(())
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = env::args()
        .nth(1)
        .ok_or("usage: conceptweave-zotero /tmp/OUTPUT.json")?;
    let output = validate_output_path(&output)?;
    let report = read_local_snapshot()?;
    if report.zotero_version.starts_with("9.") {
        eprintln!("Zotero 9 Local API is read-only; writing a local proposal report only");
    }
    write_report(&output, &report)
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

        if Path::new("/tmp").is_dir() {
            let conventional = Path::new("/tmp").join(format!(
                "conceptweave-zotero-{}-conventional.json",
                std::process::id()
            ));
            let _ = fs::remove_file(&conventional);
            assert!(validate_output_path(conventional.to_str().unwrap()).is_ok());
        }

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
