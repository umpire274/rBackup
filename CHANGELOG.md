# 📋 Changelog

📖 [Back to README](README.md)

All notable changes to the `rbackup` project will be documented in this file.

---

## [v0.5.1] - 2025-10-01

### 🔧 Change

- Updated the end-of-copy message to include the total number of files processed (copied + skipped) in addition to the separate counts for copied and skipped files. This helps quickly see the overall scope of the operation.

---

## [v0.5.0] - 2025-10-01

This release closes GitHub issue #3 — fixes and improvements described below.

### 🚀 New features

- Added enhanced exclude support for `copy`:
    - `--exclude <PATTERN>` accepts glob patterns applied by default to the path relative to the source directory.
    - `--absolute-exclude` option to match patterns against absolute source paths.
    - `--ignore-case` option for case-insensitive exclude matching.
    - Exclude matching now also tests the file basename (filename only), solving cases like `$RECYCLE.BIN`, `Thumbs.db`,
      etc.
- Added `--dry-run` behavior improvements: dry-run now simulates copies and reports counts without touching the
  destination.

### 🧾 Logging & UX

- Files that are skipped (either excluded by a pattern or because destination is newer) are now logged both to stdout
  and to the log file specified with `--log`.
- Logs include the exclude pattern that caused the skip (when applicable), making it easier to diagnose why files were
  skipped.
- Timestamps in logs are configurable and the `LogContext` carries the timestamp format.

### 🔧 Refactor & robustness

- `Config::load_or_default()` added to provide a safe fallback when the configuration file is missing or malformed.
- `create_logger()` now returns a `Result<Option<Logger>>` instead of panicking on errors; logging file creation
  failures is handled gracefully.
- `copy_incremental()` now streams `WalkDir` results (no longer builds an in-memory Vec of all files), reducing memory
  usage for large trees.
- Centralized and simplified CLI parsing via a shared `cli` module (`src/cli.rs`) so the binary and tests reuse the same
  definitions.
- Split test code out of `src/` into `tests/` integration tests and added tests for `is_newer`, exclude handling, and
  dry-run behavior.
- Improved handling of mutex poisoning when writing to the log file; flusher calls added to ensure buffered logs are
  persisted.

### ✅ Fixes

- Fixed cases where some exclude patterns (basename patterns like `$RECYCLE.BIN`) were not matched.
- Fixed logging so skipped files are included in the log file output.

### 🧪 Tests

- Added integration tests under `tests/` to exercise exclude matching, `is_newer` logic and dry-run semantics.

---

## [v0.4.0] - 2025-10-01

### 🚀 New features

- Added enhanced exclude support for `copy`:
    - `--exclude <PATTERN>` accepts glob patterns applied by default to the path relative to the source directory.
    - `--absolute-exclude` option to match patterns against absolute source paths.
    - `--ignore-case` option for case-insensitive exclude matching.
    - Exclude matching now also tests the file basename (filename only), solving cases like `$RECYCLE.BIN`, `Thumbs.db`,
      etc.
- Added `--dry-run` behavior improvements: dry-run now simulates copies and reports counts without touching the
  destination.

### 🧾 Logging & UX

- Files that are skipped (either excluded by a pattern or because destination is newer) are now logged both to stdout
  and to the log file specified with `--log`.
- Logs include the exclude pattern that caused the skip (when applicable), making it easier to diagnose why files were
  skipped.
- Timestamps in logs are configurable and the `LogContext` carries the timestamp format.

### 🔧 Refactor & robustness

- `Config::load_or_default()` added to provide a safe fallback when the configuration file is missing or malformed.
- `create_logger()` now returns a `Result<Option<Logger>>` instead of panicking on errors; logging file creation
  failures is handled gracefully.
- `copy_incremental()` now streams `WalkDir` results (no longer builds an in-memory Vec of all files), reducing memory
  usage for large trees.
- Centralized and simplified CLI parsing via a shared `cli` module (`src/cli.rs`) so the binary and tests reuse the same
  definitions.
- Split test code out of `src/` into `tests/` integration tests and added tests for `is_newer`, exclude handling, and
  dry-run behavior.
- Improved handling of mutex poisoning when writing to the log file; flusher calls added to ensure buffered logs are
  persisted.

### ✅ Fixes

- Fixed cases where some exclude patterns (basename patterns like `$RECYCLE.BIN`) were not matched.
- Fixed logging so skipped files are included in the log file output.

### 🧪 Tests

- Added integration tests under `tests/` to exercise exclude matching, `is_newer` logic and dry-run semantics.

---

## [v0.3.1] - 2025-09-12

### 🚀 Improvements

- Cleaning the screen in case the copy message took more than one row in output
- Changed the output message when copying to correct punctuation

### 🔧 Code Cleanup

- Deleted delay between one copy and the next one
- Deleted unused create imports

---

## [v0.3.0] - 2025-09-10

### 🚀 Features

- Added absolute path to name of copying file
- Added message `Skipped.` or `Copied.` at the end of each file name

### 🐛 Bug Fixes

- Fixed bug that prevented skipping of already existing files in destination directory
- Fixed the process of copy/skip file graphically

### 🔧 Code Cleanup

- Deleted parameter `-g` (no longer necessary)

### 📦 Miscellaneous

- Modified format of starting and ending backup messages
- Commented out unused functions

---

## [0.2.8] – 2025-07-23

### ✅ Fixed

- **Fixes Issue #1**: ["This program must be run as administrator"](https://github.com/umpire274/rBackup/issues/1) – now
  the application runs on **Windows** without requiring elevated privileges.
- **Code cleanup**: removed Windows-specific elevation code (`elevator.rs` was removed).
- **CI fixes**: corrected GitHub Actions workflow for all platforms.
- **Macro derive fix**: added missing `use clap::Parser;` and corrected all `#[arg(...)]` and `#[command(...)]`
  attributes.

### Notes

- No functional changes for macOS and Linux users.
- Verified correct CLI behavior with combined `--graph`, `--log`, and `--timestamp` options.

---

## [0.2.7] – 2025-06-26

### ✨ Changed

- Improved the CLI test progress bar (`--test_ui`) with better layout and live file display during test runs.
- Centralized translation system now applied across all user-facing messages via `translations.json`.

### Notes

- The `--test_ui` flag is still intended for internal testing only.
- No functional changes for end users in normal usage.

---

## [0.2.6] - 2025-06-24

### Added

- Introduced the `-T` / `--test_ui` parameter (hidden from help) to preview progress bar behavior in CLI.

### Changed

- Refactored code for improved structure and readability.
- Adjusted formatting and logic to satisfy `cargo clippy` and CI build checks across:
    - macOS (Intel & Apple Silicon)
    - Ubuntu Linux
    - Windows (MSVC)

### Notes

- This version introduces no functional changes for end users.
- The `--test_ui` option is intended solely for internal development and will not appear in standard help.

---

## [0.2.5] – 2025-06-18

### ✨ Added

- **Multilanguage support**: English and Italian (`--lang`)
- **Progress bar**: `--graph` option to display progress visually
- **Logging**: `--log <file>` option to write output to a log file
- **Quiet mode**: `--quiet` to suppress all console messages
- **Timestamping**: `--timestamp` to prefix messages with date and time
- **Final stats**: number of files copied and skipped
- **Error handling**: log permission denied or locked file errors
- **Localized messages** via `translations.json`
- **Cross-platform support**: Windows, macOS, Linux

### 🛠️ Improvements

- Code split into `main.rs` and `utils.rs`
- Uses `clap`, `indicatif`, `rayon`, `walkdir`, `crossterm`
- Embeds `translations.json` at compile time
- Windows: The elevation to Administrator privileges is now managed dynamically at runtime using the [
  `windows`](https://crates.io/crates/windows) crate, instead of relying on embedded manifest files.
- The request for elevation now occurs **only when required**, after argument validation and outside
  help/version/test-only modes.

### Notes

- This improves portability and avoids UAC prompts when not needed.

---

## [0.1.0] - 2025-06-10

### 🧱 Initial release

- Created initial project `rBackup` for one-way incremental backup
- Inspired by `robocopy` and `rsync`
- Command line arguments for source and destination
- First working build for Windows only

---

🔗 Back to the project: [GitHub - umpire274/rbackup](https://github.com/umpire274/rbackup)
