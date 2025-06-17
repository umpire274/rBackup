# rbackup

🇬🇧 [Read in English](README.md)

**rbackup** è un'utilità da riga di comando scritta in Rust, veloce, multipiattaforma e multithread, progettata per eseguire backup incrementali di directory. Si ispira a strumenti come `rsync` e `robocopy`, ma con un focus sulla semplicità, portabilità e localizzazione.

![CI](https://github.com/umpire274/rbackup/actions/workflows/ci.yml/badge.svg)
[![Licenza MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
![Piattaforme](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-blue)
[![Versione](https://img.shields.io/badge/version-0.2.0-orange)](https://github.com/umpire274/rbackup/releases/tag/v0.2.0)

📋 [Visualizza le modifiche recenti (Changelog)](CHANGELOG.it.md)


---

## ✨ Caratteristiche principali

- 🚀 **Backup incrementale** – copia solo i file nuovi o modificati
- ⚡ **Multithread** – sfrutta tutti i core della CPU per backup rapidi
- 🌍 **Supporto multilingua** – Italiano e Inglese (con rilevamento automatico)
- 📦 **Portatile** – nessuna installazione, singolo eseguibile
- 🧾 **Log opzionale** – registra l’output su file
- 📊 **Barra di avanzamento** – grafica opzionale per il progresso
- 🤫 **Modalità silenziosa** – nessun messaggio a video

---

## 📥 Download

I binari precompilati sono disponibili nella sezione [Releases](https://github.com/umpire274/rbackup/releases).

| Piattaforma | Architettura | File |
|-------------|--------------|------|
| Windows     | x86_64       | `rbackup-windows-x86_64-v0.2.0.zip` |
| Linux       | x86_64       | `rbackup-linux-x86_64-v0.2.0.tar.gz` |
| macOS       | x86_64       | `rbackup-macos-x86_64-v0.2.0.tar.gz` |


---

## 🚀 Utilizzo

```bash
rbackup <sorgente> <destinazione> [OPZIONI]
```

---

## ✅ Esempio base

```sh
rbackup ~/Documenti /mnt/backup_usb/Documenti
```

---

## 🧩 Opzioni disponibili

| Opzione                 | Descrizione                            |
| ----------------------- | -------------------------------------- |
| `-g`, `--graph`         | Mostra la barra di avanzamento grafica |
| `-q`, `--quiet`         | Sopprime l’output a schermo            |
| `-t`, `--timestamp`     | Aggiunge timestamp ai messaggi         |
| `--log <FILE>`          | Registra l’output su file              |
| `-l`, `--lang <codice>` | Forza la lingua (es. `it`, `en`)       |
| `-V`, `--version`       | Mostra la versione                     |
| `-h`, `--help`          | Mostra la guida                        |

> Con `--lang auto` (default), la lingua viene rilevata automaticamente dal sistema operativo.

---

## 📝 Esempio completo

```sh
rbackup /home/alex/Progetti /mnt/usb-backup -g --log backup.log --timestamp
```

## 🧪 Compilazione da sorgente

Per compilare rbackup dal codice sorgente:

```sh
git clone https://github.com/tuo-utente/rbackup.git
cd rbackup
cargo build --release
```

Su Windows, il file rbackup.rc sarà incorporato automaticamente.

---

## 🔒 Licenza

Questo progetto è distribuito con licenza MIT.

© 2025 Alessandro Maestri

---

## 💡 Contribuire

Le pull request sono benvenute! Se vuoi aggiungere nuove lingue, migliorare le prestazioni o correggere problemi, sentiti libero di contribuire al progetto.
