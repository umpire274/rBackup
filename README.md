# rbackup

**rbackup** is a fast, cross-platform, and multithreaded command-line utility written in Rust for performing incremental backups of directories. It is inspired by tools like `rsync` and `robocopy`, but designed with simplicity, portability, and localization in mind.

![CI](https://github.com/umpire274/rbackup/actions/workflows/ci.yml/badge.svg)
[![Licenza MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS%20%7C%20FreeBSD-blue)](https://github.com/umpire274/rBackup/releases)
[![Versione](https://img.shields.io/badge/version-0.2.6-orange)](https://github.com/umpire274/rbackup/releases/tag/v0.2.6)

📋 [View recent changes (Changelog)](CHANGELOG.md)

---

## ✨ Features

- 🚀 **Incremental backup** – copies only new or modified files
- ⚡ **Multithreaded** – uses all CPU cores to speed up large backups
- 🌍 **Multilingual support** – English and Italian (with auto-detection)
- 📦 **Portable** – no installation required, single binary
- 🧾 **Optional logging** – write backup reports to a file
- 📊 **Progress bar** – optionally display graphical progress bar
- 🤫 **Quiet mode** – suppress all output for silent operation

---

## 📥 Download

Precompiled binaries are available in the [Releases](https://github.com/umpire274/rbackup/releases) section.

| Platform | Architecture | File |
|----------|--------------|------|
| Windows  | x86_64       | `rbackup-<version>-x86_64-pc-windows-msvc.zip` |
| Linux    | x86_64       | `rbackup-<version>-unknown-linux-gnu.tar.gz` |
| macOS    | x86_64       | `rbackup-<version>-x86_64-apple-darwin.tar.gz` |
| macOS    | aarch64      | `rbackup-<version>-aarch64-apple-darwin.tar.gz` |
| FreeBSD  | x86_64       | coming soon |

---

## 🔐 GPG Signature

All release archives are cryptographically signed with GPG.

- `.sig` files contain the ASCII-armored detached signature for the corresponding archive.
- You can verify the archive with:

```bash
gpg --verify rbackup-<version>-<target>.tar.gz.sig rbackup-<version>-<target>.tar.gz
```

---

## 🔑 Public Key

The releases are signed with the following GPG key:

* Key ID: 423FABCE0A1921FB
* Fingerprint: 8118 9716 9512 2A32 1F3D C04C 423F ABCE 0A19 21FB
* Download: https://github.com/umpire274.gpg

To import the key from a keyserver:

```sh
gpg --recv-keys 423FABCE0A1921FB
```

Or from OpenPGP server:

```sh
gpg --keyserver keys.openpgp.org --recv-keys 423FABCE0A1921FB
```

Then verify the fingerprint:

```sh
gpg --fingerprint 423FABCE0A1921FB
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
| `-g`, `--graph`       | Show graphical progress bar      |
| `-q`, `--quiet`       | Suppress console output          |
| `-t`, `--timestamp`   | Prepend timestamp to messages    |
| `--log <FILE>`        | Write output to a log file       |
| `-l`, `--lang <code>` | Force language (e.g. `en`, `it`) |
| `-V`, `--version`     | Show version                     |
| `-h`, `--help`        | Show help message                |

> With `--lang auto` (default), the language is automatically detected from the operating system.

---

## 📝 Example

```sh
rbackup /home/alex/Projects /mnt/usb-backup -g --log backup.log --timestamp
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
