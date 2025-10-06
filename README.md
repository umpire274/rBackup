# rbackup

**rbackup** is a fast, cross-platform, and multithreaded command-line utility written in Rust for performing incremental backups of directories. It is inspired by tools like `rsync` and `robocopy`, but designed with simplicity, portability, and localization in mind.

![CI](https://github.com/umpire274/rbackup/actions/workflows/ci.yml/badge.svg)
[![Licenza MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS%20Intel%20%7C%20macOS%20Apple%20Silicon-blue)](https://github.com/umpire274/rBackup/releases)
[![Versione](https://img.shields.io/badge/version-0.5.1-orange)](https://github.com/umpire274/rbackup/releases)

📋 [View recent changes (Changelog)](CHANGELOG.md)

---

## ✨ Features

- 🚀 **Incremental backup** – copies only new or modified files
- ⚡ **Multithreaded** – uses all CPU cores to speed up large backups
- 🌍 **Multilingual support** – English and Italian (with auto-detection)
- 📦 **Portable** – no installation required, single binary
- 🧾 **Optional logging** – write backup reports to a file
- 📊 **Progress bar** – display graphical progress bar during copy process
- 🤫 **Quiet mode** – suppress all output for silent operation

---

## 📦 Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/rbackup.svg)](https://repology.org/project/rbackup/versions)

### 🐧 AUR (Arch Linux)

[![AUR](https://img.shields.io/aur/version/rbackup)](https://aur.archlinux.org/packages/rbackup)

```bash
yay -S rfortune
# or
paru -S rfortune
```

### 🍺 Homebrew (macOS/Linux)

[![Homebrew Tap](https://img.shields.io/badge/homebrew-tap-brightgreen)](https://github.com/umpire274/homebrew-rbackup)

```bash
brew tap umpire274/rbackup
brew install rbackup
```

### 🦀 Crates.io (Rust)

[![Crates.io](https://img.shields.io/crates/v/rbackup)](https://crates.io/crates/rbackup)

```bash
cargo install rbackup
```

---

## 🚀 Usage

```sh
rbackup <source> <destination> [OPTIONS]
```

---

## ✅ Basic example

```sh
rbackup ~/Documents /mnt/backup_drive/Documents
```

---

## 🧩 Options

| Option                | Description                      |
| --------------------- | -------------------------------- |
| `-q`, `--quiet`       | Suppress console output          |
| `-t`, `--timestamp`   | Prepend timestamp to messages    |
| `--log <FILE>`        | Write output to a log file       |
| `-l`, `--lang <code>` | Force language (e.g. `en`, `it`) |
| `-V`, `--version`     | Show version                     |
| `-h`, `--help`        | Show help message                |


---

## 🔎 Exclude patterns (`--exclude`)

`rbackup copy` supports flexible exclude patterns to skip files and directories during a backup. The `--exclude <PATTERN>` option can be used multiple times to provide multiple glob patterns.

Where patterns are matched

- By default patterns are matched against the path *relative* to the source directory. Example: with source `/home/me/project`, pattern `build/**` matches `/home/me/project/build/foo`.
- Use `--absolute-exclude` to match the pattern against the absolute path of the source file instead.
- The matcher also tests the file basename (the filename only). This means a simple pattern like `$RECYCLE.BIN` or `Thumbs.db` will match files whose name equals that string anywhere in the source tree.

Case sensitivity

- By default matching is case-sensitive.
- Use `--ignore-case` to enable case-insensitive matching for exclude patterns.

Examples

- Exclude macOS DMG files and Thumbs.db files (case-insensitive):

```bash
rbackup copy /source /dest --log backup.log --exclude '*.dmg' --exclude 'Thumbs.db' --ignore-case
```

- Exclude the Windows Recycle Bin directory by basename and hidden files starting with a dot:

```bash
rbackup copy /source /dest --exclude '$RECYCLE.BIN' --exclude '.*'
```

> Tip: In `zsh`/`bash` wrap patterns that contain `$` or other special characters in single quotes: `'\$RECYCLE.BIN'` or better `'$RECYCLE.BIN'`.

Absolute vs relative matching

- Relative match (default): `--exclude 'temp/**'` will skip anything under `source/temp/`.
- Absolute match: `--absolute-exclude` with `--exclude '/home/me/project/temp/**'` will match only that absolute path.

Dry-run and logging

- Combine `--dry-run` with `--log` to generate a report of what would be copied or skipped — but without changing the destination:

```bash
rbackup copy /source /dest --exclude '*.tmp' --dry-run --log dryrun.log
```

- The log file contains both `Copied` and `Skipped` entries. Skipped entries include the exclude pattern that caused the skip when applicable, which helps debugging complex exclude sets.

Use-cases

- Backup only source code files, ignoring build artifacts:

```bash
rbackup copy /home/dev/project /backup/project --exclude 'target/**' --exclude '*.o' --exclude '*.class'
```

- Mirror a user's Documents folder but exclude large media files and the Recycle Bin:

```bash
rbackup copy ~/Documents /mnt/backup/Documents --exclude '*.mp4' --exclude '$RECYCLE.BIN' --exclude 'Thumbs.db' --ignore-case
```

- Debug why a file is skipped: run a dry-run with logging and inspect the log — each skipped line shows the pattern that caused the skip.


---

## 📝 Example

```sh
rbackup /home/alex/Projects /mnt/usb-backup --log backup.log --timestamp
```

## 🧪 Build from source

To compile rbackup yourself:

```sh
git clone https://github.com/your-username/rbackup.git
cd rbackup
cargo build --release
```

For Windows, rbackup.rc will be embedded automatically in the executable.

---

## 🔒 License

This project is licensed under the MIT License.

© 2025 Alessandro Maestri

---

## 💡 Contributing

Pull requests are welcome! If you’d like to add support for more languages, improve performance, or fix bugs, feel free to fork the repo and contribute.
