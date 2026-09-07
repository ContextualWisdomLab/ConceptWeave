#![forbid(unsafe_code)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use conceptweave_zotero::{
    ClassificationReport, FullTextCapture, FullTextReviewApproval, FullTextReviewWorksheet,
    GoldenSetApproval, MAX_REVIEW_BATCH_ITEMS, StewardDecisionPatch, StewardReviewBatch,
    StewardReviewWorksheet, apply_full_text_review_view, apply_steward_decision_patch,
    assess_steward_review_progress, build_bound_full_text_review_json, build_full_text_review_json,
    build_full_text_review_worksheet, build_steward_review_batch, build_steward_review_worksheet,
    decision_patch_from_review_batch, finalize_full_text_review, read_local_full_text,
    read_local_snapshot, reviewed_golden_set_from_worksheet,
};
use serde::de::DeserializeOwned;
use std::collections::BTreeSet;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

const USAGE: &str = "usage: conceptweave-zotero /tmp/REPORT.json | --capture-full-text /tmp/REPORT.json /tmp/CAPTURE.json | --full-text-worksheet /tmp/REPORT.json /tmp/CAPTURE.json /tmp/WORKSHEET.json | --bound-full-text-review /tmp/REPORT.json /tmp/WORKSHEET.json /tmp/CAPTURE.json LIMIT /tmp/VIEW.json | --apply-full-text-review /tmp/REPORT.json /tmp/WORKSHEET.json /tmp/CAPTURE.json /tmp/COMPLETED_VIEW.json /tmp/UPDATED.json | --finalize-full-text-review /tmp/REPORT.json /tmp/WORKSHEET.json /tmp/CAPTURE.json /tmp/APPROVAL.json /tmp/GOLDEN.json | --full-text-review /tmp/REPORT.json /tmp/WORKSHEET.json /tmp/CAPTURE.json LIMIT /tmp/VIEW.json | --worksheet /tmp/REPORT.json /tmp/WORKSHEET.json | --review-progress /tmp/REPORT.json /tmp/WORKSHEET.json /tmp/PROGRESS.json | --review-batch /tmp/REPORT.json /tmp/CURRENT_WORKSHEET.json LIMIT /tmp/BATCH.json | --apply-review-batch /tmp/REPORT.json /tmp/CURRENT_WORKSHEET.json /tmp/COMPLETED_BATCH.json /tmp/UPDATED_WORKSHEET.json | --apply-decision-patch /tmp/REPORT.json /tmp/CURRENT_WORKSHEET.json /tmp/PATCH.json /tmp/UPDATED_WORKSHEET.json | --finalize /tmp/REPORT.json /tmp/WORKSHEET.json /tmp/APPROVAL.json /tmp/GOLDEN.json";
const MAX_ARTIFACT_BYTES: u64 = 16 * 1024 * 1024;
// JSON escaping and envelope bytes are separate from the capture's raw-body budget.
const MAX_CAPTURE_FILE_BYTES: u64 = 512 * 1024 * 1024;

#[derive(Debug, PartialEq, Eq)]
enum OutputRequest {
    Report(String),
    FullTextCapture {
        report: String,
        output: String,
    },
    FullTextWorksheet {
        report: String,
        capture: String,
        output: String,
    },
    FullTextReview {
        report: String,
        worksheet: String,
        capture: String,
        limit: usize,
        output: String,
    },
    BoundFullTextReview {
        report: String,
        worksheet: String,
        capture: String,
        limit: usize,
        output: String,
    },
    ApplyFullTextReview {
        report: String,
        worksheet: String,
        capture: String,
        completed_view: String,
        output: String,
    },
    FinalizeFullTextReview {
        report: String,
        worksheet: String,
        capture: String,
        approval: String,
        output: String,
    },
    Worksheet {
        report: String,
        worksheet: String,
    },
    ReviewProgress {
        report: String,
        worksheet: String,
        output: String,
    },
    ReviewBatch {
        report: String,
        worksheet: String,
        limit: usize,
        output: String,
    },
    ApplyDecisionPatch {
        report: String,
        worksheet: String,
        patch: String,
        output: String,
    },
    ApplyReviewBatch {
        report: String,
        worksheet: String,
        batch: String,
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
    let request = if first == "--capture-full-text" {
        let report = args
            .next()
            .ok_or("--capture-full-text requires report and output paths")?;
        let output = args
            .next()
            .ok_or("--capture-full-text requires report and output paths")?;
        if report == output {
            return Err("full-text report and output paths must differ");
        }
        OutputRequest::FullTextCapture { report, output }
    } else if first == "--full-text-worksheet" {
        let report = args
            .next()
            .ok_or("--full-text-worksheet requires report, capture, and output paths")?;
        let capture = args
            .next()
            .ok_or("--full-text-worksheet requires report, capture, and output paths")?;
        let output = args
            .next()
            .ok_or("--full-text-worksheet requires report, capture, and output paths")?;
        if BTreeSet::from([&report, &capture, &output]).len() != 3 {
            return Err("full-text worksheet paths must be distinct");
        }
        OutputRequest::FullTextWorksheet {
            report,
            capture,
            output,
        }
    } else if first == "--apply-full-text-review" || first == "--finalize-full-text-review" {
        let report = args
            .next()
            .ok_or("full-text review application and finalization require five paths")?;
        let worksheet = args
            .next()
            .ok_or("full-text review application and finalization require five paths")?;
        let capture = args
            .next()
            .ok_or("full-text review application and finalization require five paths")?;
        let input = args
            .next()
            .ok_or("full-text review application and finalization require five paths")?;
        let output = args
            .next()
            .ok_or("full-text review application and finalization require five paths")?;
        if BTreeSet::from([&report, &worksheet, &capture, &input, &output]).len() != 5 {
            return Err("full-text review paths must be distinct");
        }
        if first == "--apply-full-text-review" {
            OutputRequest::ApplyFullTextReview {
                report,
                worksheet,
                capture,
                completed_view: input,
                output,
            }
        } else {
            OutputRequest::FinalizeFullTextReview {
                report,
                worksheet,
                capture,
                approval: input,
                output,
            }
        }
    } else if first == "--full-text-review" || first == "--bound-full-text-review" {
        let report = args
            .next()
            .ok_or("full-text review mode requires a report path")?;
        let worksheet = args
            .next()
            .ok_or("full-text review mode requires a worksheet path")?;
        let capture = args
            .next()
            .ok_or("full-text review mode requires a capture path")?;
        let limit = args
            .next()
            .ok_or("full-text review mode requires a limit")?;
        let output = args
            .next()
            .ok_or("full-text review mode requires an output path")?;
        if limit.is_empty() || !limit.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err("full-text review limit must be an unsigned decimal integer");
        }
        let limit: usize = limit
            .parse()
            .map_err(|_| "full-text review limit is out of range")?;
        if !(1..=MAX_REVIEW_BATCH_ITEMS).contains(&limit) {
            return Err("full-text review limit must be between 1 and 100");
        }
        if BTreeSet::from([&report, &worksheet, &capture, &output]).len() != 4 {
            return Err("full-text review paths must be distinct");
        }
        if first == "--bound-full-text-review" {
            OutputRequest::BoundFullTextReview {
                report,
                worksheet,
                capture,
                limit,
                output,
            }
        } else {
            OutputRequest::FullTextReview {
                report,
                worksheet,
                capture,
                limit,
                output,
            }
        }
    } else if first == "--worksheet" {
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
    } else if first == "--review-batch" {
        let report = args
            .next()
            .ok_or("--review-batch requires report, worksheet, limit, and output")?;
        let worksheet = args
            .next()
            .ok_or("--review-batch requires report, worksheet, limit, and output")?;
        let limit = args
            .next()
            .ok_or("--review-batch requires report, worksheet, limit, and output")?;
        let output = args
            .next()
            .ok_or("--review-batch requires report, worksheet, limit, and output")?;
        if BTreeSet::from([report.as_str(), worksheet.as_str(), output.as_str()]).len() != 3 {
            return Err("review batch artifact paths must differ");
        }
        if limit.is_empty() || !limit.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err("review batch limit must be an unsigned decimal integer");
        }
        let limit = limit
            .parse::<usize>()
            .map_err(|_| "review batch limit is out of range")?;
        if !(1..=MAX_REVIEW_BATCH_ITEMS).contains(&limit) {
            return Err("review batch limit must be between 1 and 100");
        }
        OutputRequest::ReviewBatch {
            report,
            worksheet,
            limit,
            output,
        }
    } else if first == "--apply-review-batch" {
        let report = args
            .next()
            .ok_or("--apply-review-batch requires four artifact paths")?;
        let worksheet = args
            .next()
            .ok_or("--apply-review-batch requires four artifact paths")?;
        let batch = args
            .next()
            .ok_or("--apply-review-batch requires four artifact paths")?;
        let output = args
            .next()
            .ok_or("--apply-review-batch requires four artifact paths")?;
        if BTreeSet::from([
            report.as_str(),
            worksheet.as_str(),
            batch.as_str(),
            output.as_str(),
        ])
        .len()
            != 4
        {
            return Err("review batch application artifact paths must differ");
        }
        OutputRequest::ApplyReviewBatch {
            report,
            worksheet,
            batch,
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
    read_private_input(raw, |file, length| read_bounded_json(file, length))
}

/// Retains exact completed-view bytes for the duplicate-key-aware review parser.
fn read_private_bytes(raw: &str) -> io::Result<(Vec<u8>, ArtifactIdentity)> {
    read_private_input(raw, |file, length| read_bounded_bytes(file, length))
}

/// Restores a private capture without allocating a second full-file byte buffer.
fn read_private_capture(raw: &str) -> io::Result<(FullTextCapture, ArtifactIdentity)> {
    read_private_input(raw, |file, length| {
        read_bounded_capture(file, length, MAX_CAPTURE_FILE_BYTES)
    })
}

/// Shares the validated, single-open file boundary across bounded artifact parsers.
fn read_private_input<T>(
    raw: &str,
    parse: impl FnOnce(&mut File, u64) -> io::Result<T>,
) -> io::Result<(T, ArtifactIdentity)> {
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
    let resolved_parent = parent.canonicalize()?;
    if !allowed_output_parents().contains(&resolved_parent) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "review input must be a direct child of the system temp directory",
        ));
    }
    let file_name = path.file_name().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "review input has no file name")
    })?;
    let validated_path = resolved_parent.join(file_name);
    let path_metadata = fs::symlink_metadata(&validated_path)?;
    if path_metadata.file_type().is_symlink() || !path_metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "review input must be a regular file",
        ));
    }
    let (file, opened_metadata) = open_with_metadata(&validated_path)?;
    #[cfg(not(unix))]
    {
        let _ = (path_metadata, opened_metadata, file, parse);
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "private review input requires a Unix platform",
        ));
    }
    #[cfg(unix)]
    let identity = validate_opened_identity(&path_metadata, &opened_metadata)?;
    #[cfg(unix)]
    {
        let parsed = parse(&mut { file }, opened_metadata.len())?;
        Ok((parsed, identity))
    }
}

/// Opens without following a symlink or waiting for a raced FIFO, then reads handle metadata.
fn open_with_metadata(path: &Path) -> io::Result<(File, fs::Metadata)> {
    #[cfg(unix)]
    use std::os::unix::fs::OpenOptionsExt;

    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
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

/// Reads bounded JSON without exposing rejected field names or values in diagnostics.
fn read_bounded_json<T: DeserializeOwned>(
    reader: &mut dyn Read,
    advertised_len: u64,
) -> io::Result<T> {
    serde_json::from_slice(&read_bounded_bytes(reader, advertised_len)?)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "review input is invalid"))
}

/// Shares the metadata byte ceiling without normalizing completed review JSON.
fn read_bounded_bytes(reader: &mut dyn Read, advertised_len: u64) -> io::Result<Vec<u8>> {
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
    Ok(content)
}

/// Parses capture JSON through a fixed buffer and rejects size drift or trailing data.
fn read_bounded_capture<T: DeserializeOwned>(
    reader: &mut dyn Read,
    advertised_len: u64,
    file_limit: u64,
) -> io::Result<T> {
    if advertised_len > file_limit {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "full-text capture input exceeds the file size limit",
        ));
    }
    let mut bounded = reader.take(file_limit + 1);
    // from_reader checks the complete stream, including trailing whitespace.
    let parsed = serde_json::from_reader(BufReader::new(&mut bounded));
    if bounded.limit() == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "full-text capture input grew beyond the file size limit",
        ));
    }
    let parsed = parsed.map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "full-text capture input is invalid",
        )
    })?;
    if file_limit + 1 - bounded.limit() != advertised_len {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "full-text capture input changed size while reading",
        ));
    }
    Ok(parsed)
}

/// Preserves an input error kind while naming the rejected artifact.
fn label_input(name: &str, error: io::Error) -> io::Error {
    io::Error::new(error.kind(), format!("{name}: {error}"))
}

/// Validates canonical report and worksheet destinations before reading Zotero.
fn validate_output_request(
    report_output: Option<&str>,
    output: &str,
) -> io::Result<(Option<PathBuf>, PathBuf)> {
    let output = validate_output_path(output)?;
    let report_output = report_output.map(validate_output_path).transpose()?;
    if report_output.as_ref() == Some(&output) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "report and worksheet output paths must differ",
        ));
    }
    Ok((report_output, output))
}

/// Writes one bounded create-new private artifact; failures may leave partial files.
fn write_private_output(path: &Path, content: &[u8]) -> io::Result<()> {
    if content.len() as u64 > MAX_ARTIFACT_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "metadata output exceeds the artifact size limit",
        ));
    }
    write_private_output_with(path, content, &mut write_all_and_flush)
}

/// Writes and flushes the complete serialized artifact.
fn write_all_and_flush(writer: &mut BufWriter<File>, content: &[u8]) -> io::Result<()> {
    writer.write_all(content)?;
    writer.flush()
}

/// Runs the private-output boundary with an injectable writer for failure testing.
type PrivateOutputWriter<'writer> =
    dyn FnMut(&mut BufWriter<File>, &[u8]) -> io::Result<()> + 'writer;

fn write_private_output_with(
    path: &Path,
    content: &[u8],
    write: &mut PrivateOutputWriter<'_>,
) -> io::Result<()> {
    let file = create_report_file(path)?;
    let mut writer = BufWriter::new(file);
    let result = write(&mut writer, content);
    // Never retry buffered writes on drop or unlink a possibly replaced pathname.
    let _ = writer.into_parts();
    result
}

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
    match parse_output_request(env::args().skip(1))? {
        OutputRequest::BoundFullTextReview {
            report,
            worksheet,
            capture,
            limit,
            output,
        } => {
            let output_path = validate_output_path(&output)?;
            let (report, report_identity): (ClassificationReport, _) =
                read_private_json(&report).map_err(|error| label_input("report", error))?;
            let (worksheet, worksheet_identity): (FullTextReviewWorksheet, _) =
                read_private_json(&worksheet).map_err(|error| label_input("worksheet", error))?;
            let (capture, capture_identity) =
                read_private_capture(&capture).map_err(|error| label_input("capture", error))?;
            if BTreeSet::from([report_identity, worksheet_identity, capture_identity]).len() != 3 {
                return Err("full-text review inputs must be distinct files".into());
            }
            let content = build_bound_full_text_review_json(&report, &worksheet, &capture, limit)?;
            write_private_output(&output_path, &content)?;
        }
        OutputRequest::ApplyFullTextReview {
            report,
            worksheet,
            capture,
            completed_view,
            output,
        } => {
            let output_path = validate_output_path(&output)?;
            let (report, report_identity): (ClassificationReport, _) =
                read_private_json(&report).map_err(|error| label_input("report", error))?;
            let (worksheet, worksheet_identity): (FullTextReviewWorksheet, _) =
                read_private_json(&worksheet).map_err(|error| label_input("worksheet", error))?;
            let (capture, capture_identity) =
                read_private_capture(&capture).map_err(|error| label_input("capture", error))?;
            let (completed_view, view_identity) = read_private_bytes(&completed_view)
                .map_err(|error| label_input("completed view", error))?;
            if BTreeSet::from([
                report_identity,
                worksheet_identity,
                capture_identity,
                view_identity,
            ])
            .len()
                != 4
            {
                return Err("full-text review inputs must be distinct files".into());
            }
            let updated =
                apply_full_text_review_view(&report, &worksheet, &capture, &completed_view)?;
            write_private_output(&output_path, &serde_json::to_vec_pretty(&updated)?)?;
        }
        OutputRequest::FinalizeFullTextReview {
            report,
            worksheet,
            capture,
            approval,
            output,
        } => {
            let output_path = validate_output_path(&output)?;
            let (report, report_identity): (ClassificationReport, _) =
                read_private_json(&report).map_err(|error| label_input("report", error))?;
            let (worksheet, worksheet_identity): (FullTextReviewWorksheet, _) =
                read_private_json(&worksheet).map_err(|error| label_input("worksheet", error))?;
            let (capture, capture_identity) =
                read_private_capture(&capture).map_err(|error| label_input("capture", error))?;
            let (approval, approval_identity): (FullTextReviewApproval, _) =
                read_private_json(&approval).map_err(|error| label_input("approval", error))?;
            if BTreeSet::from([
                report_identity,
                worksheet_identity,
                capture_identity,
                approval_identity,
            ])
            .len()
                != 4
            {
                return Err("full-text review inputs must be distinct files".into());
            }
            let golden = finalize_full_text_review(&report, &worksheet, &capture, approval)?;
            write_private_output(&output_path, &serde_json::to_vec_pretty(&golden)?)?;
        }
        OutputRequest::FullTextWorksheet {
            report,
            capture,
            output,
        } => {
            let output_path = validate_output_path(&output)?;
            let (report, report_identity): (ClassificationReport, _) =
                read_private_json(&report).map_err(|error| label_input("report", error))?;
            let (capture, capture_identity) =
                read_private_capture(&capture).map_err(|error| label_input("capture", error))?;
            if report_identity == capture_identity {
                return Err("full-text worksheet inputs must be distinct files".into());
            }
            let worksheet = build_full_text_review_worksheet(&report, &capture)?;
            write_private_output(&output_path, &serde_json::to_vec_pretty(&worksheet)?)?;
        }
        OutputRequest::FullTextReview {
            report,
            worksheet,
            capture,
            limit,
            output,
        } => {
            let output_path = validate_output_path(&output)?;
            let (report, report_identity): (ClassificationReport, _) =
                read_private_json(&report).map_err(|error| label_input("report", error))?;
            let (worksheet, worksheet_identity): (StewardReviewWorksheet, _) =
                read_private_json(&worksheet).map_err(|error| label_input("worksheet", error))?;
            let (capture, capture_identity) =
                read_private_capture(&capture).map_err(|error| label_input("capture", error))?;
            if BTreeSet::from([report_identity, worksheet_identity, capture_identity]).len() != 3 {
                return Err("full-text review inputs must be distinct files".into());
            }
            let content = build_full_text_review_json(&report, &worksheet, &capture, limit)?;
            write_private_output(&output_path, &content)?;
        }
        OutputRequest::FullTextCapture { report, output } => {
            let output = validate_output_path(&output)?;
            let (report, _): (ClassificationReport, _) =
                read_private_json(&report).map_err(|error| label_input("report", error))?;
            let capture = read_local_full_text(&report)?;
            write_private_output_with(&output, &[], &mut |writer, _| {
                serde_json::to_writer(&mut *writer, &capture).map_err(io::Error::other)?;
                writer.flush()
            })?;
        }
        OutputRequest::Report(output) => {
            let output = validate_output_path(&output)?;
            let report = read_local_snapshot()?;
            if report.zotero_version.starts_with("9.") {
                eprintln!("Zotero 9 Local API is read-only; writing local proposal output only");
            }
            write_private_output(&output, &serde_json::to_vec_pretty(&report)?)?;
        }
        OutputRequest::Worksheet { report, worksheet } => {
            let (report_output, worksheet_output) =
                validate_output_request(Some(&report), &worksheet)?;
            let report_output = report_output.expect("worksheet request includes report path");
            let report = read_local_snapshot()?;
            if report.zotero_version.starts_with("9.") {
                eprintln!("Zotero 9 Local API is read-only; writing local proposal output only");
            }
            let worksheet = build_steward_review_worksheet(&report)?;
            let report_content = serde_json::to_vec_pretty(&report)?;
            let worksheet_content = serde_json::to_vec_pretty(&worksheet)?;
            write_private_output(&report_output, &report_content)?;
            write_private_output(&worksheet_output, &worksheet_content)?;
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
        OutputRequest::ReviewBatch {
            report,
            worksheet,
            limit,
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
                    "review batch inputs must be distinct files",
                )
                .into());
            }
            let batch = build_steward_review_batch(&report, &worksheet, limit)?;
            let content = serde_json::to_vec_pretty(&batch)?;
            write_private_output(&output, &content)?;
        }
        OutputRequest::ApplyReviewBatch {
            report,
            worksheet,
            batch,
            output,
        } => {
            let output = validate_output_path(&output)?;
            let (report, report_identity): (ClassificationReport, _) =
                read_private_json(&report).map_err(|error| label_input("report", error))?;
            let (worksheet, worksheet_identity): (StewardReviewWorksheet, _) =
                read_private_json(&worksheet).map_err(|error| label_input("worksheet", error))?;
            let (batch, batch_identity): (StewardReviewBatch, _) =
                read_private_json(&batch).map_err(|error| label_input("review batch", error))?;
            if BTreeSet::from([report_identity, worksheet_identity, batch_identity]).len() != 3 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "review batch application inputs must be distinct files",
                )
                .into());
            }
            let patch = decision_patch_from_review_batch(&report, &worksheet, &batch)?;
            let updated = apply_steward_decision_patch(&report, &worksheet, &patch)?;
            write_private_output(&output, &serde_json::to_vec_pretty(&updated)?)?;
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
mod full_text_review_cli_reader_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bound_full_text_modes_require_complete_distinct_paths_and_limits() {
        for mode in [
            "--bound-full-text-review",
            "--apply-full-text-review",
            "--finalize-full-text-review",
        ] {
            let fourth = if mode == "--bound-full-text-review" {
                "1"
            } else {
                "input"
            };
            let args = [mode, "report", "worksheet", "capture", fourth, "output"];
            assert!(parse_output_request(args).is_ok());
            for count in 1..6 {
                assert!(parse_output_request(args[..count].iter().copied()).is_err());
            }
            assert!(parse_output_request([mode, "r", "r", "c", fourth, "o"]).is_err());
            assert!(parse_output_request([mode, "r", "w", "c", fourth, "c"]).is_err());
            assert!(parse_output_request([mode, "r", "w", "c", fourth, "o", "extra"]).is_err());
        }
        for limit in [
            "",
            "0",
            "101",
            "-1",
            "+1",
            " 1",
            "one",
            "999999999999999999999999",
        ] {
            assert!(
                parse_output_request(["--bound-full-text-review", "r", "w", "c", limit, "o"])
                    .is_err()
            );
        }
    }

    #[test]
    fn full_text_worksheet_mode_requires_three_distinct_paths() {
        assert!(
            parse_output_request([
                "--full-text-worksheet",
                "/tmp/report.json",
                "/tmp/capture.json",
                "/tmp/worksheet.json",
            ])
            .is_ok()
        );
        for length in 1..4 {
            let arguments = ["--full-text-worksheet", "r", "c", "o"];
            assert!(parse_output_request(arguments[..length].iter().copied()).is_err());
        }
        for paths in [["r", "r", "o"], ["r", "c", "r"], ["r", "c", "c"]] {
            assert!(
                parse_output_request(["--full-text-worksheet", paths[0], paths[1], paths[2]])
                    .is_err()
            );
        }
        assert!(
            parse_output_request(["--full-text-worksheet", "r", "c", "o", "metadata-worksheet"])
                .is_err()
        );
    }

    #[test]
    fn full_text_review_mode_requires_five_distinct_bounded_arguments() {
        assert!(
            parse_output_request([
                "--full-text-review",
                "/tmp/report.json",
                "/tmp/worksheet.json",
                "/tmp/capture.json",
                "2",
                "/tmp/view.json",
            ])
            .is_ok()
        );
        for length in 1..6 {
            let arguments = ["--full-text-review", "r", "w", "c", "1", "o"];
            assert!(parse_output_request(arguments[..length].iter().copied()).is_err());
        }
        for limit in [
            "",
            "0",
            "101",
            " 1",
            "+1",
            "-1",
            "one",
            "999999999999999999999999999",
        ] {
            assert!(
                parse_output_request(["--full-text-review", "r", "w", "c", limit, "o"]).is_err()
            );
        }
        for paths in [
            ["r", "r", "c", "o"],
            ["r", "w", "r", "o"],
            ["r", "w", "c", "r"],
        ] {
            assert!(
                parse_output_request([
                    "--full-text-review",
                    paths[0],
                    paths[1],
                    paths[2],
                    "1",
                    paths[3]
                ])
                .is_err()
            );
        }
        assert!(
            parse_output_request(["--full-text-review", "r", "w", "c", "1", "o", "extra"]).is_err()
        );
    }

    #[test]
    fn full_text_mode_requires_two_distinct_artifact_paths() {
        let report = "/tmp/report.json";
        let output = "/tmp/full-text.json";
        assert_eq!(
            parse_output_request(["--capture-full-text", report, output]),
            Ok(OutputRequest::FullTextCapture {
                report: report.into(),
                output: output.into(),
            })
        );
        assert!(parse_output_request(["--capture-full-text"]).is_err());
        assert!(parse_output_request(["--capture-full-text", report]).is_err());
        assert!(parse_output_request(["--capture-full-text", report, report]).is_err());
        assert!(parse_output_request(["--capture-full-text", report, output, "extra"]).is_err());
    }

    #[test]
    #[cfg(unix)]
    fn opening_private_input_refuses_a_symlink_before_reading() {
        use std::os::unix::fs::symlink;
        let target = unique_temp_path("open-target");
        let link = unique_temp_path("open-symlink");
        write_private_output(&target, b"{}").unwrap();
        symlink(&target, &link).unwrap();
        let result = open_with_metadata(&link);
        fs::remove_file(&link).unwrap();
        fs::remove_file(&target).unwrap();
        assert!(result.is_err());
    }

    #[test]
    fn private_input_rejects_a_nameless_child_of_the_validated_parent() {
        let path = env::temp_dir().join("..");
        let error = read_private_json::<serde_json::Value>(path.to_str().unwrap()).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert_eq!(error.to_string(), "review input has no file name");
    }

    #[test]
    #[cfg(unix)]
    fn replaced_fifo_does_not_block_before_identity_rejection() {
        let target = unique_temp_path("fifo-replacement");
        let retained = unique_temp_path("fifo-retained");
        write_private_output(&target, b"{}").unwrap();
        let checked_metadata = fs::symlink_metadata(&target).unwrap();
        fs::rename(&target, &retained).unwrap();
        assert!(
            std::process::Command::new("mkfifo")
                .arg(&target)
                .status()
                .unwrap()
                .success()
        );
        let (sender, receiver) = std::sync::mpsc::channel();
        let opened_path = target.clone();
        std::thread::spawn(move || {
            let result = open_with_metadata(&opened_path).map(|(_, metadata)| metadata);
            let _ = sender.send(result);
        });
        let result = receiver.recv_timeout(std::time::Duration::from_secs(2));
        fs::remove_file(&target).unwrap();
        fs::remove_file(&retained).unwrap();
        let opened_metadata = result
            .expect("opening a raced FIFO must not wait for a writer")
            .unwrap();
        assert_eq!(
            validate_opened_identity(&checked_metadata, &opened_metadata)
                .unwrap_err()
                .kind(),
            io::ErrorKind::PermissionDenied
        );
    }

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
            parse_output_request(vec![
                "--apply-decision-patch",
                report,
                worksheet,
                patch,
                output
            ]),
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
            parse_output_request(vec![
                "--apply-decision-patch",
                report,
                worksheet,
                patch,
                report
            ])
            .is_err()
        );
        assert!(
            parse_output_request(vec![
                "--apply-decision-patch",
                report,
                worksheet,
                worksheet,
                output
            ])
            .is_err()
        );
        assert!(
            parse_output_request(vec![
                "--apply-decision-patch",
                report,
                report,
                patch,
                output
            ])
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

    #[test]
    fn apply_review_batch_mode_requires_four_distinct_artifact_paths() {
        let report = "/tmp/report.json";
        let worksheet = "/tmp/worksheet.json";
        let batch = "/tmp/batch.json";
        let output = "/tmp/updated-worksheet.json";
        assert_eq!(
            parse_output_request(vec![
                "--apply-review-batch",
                report,
                worksheet,
                batch,
                output,
            ]),
            Ok(OutputRequest::ApplyReviewBatch {
                report: report.to_owned(),
                worksheet: worksheet.to_owned(),
                batch: batch.to_owned(),
                output: output.to_owned(),
            })
        );
        assert!(parse_output_request(vec!["--apply-review-batch"]).is_err());
        assert!(parse_output_request(vec!["--apply-review-batch", report]).is_err());
        assert!(parse_output_request(vec!["--apply-review-batch", report, worksheet]).is_err());
        assert!(
            parse_output_request(vec!["--apply-review-batch", report, worksheet, batch]).is_err()
        );
        assert!(
            parse_output_request(vec![
                "--apply-review-batch",
                report,
                worksheet,
                batch,
                report,
            ])
            .is_err()
        );
        assert!(
            parse_output_request(vec![
                "--apply-review-batch",
                report,
                worksheet,
                batch,
                output,
                "extra",
            ])
            .is_err()
        );
    }

    #[test]
    fn review_batch_mode_requires_distinct_paths_and_decimal_limit() {
        let report = "/tmp/report.json";
        let worksheet = "/tmp/worksheet.json";
        let output = "/tmp/batch.json";
        assert_eq!(
            parse_output_request(vec!["--review-batch", report, worksheet, "25", output]),
            Ok(OutputRequest::ReviewBatch {
                report: report.to_owned(),
                worksheet: worksheet.to_owned(),
                limit: 25,
                output: output.to_owned(),
            })
        );
        for limit in [
            "",
            "0",
            "101",
            " 1",
            "+1",
            "-1",
            "one",
            "9999999999999999999999999999999999999999",
        ] {
            assert!(
                parse_output_request(vec!["--review-batch", report, worksheet, limit, output])
                    .is_err()
            );
        }
        assert!(parse_output_request(vec!["--review-batch"]).is_err());
        assert!(parse_output_request(vec!["--review-batch", report]).is_err());
        assert!(parse_output_request(vec!["--review-batch", report, worksheet]).is_err());
        assert!(parse_output_request(vec!["--review-batch", report, worksheet, "25"]).is_err());
        assert!(
            parse_output_request(vec!["--review-batch", report, report, "25", output]).is_err()
        );
        assert!(
            parse_output_request(vec!["--review-batch", report, worksheet, "25", report]).is_err()
        );
        assert!(
            parse_output_request(vec![
                "--review-batch",
                report,
                worksheet,
                "25",
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
        assert!(read_private_json::<serde_json::Value>("/tmp/..").is_err());
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
        assert_eq!(
            oversized.to_string(),
            "review input exceeds the artifact size limit"
        );

        let grown = read_bounded_json::<serde_json::Value>(
            &mut io::repeat(b' ').take(MAX_ARTIFACT_BYTES + 1),
            0,
        )
        .unwrap_err();
        assert_eq!(grown.kind(), io::ErrorKind::InvalidData);
        assert_eq!(
            grown.to_string(),
            "review input grew beyond the artifact size limit"
        );

        let read_failure =
            read_bounded_json::<serde_json::Value>(&mut FailingReader, 0).unwrap_err();
        assert_eq!(read_failure.kind(), io::ErrorKind::Other);
        assert_eq!(read_failure.to_string(), "injected read failure");

        let labeled = label_input(
            "worksheet",
            io::Error::new(io::ErrorKind::PermissionDenied, "unsafe"),
        );
        assert_eq!(labeled.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(labeled.to_string(), "worksheet: unsafe");
    }

    #[test]
    fn private_json_diagnostic_hides_unknown_field_names() {
        #[derive(Debug, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct StrictReviewInput {}

        // Original metadata types permit additional fields. This strict test-only type
        // exercises the shared reader's confidentiality contract for future callers.
        let error = read_bounded_json::<StrictReviewInput>(
            &mut br#"{"synthetic-private-field-sentinel":true}"#.as_slice(),
            0,
        )
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(error.to_string(), "review input is invalid");
        assert_eq!(
            label_input("worksheet", error).to_string(),
            "worksheet: review input is invalid"
        );
    }

    #[test]
    fn private_json_diagnostic_hides_rejected_enum_values() {
        let error = read_bounded_json::<conceptweave_zotero::Disposition>(
            &mut br#""synthetic-private-enum-sentinel""#.as_slice(),
            0,
        )
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(error.to_string(), "review input is invalid");
    }

    #[cfg(unix)]
    #[test]
    fn private_output_limit_accepts_exactly_the_readable_artifact_boundary() {
        use std::os::unix::fs::PermissionsExt;

        let output = unique_temp_path("exact-output-limit");
        assert!(!output.exists());
        let mut content = vec![b' '; MAX_ARTIFACT_BYTES as usize];
        content[..2].copy_from_slice(b"{}");
        write_private_output(&output, &content).unwrap();
        let metadata = fs::metadata(&output).unwrap();
        let restored = read_private_json::<serde_json::Value>(output.to_str().unwrap());
        fs::remove_file(output).unwrap();
        assert_eq!(metadata.len(), MAX_ARTIFACT_BYTES);
        assert_eq!(metadata.permissions().mode() & 0o777, 0o600);
        assert_eq!(restored.unwrap().0, serde_json::json!({}));
    }

    #[cfg(unix)]
    #[test]
    fn private_output_limit_rejects_oversize_before_creating_or_touching_a_file() {
        let output = unique_temp_path("oversized-output-limit");
        let existing = unique_temp_path("existing-output-limit");
        assert!(!output.exists());
        assert!(!existing.exists());
        write_private_output(&existing, b"original").unwrap();
        let content = vec![b' '; MAX_ARTIFACT_BYTES as usize + 1];
        let rejected = write_private_output(&output, &content);
        let touched = write_private_output(&existing, &content);
        let created = output.exists();
        let preserved = fs::read(&existing).unwrap();
        // Clean synthetic artifacts even when the RED implementation creates the file.
        let _ = fs::remove_file(output);
        fs::remove_file(existing).unwrap();
        assert!(
            !created,
            "oversized metadata must be rejected before creation"
        );
        assert_eq!(preserved, b"original");
        for error in [rejected.unwrap_err(), touched.unwrap_err()] {
            assert_eq!(error.kind(), io::ErrorKind::InvalidData);
            assert_eq!(
                error.to_string(),
                "metadata output exceeds the artifact size limit"
            );
        }
    }

    #[test]
    fn canonical_output_aliases_are_rejected_without_creating_files() {
        let output =
            Path::new("/tmp").join(format!("conceptweave-alias-{}.json", std::process::id()));
        let canonical = output
            .parent()
            .unwrap()
            .canonicalize()
            .unwrap()
            .join(output.file_name().unwrap());
        let result =
            validate_output_request(Some(output.to_str().unwrap()), canonical.to_str().unwrap());
        assert!(!output.exists());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidInput);
        let other = unique_temp_path("other-output");
        assert!(
            validate_output_request(Some(output.to_str().unwrap()), other.to_str().unwrap())
                .is_ok()
        );
        assert!(validate_output_request(None, output.to_str().unwrap()).is_ok());
        assert!(validate_output_request(None, "relative-output").is_err());
        assert!(
            validate_output_request(Some("relative-report"), output.to_str().unwrap()).is_err()
        );
    }

    #[test]
    fn write_failure_preserves_replacement_at_output_path() {
        let output = unique_temp_path("write-race");
        let retained = unique_temp_path("write-race-retained");
        assert!(!output.exists());
        assert!(!retained.exists());
        let error = write_private_output_with(&output, b"content", &mut |_, _| {
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
        let error =
            write_private_output_with(&output, b"buffered content", &mut |writer, content| {
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
    fn failed_private_output_is_preserved_and_requires_a_new_path() {
        let output = unique_temp_path("failed-output");
        let _ = fs::remove_file(&output);
        let error = write_private_output_with(&output, b"content", &mut |_, _| {
            Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "injected write failure",
            ))
        })
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::WriteZero);
        assert_eq!(fs::metadata(&output).unwrap().len(), 0);
        assert!(write_private_output(&output, b"retry").is_err());
        fs::remove_file(&output).unwrap();

        write_private_output(&output, b"complete").unwrap();
        assert_eq!(fs::read(&output).unwrap(), b"complete");
        assert!(write_private_output(&output, b"replacement").is_err());
        fs::remove_file(output).unwrap();

        let read_only = unique_temp_path("read-only-writer");
        fs::write(&read_only, b"input").unwrap();
        let mut writer = BufWriter::with_capacity(1, File::open(&read_only).unwrap());
        assert!(write_all_and_flush(&mut writer, b"content").is_err());
        fs::remove_file(read_only).unwrap();
    }

    pub(super) fn unique_temp_path(suffix: &str) -> PathBuf {
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
        let missing_name = validate_output_path(env::temp_dir().join("..").to_str().unwrap())
            .expect_err("an allowed parent still requires a file name");
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
