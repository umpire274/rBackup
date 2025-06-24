# 📋 Changelog

📖 [Torna al README](README.it.md)

Tutte le modifiche rilevanti apportate al progetto `rbackup`.

---

## [0.2.6] - 2025-06-24

### Aggiunto
- Introdotto il parametro `-T` / `--test_ui` (nascosto dalla guida) per testare il comportamento della barra di avanzamento nel terminale.

### Modificato
- Rifattorizzazione del codice per migliorarne la struttura e la leggibilità.
- Riformattazione e adeguamento del codice per superare con successo i controlli di compilazione (`cargo clippy`) e i test automatici CI per le seguenti piattaforme:
  - macOS (Intel e Apple Silicon)
  - Ubuntu Linux
  - Windows (MSVC)

### Note
- Questa versione non introduce modifiche funzionali per gli utenti finali.
- L'opzione `--test_ui` è pensata solo per lo sviluppo interno e non compare nella guida standard.

---

## [0.2.5] – 2025-06-18

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
- Windows: La richiesta dei privilegi di amministratore è ora gestita dinamicamente a runtime utilizzando il crate [`windows`](https://crates.io/crates/windows), invece di affidarsi a manifest file incorporati.
- L'elevazione viene richiesta **solo quando necessaria**, dopo la validazione dei parametri e non in modalità help, versione o test.

### Note
- Questo migliora la portabilità e riduce le richieste UAC non necessarie.

---

## [0.1.0] - 2025-06-10

### 🧱 Iniziale
- Creazione progetto `rBackup` per backup incrementale unidirezionale
- Uso di `robocopy` come ispirazione
- Parametri da linea di comando per sorgente e destinazione
- Prima versione funzionante solo per Windows

---

🔗 Torna al progetto: [GitHub - umpire274/rbackup](https://github.com/umpire274/rbackup)
