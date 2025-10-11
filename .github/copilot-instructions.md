# Copilot instructions for contributors and coding agents (sintesi pratica)

Scopo: fornire a un agente di coding tutte le informazioni essenziali per lavorare con efficienza su questo repository senza dover esplorare l'intero albero dei file a ogni modifica.

Breve piano d'azione quando apri una modifica
- Capire il contesto (leggi README.md e Cargo.toml).
- Eseguire i comandi di validazione locali (format, clippy, test).
- Modificare in piccoli commit atomicI e aggiungere/aggiornare test quando necessario.
- Assicurarsi che la CI sia verde prima di richiedere review.

Comandi fondamentali (esegui da repository root, shell zsh/bash)
```bash
# installa toolchain stabile (se non presente)
rustup toolchain install stable
# assicurati di avere rustfmt e clippy
rustup component add rustfmt
rustup component add clippy
# check formato
cargo fmt --all -- --check
# check clippy (fallisce se ci sono warnings)
cargo clippy --all-targets --all-features -- -D warnings
# esegui i test
cargo test --all
# build di controllo
cargo build
# build di release
cargo build --release
```

Regole imprescindibili prima di aprire una PR
- Sempre eseguire: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, e `cargo test --all`.
- Se cambi API pubbliche o comportamento, aggiungi test (unit/integration) per coprire il caso.
- Mantieni i cambi piccoli e con un solo obiettivo per commit.
- Aggiorna `CHANGELOG.md` e `README.md` solo se la modifica introduce funzionalità visibili all'utente.

Informazioni sul progetto (high level)
- Tipo: CLI tool scritto in Rust (edition = 2024). Progetto multi-piattaforma per backup incrementali.
- Linguaggi/strumenti: Rust + Cargo; dipendenze principali: clap, walkdir, globset, rayon, serde, serde_json.
- Dimensione: codice sorgente concentrato sotto `src/`, test in `tests/`, small helper script in `scripts/translations_tool/`, risorse in `assets/`.

Layout chiave (file e cartelle da conoscere subito)
- Cargo.toml — metadati crate e dipendenze (root).
- README.md, CHANGELOG.md, LICENSE — documentazione e rilascio.
- src/
  - `main.rs` — punto d'entrata CLI.
  - `lib.rs` — code condiviso tra bin e test.
  - `cli.rs`, `commands.rs`, `copy.rs`, `config.rs`, `utils.rs`, `output.rs`, `ui.rs` — moduli principali.
- tests/ — test d'integrazione e controlli automatici.
- scripts/translations_tool/ — utility Rust per gestire `assets/translations.json`.
- .github/workflows/* — pipeline CI: `rust.yml` (lint & test) e `ci.yml` (build/package/release).

Pipeline CI e cosa replicare localmente
- GitHub `rust.yml` (on push/pr) esegue: install toolchain, install rustfmt & clippy, `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all`.
- Per release la pipeline `ci.yml` esegue build cross-target e packaging; replica locale: aggiungere target con `rustup target add <target-triple>` prima di `cargo build --release --target <triple>`.
- Tag e versioning: il release workflow estrae la versione da Cargo.toml; le release su GitHub usano tag `v{version}`.

Casi comuni e trappole già osservate
- Assicurati di quotare pattern/argomenti che contengono caratteri speciali (es. `$RECYCLE.BIN`) quando testi comandi in zsh/bash.
- Componenti platform-specific: attenzione a path e separatori (Windows vs Unix).
- Se aggiungi nuove dipendenze, aggiorna Cargo.toml e verifica che la cache CI non sia invalidata erroneamente.
- Cross-compilazione: la CI esplicita `rustup target add <triple>` prima di `cargo build`.

Consigli su ricerca e modifica del codice
- Parti sempre da `src/main.rs` e `src/lib.rs` per comprendere l'architettura.
- Per funzionalità CLI guarda `src/cli.rs` e `src/commands.rs`.
- Per il processo di copia e filtri controlla `src/copy.rs` e l'uso di `globset`.
- Per output/logging esamina `src/output.rs` e `src/ui.rs`.
- Usa `rg`/`grep` per trovare rapidamente simboli: es. `rg "exclude" src/` o `rg "GlobSet" -S`.

Testing e aumentare la fiducia nelle modifiche
- Aggiungi test unitari in `src/` con `#[cfg(test)]` e test d'integrazione in `tests/`.
- Per scenari CLI puoi esporre funzioni testabili in `lib.rs` invece di lanciare il binario.
- Se un test è fluttuante, isola e riproduci localmente prima di disabilitarlo.

Linee guida per commit/PR
- Message sintetico e descrittivo: prefisso opzionale `feat:`, `fix:`, `chore:`, `docs:`.
- Aggancia al changelog le modifiche che impattano l'utente finale.
- Se la PR cambia comportamento o UX, aggiorna README con esempi.

Quando cercare oltre queste istruzioni
- Fidati di queste istruzioni per attività comuni (build, lint, test, piccole modifiche). Cerca nel repo solo quando:
  - Hai bisogno di capire un dettaglio concreto non descritto qui (es. implementazione specifica di una funzione).
  - CI fallisce in un modo che non è riproducibile localmente.
  - Il cambiamento tocca aree non documentate (es. packaging, cross-compilation complessa).

Contatti e note finali
- CI principale: `.github/workflows/rust.yml` (lint & test).
- Release & packaging: `.github/workflows/ci.yml`.
- Script utile per traduzioni: `scripts/translations_tool/`.

Queste istruzioni devono essere considerate la fonte primaria per un agente di coding che interviene su questo repository; esegui i comandi indicati localmente prima di aprire una PR. Se dovessi trovare discrepanze (es. comandi che falliscono), esegui una ricerca mirata e aggiorna questo file con una nota breve sul problema e la soluzione.

