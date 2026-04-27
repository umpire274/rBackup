<h1 style="text-align: left;">
  <img src="assets/rbackup.svg" width="90" style="vertical-align: middle; margin-right: 8px;" alt="rBackup Logo"/>
  rBackup
</h1>

**rBackup** is a fast, cross-platform, and multithreaded command-line utility written in Rust for performing incremental
backups of directories. It is inspired by tools like `rsync` and `robocopy`, but designed with simplicity, portability,
and localization in mind.

> **Note:** rBackup does not require administrator privileges on any platform.

[![Build Status](https://github.com/umpire274/rBackup/actions/workflows/ci.yml/badge.svg)](https://github.com/umpire274/rBackup/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/umpire274/rBackup)](https://github.com/umpire274/rBackup/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS%20Intel%20%7C%20macOS%20Apple%20Silicon-blue)](https://github.com/umpire274/rBackup/releases)

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
yay -S rbackup
# or
paru -S rbackup
```

### 🍺 Homebrew (macOS/Linux)

[![Homebrew](https://img.shields.io/badge/Homebrew-rBackup-orange.svg?logo=homebrew)](https://github.com/umpire274/homebrew-tap)

```bash
brew tap umpire274/tap
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

> Note: If `rbackup` is executed without any subcommand or positional arguments, the program will print the full help
> message and exit successfully with status code 0. This behavior is intended so invoking the binary with no arguments
> acts as a safe help display rather than an error.

---

## ✅ Basic example

```sh
rbackup copy ~/Documents /mnt/backup_drive/Documents
```

---

## 🧩 Options

### Common Options (per-command)

These options are available on most commands:

| Option              | Description                   |
|---------------------|-------------------------------|
| `-q`, `--quiet`     | Suppress console output       |
| `-t`, `--timestamp` | Prepend timestamp to messages |
| `--log <FILE>`      | Write output to a log file    |

### Global Options

| Option            | Description       |
|-------------------|-------------------|
| `-V`, `--version` | Show version      |
| `-h`, `--help`    | Show help message |

---

## 📌 Commands

Below are the top-level commands with the most relevant options and quick usage notes for each. This layout is easier to
scan than a dense table when commands have many options.

### copy

Description: Perform an incremental backup from a `source` directory to a `destination` directory. Only new or modified
files are copied.

Usage:

```sh
rbackup copy <source> <destination> [OPTIONS]
```

Important options:

- `<source> <destination>` — required positional arguments
- `-d`, `--delta` — copy only changed/new items (delta mode); skipped items are hidden by default
- `-q`, `--quiet` — suppress console output
- `-t`, `--timestamp` — prepend timestamps to messages
- `--log <FILE>` — write output to a log file
- `-x, --exclude <PATTERN>` — exclude files matching the given glob pattern (repeatable)
- `--absolute-exclude` — match exclude patterns against absolute source paths
- `--ignore-case` — perform case-insensitive matching for exclude patterns
- `--dry-run` — perform a dry-run without copying files
- `-j, --jobs <N>` — number of worker threads to use (overrides automatic choice)
- `--show-skipped <never|summary|all>` — control whether skipped items are printed during the run (default: `summary`;
  overrides `--delta` default)

Example:

```sh
rbackup copy C:\source\folder D:\backup\folder --exclude "*.tmp" --dry-run --log dryrun.log
```

---

### config

Description: Manage the configuration file (view, initialize or edit).

Usage:

```sh
rbackup config [OPTIONS]
```

Important options:

- `--init` — initialize a default configuration file
- `--print` — print the current configuration to stdout
- `--edit` — open the configuration in the user's editor
- `--upgrade` — insert new config options into existing `rbackup.conf` when missing
- `--editor <EDITOR>` — specify the editor to use (overrides `$EDITOR`/`$VISUAL`)

Example:

```sh
rbackup config --init
```

---

### help

Usage:

```sh
rbackup help [COMMAND]
```

Description: Print help for a specific command (e.g. `rbackup help copy`).

---

## 🌍 Language Support

rBackup supports multiple languages with automatic system locale detection. The language is configured via the
configuration file (`rbackup.conf`):

- **auto** (default) — automatically detect system locale (falls back to English if not supported)
- **en** — English
- **it** — Italian

To manage the configuration file:

```bash
rbackup config --init     # Create default config
rbackup config --edit     # Edit config with default editor
rbackup config --print    # View current config
```

---

## 🔎 Exclude patterns (`--exclude`)

`rbackup copy` supports flexible exclude patterns to skip files and directories during a backup. The
`--exclude <PATTERN>` option can be used multiple times to provide multiple glob patterns.

Where patterns are matched

- By default, patterns are matched against the path *relative* to the source directory. Example: with source
  `/home/me/project`, pattern `build/**` matches `/home/me/project/build/foo`.
- Use `--absolute-exclude` to match the pattern against the absolute path of the source file instead.
- The matcher also tests the file basename (the filename only). This means a simple pattern like `$RECYCLE.BIN` or
  `Thumbs.db` will match files whose name equals that string anywhere in the source tree.

Case sensitivity

- By default, matching is case-sensitive.
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

> Tip: In `zsh`/`bash` wrap patterns that contain `$` or other special characters in single quotes: `'$RECYCLE.BIN'`.

Absolute vs relative matching

- Relative match (default): `--exclude 'temp/**'` will skip anything under `source/temp/`.
- Absolute match: `--absolute-exclude` with `--exclude '/home/me/project/temp/**'` will match only that absolute path.

Dry-run and logging

- Combine `--dry-run` with `--log` to generate a report of what would be copied or skipped — but without changing the
  destination:

```bash
rbackup copy /source /dest --exclude '*.tmp' --dry-run --log dryrun.log
```

- The log file contains both `Copied` and `Skipped` entries. Skipped entries include the exclude pattern that caused the
  skip when applicable, which helps debugging complex exclude sets. Skipped files are also printed to the console unless
  the `--quiet` option is used.

Use-cases

- Backup only source code files, ignoring build artifacts:

```bash
rbackup copy /home/dev/project /backup/project --exclude 'target/**' --exclude '*.o' --exclude '*.class'
```

- Mirror a user's Documents folder but exclude large media files and the Recycle Bin:

```bash
rbackup copy ~/Documents /mnt/backup/Documents --exclude '*.mp4' --exclude '$RECYCLE.BIN' --exclude 'Thumbs.db' --ignore-case
```

- Debug why a file is skipped: run a dry-run with logging and inspect the log — each skipped line shows the pattern that
  caused the skip.

---

## Use cases (CLI & developer)

This section provides concise, copy-pastable examples for common CLI workflows and for managing translations during
development.

### 1) Incremental backup (copy)

- Basic backup:

```bash
rbackup copy C:\Users\alice\Documents D:\Backups\alice\Documents
```

- Backup with logging, timestamps and quiet off:

```bash
rbackup copy C:\source\folder D:\backup\folder --log backup.log --timestamp
```

- Dry-run to debug excludes and generate a report (Windows `cmd.exe`):

```bat
rbackup copy C:\source D:\dest --exclude "*.tmp" --dry-run --log dryrun.log
```

### 2) Manage configuration

- Initialize a default configuration file:

```bat
rbackup config --init
```

- Edit configuration using the default editor (Windows will use Notepad if $EDITOR not set):

```bat
rbackup config --edit
```

### 3) Developer: translations workflow (translations_tool)

A small helper script is provided at `scripts/translations_tool` to validate translations and to generate/apply language
templates.

- Validate all language entries have the same keys (must be run from the repo root):

```bat
cd scripts\translations_tool
cargo run -- validate
```

- Generate a template for a new language `es` and print it (prefill with English values):

```bat
cd scripts\translations_tool
cargo run -- template es --fill-en
```

- Apply (insert) a template for `es` into `assets/translations.json`, creating a timestamped backup, and do not
  overwrite if `es` already exists:

```bat
cd scripts\translations_tool
cargo run -- apply es --fill-en
```

- Force overwrite an existing language entry (use with caution):

```bat
cd scripts\translations_tool
cargo run -- apply es --fill-en --force
```

Notes:

- The `apply` command creates a backup file under `assets/` named like `translations.json.YYYYMMDD_HHMMSS.bak` before
  modifying `assets/translations.json`.
- After applying a template, translate the values in-place with your editor, then run `cargo run -- validate` to ensure
  key consistency.

---

## 📝 Example

```sh
rbackup copy /home/alex/Projects /mnt/usb-backup --log backup.log --timestamp
```

## 🧪 Build from source

To compile rBackup yourself:

```sh
git clone https://github.com/umpire274/rbackup.git
cd rbackup
cargo build --release
```

The compiled binary will be available in `target/release/`.

### Platform-specific requirements

- **Windows**: The application icon (`assets/rbackup.ico`) is automatically embedded during the build via the `build.rs`
  script.
- **macOS/Linux**: No additional steps needed; build proceeds with standard Rust toolchain.

### Development setup

If you're contributing to the project, ensure you have the required Rust components:

```sh
rustup component add rustfmt
rustup component add clippy
```

Run validation before submitting a PR:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

---

## 🧭 Maintainers: Manual Release Trigger

If you need to manually build and publish a new release (or rebuild an existing one),  
you can trigger the **“Build, Package and Release”** workflow directly from GitHub Actions.

### 🔹 Standard release (safe)

1. Go to **Actions → Build, Package and Release**
2. Click **Run workflow**
3. Leave `force_replace` = `false`
4. Confirm → the workflow will only create the release if it does not already exist.

### 🔹 Force rebuild of an existing release

1. Open the same workflow in **Actions**
2. Click **Run workflow**
3. Set `force_replace` = `true`
4. Confirm → the workflow will delete the existing GitHub release and tag, then recreate them with the latest artifacts.

> 💡 For command-line users: you can also trigger this manually via GitHub CLI:
> ```bash
> gh workflow run "Build, Package and Release" -f force_replace=true
> ```

This ensures you can safely regenerate signed release packages when needed,  
without affecting your normal automated release process.

---

## 🔒 License

This project is licensed under the MIT License.

© 2025 Alessandro Maestri

---

## 💡 Contributing

Pull requests are welcome! If you’d like to add support for more languages, improve performance, or fix bugs, feel free
to fork the repo and contribute.
