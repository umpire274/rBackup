# 📋 Changelog

📖 [Back to README](README.md)

All notable changes to the `rbackup` project will be documented in this file.

---

## [0.2.0] - 2025-06-13

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

---

## [0.1.0] - 2025-06-10

### 🧱 Initial release
- Created initial project `winrsync` for one-way incremental backup
- Inspired by `robocopy` and `rsync`
- Command line arguments for source and destination
- First working build for Windows only

---

🔗 Back to the project: [GitHub - umpire274/rbackup](https://github.com/umpire274/rbackup)
