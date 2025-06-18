# 📋 Changelog

📖 [Torna al README](README.it.md)

Tutte le modifiche rilevanti apportate al progetto `rbackup`.

---

## [0.2.5] – 2025-06-18

### Modifiche
- Windows: La richiesta dei privilegi di amministratore è ora gestita dinamicamente a runtime utilizzando il crate [`windows`](https://crates.io/crates/windows), invece di affidarsi a manifest file incorporati.
- L'elevazione viene richiesta **solo quando necessaria**, dopo la validazione dei parametri e non in modalità help, versione o test.

### Note
- Questo migliora la portabilità e riduce le richieste UAC non necessarie.

---

## [0.2.0] - 2025-06-13

### ✨ Nuove funzionalità
- **Multilingua**: supporto per italiano e inglese (`--lang`)
- **Progress bar**: con opzione `--graph` per mostrare barra grafica
- **Log file**: opzione `--log <file>` per salvare l’output
- **Modalità silenziosa**: opzione `--quiet` per nascondere output a console
- **Timestamp**: opzione `--timestamp` per aggiungere data/ora all’output
- **Contatore finale**: numero di file copiati e file saltati
- **Gestione errori**: log per permessi negati o file bloccati
- **Messaggi localizzati** caricati da `translations.json`
- **Distribuzioni per Windows, macOS e Linux**

### 🛠️ Miglioramenti
- Separazione di `main.rs` e `utils.rs`
- Integrazione di `clap`, `indicatif`, `rayon`, `walkdir`, `crossterm`
- Incorporazione del file `translations.json` in fase di compilazione

---

## [0.1.0] - 2025-06-10

### 🧱 Iniziale
- Creazione progetto `winrsync` per backup incrementale unidirezionale
- Uso di `robocopy` come ispirazione
- Parametri da linea di comando per sorgente e destinazione
- Prima versione funzionante solo per Windows

---

🔗 Torna al progetto: [GitHub - umpire274/rbackup](https://github.com/umpire274/rbackup)
