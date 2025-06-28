# 📋 Changelog

📖 [Back to README](README.md)

All notable changes to the `rbackup` project will be documented in this file.

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
- Windows: The elevation to Administrator privileges is now managed dynamically at runtime using the [`windows`](https://crates.io/crates/windows) crate, instead of relying on embedded manifest files.
- The request for elevation now occurs **only when required**, after argument validation and outside help/version/test-only modes.

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
