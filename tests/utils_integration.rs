use std::time::{Duration, SystemTime};

use filetime::{FileTime, set_file_mtime};
use tempfile::tempdir;

use rbackup::{LogContext, Messages, build_exclude_matcher, copy_incremental, is_newer};

#[test]
fn test_is_newer_integration() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src.txt");
    let dst = dir.path().join("dst.txt");

    std::fs::write(&src, b"src").unwrap();
    std::fs::write(&dst, b"dst").unwrap();

    let now = SystemTime::now();
    let older = FileTime::from_system_time(now - Duration::from_secs(60));
    let newer = FileTime::from_system_time(now + Duration::from_secs(60));

    set_file_mtime(&dst, older).unwrap();
    set_file_mtime(&src, newer).unwrap();

    assert!(is_newer(&src, &dst).unwrap());
    assert!(!is_newer(&dst, &src).unwrap());
}

#[test]
fn test_copy_incremental_integration() {
    let src_dir = tempdir().unwrap();
    let dst_dir = tempdir().unwrap();

    let src_path = src_dir.path().join("keep.txt");
    let src_skip = src_dir.path().join("skip.txt");
    let dst_keep = dst_dir.path().join("keep.txt");

    std::fs::write(&src_path, b"new").unwrap();
    std::fs::write(&src_skip, b"skip").unwrap();
    std::fs::write(&dst_keep, b"old").unwrap();

    // set dest keep older than src keep
    let now = SystemTime::now();
    let older = FileTime::from_system_time(now - Duration::from_secs(60));
    let newer = FileTime::from_system_time(now + Duration::from_secs(60));
    set_file_mtime(&dst_keep, older).unwrap();
    set_file_mtime(&src_path, newer).unwrap();

    // prepare messages
    let msg = Messages {
        cur_conf: "".into(),
        conf_file_not_found: "".into(),
        conf_initialized: "".into(),
        backup_init: "".into(),
        backup_ended: "".into(),
        starting_backup: "".into(),
        to: "".into(),
        copying_file: "copying".into(),
        language_not_supported: "".into(),
        files_total: "{}".into(),
        files_copied: "{}".into(),
        files_skipped: "{}".into(),
        copy_progress: "".into(),
        copied_file: "copied".into(),
        skipped_file: "skipped".into(),
        generic_error: "".into(),
        error_exclude_parsing: "".into(),
    };

    // build exclude matcher for skip.txt
    let matcher = build_exclude_matcher(&["skip.txt".to_string()], false).unwrap();

    let ctx = LogContext {
        quiet: true,
        timestamp_format: Some("%Y-%m-%d".into()),
        row: Some(1),
        on_log: false,
        exclude_matcher: Some(matcher),
        ..Default::default()
    };
    let (copied, skipped) =
        copy_incremental(src_dir.path(), dst_dir.path(), &msg, &ctx, false).unwrap();

    assert_eq!(copied, 1);
    assert_eq!(skipped, 1);

    // verify file exists in dest
    let dst_keep_contents = std::fs::read_to_string(dst_keep).unwrap();
    assert_eq!(dst_keep_contents, "new");
}

#[test]
fn test_copy_incremental_dry_run() {
    let src_dir = tempdir().unwrap();
    let dst_dir = tempdir().unwrap();

    let src_path = src_dir.path().join("file.txt");
    let dst_path = dst_dir.path().join("file.txt");

    std::fs::write(&src_path, b"content").unwrap();
    // ensure dst absent

    let msg = Messages {
        cur_conf: "".into(),
        conf_file_not_found: "".into(),
        conf_initialized: "".into(),
        backup_init: "".into(),
        backup_ended: "".into(),
        starting_backup: "".into(),
        to: "".into(),
        copying_file: "copying".into(),
        language_not_supported: "".into(),
        files_total: "{}".into(),
        files_copied: "{}".into(),
        files_skipped: "{}".into(),
        copy_progress: "".into(),
        copied_file: "copied".into(),
        skipped_file: "skipped".into(),
        generic_error: "".into(),
        error_exclude_parsing: "".into(),
    };

    let ctx = LogContext {
        quiet: true,
        timestamp_format: Some("%Y-%m-%d".into()),
        row: Some(1),
        on_log: false,
        dry_run: true,
        ..Default::default()
    };

    let (copied, skipped) =
        copy_incremental(src_dir.path(), dst_dir.path(), &msg, &ctx, false).unwrap();

    assert_eq!(copied, 1);
    assert_eq!(skipped, 0);
    assert!(!dst_path.exists()); // dry-run should not create file
}
