# rBackup translations (translations.json)

This document describes the structure and expected keys inside `assets/translations.json` and explains how to add new
languages.

## File overview

`assets/translations.json` is a JSON object mapping language codes (for example `"en"`, `"it"`) to dictionaries of
localized strings used by rBackup. Each language entry must contain the same set of keys so the program can look up
localized messages without runtime errors.

Example top-level shape:

{
"en": { ... },
"it": { ... }
}

## Expected keys

Each language object should contain the following string keys (all values must be strings):

- `appname` — Application name displayed in some contexts (e.g. `rBackup`).
- `cur_conf` — Short label for the current configuration (example: "Current configuration").
- `conf_file_not_found` — Message shown when the configuration file cannot be found.
- `conf_initialized` — Message shown after creating a default configuration file (often followed by the path).
- `backup_init` — Short header printed when a backup begins (for example: `=== Backup started ===`).
- `backup_ended` — Short header printed when a backup ends (for example: `=== Backup ended ===`).
- `starting_backup` — Label before source path (for example: "Backup from:").
- `to` — Label before destination path (for example: "to:").
- `copying_file` — Label used when printing the current file being copied (for example: "Copying:").
- `invalid_source` — Error shown when the provided source path is not a valid directory.
- `language_not_supported` — Message printed when the detected language code is not present in the translations file.
  This string may include a single placeholder `{}` where the unsupported language code is substituted. Example:
  `Language "{}" not supported, falling back to "en".`
- `files_total` — Format string showing the total number of files. Contains one `{}` placeholder for the number.
  Example: `Total files: {}`.
- `files_copied` �� Format string showing the number of files copied. Contains one `{}` placeholder.
- `files_skipped` — Format string showing the number of skipped/failed files. Contains one `{}` placeholder.
- `copy_progress` — Short label used as the progress bar title (for example: "Copy progress:").
- `copied_file` — Short status string used when a file was copied successfully (for example: "Copied.").
- `skipped_file` — Short status string used when a file is skipped (for example: "Skipped.").
- `generic_error` — Generic error prefix used when a copy operation fails (for example: "Error during copy").
- `error_exclude_parsing` — Error message shown when exclude pattern parsing fails.

Notes about placeholders: keys that include `{}` are format placeholders and are substituted at runtime by the
application using simple string replacement. The application expects exactly one `{}` where it substitutes values (for
the keys listed above that contain placeholders). Avoid other brace syntax or numbered placeholders — keep `"{}"` as the
substitution marker.

## How rBackup chooses language at runtime

- The configuration (`Config`) contains a `language` field. If its value is `"auto"`, rBackup will attempt to detect the
  system locale (for example `en_US`, `it_IT`) and use the language part (the two-letter code before `_` or `-`,
  lowercased) to select the translation.
- If the configured language is a specific code (for example `"en"` or `"it"`), rBackup will use that language directly.
- If the selected language code is not present in `translations.json`, rBackup prints the message
  `language_not_supported` from the English (`en`) bundle and falls back to `en`.

## Adding a new language

To add support for a new language (for example Spanish `es`), follow these steps:

1. Create a new object keyed by the language code at the top-level of `assets/translations.json` (use the same keys
   listed in this document).
2. Provide translations for every expected key. Keep the keys identical; missing keys can cause runtime fallbacks or
   unexpected text usage.
3. Make sure any keys that include `{}` placeholders keep exactly the placeholder token `"{}"` where required.
4. Save the file using UTF-8 encoding. Avoid BOM markers.
5. Optionally, run the unit/doctests to verify compilation and doctests (see testing section).

### Example: adding Spanish (`es`)

Add an entry like this to `assets/translations.json` (merge it at top-level with existing languages):

```json
{
  "es": {
    "appname": "rBackup",git add -A
    git commit -m "docs(translations): add QA checklist to TRANSLATIONS.md and update README/CHANGELOG use-cases" -m "Append QA checklist for translators to assets/TRANSLATIONS.md. The checklist covers placeholders, encoding, UI length considerations, validation commands and PR checklist. Also updated README/CHANGELOG with use-case references. Docs only; verified validate script."git add -A
    git commit -m "docs(translations): add QA checklist to TRANSLATIONS.md and update README/CHANGELOG use-cases" -m "Append QA checklist for translators to assets/TRANSLATIONS.md. The checklist covers placeholders, encoding, UI length considerations, validation commands and PR checklist. Also updated README/CHANGELOG with use-case references. Docs only; verified validate script."
    "cur_conf": "Configuración actual",
    "conf_file_not_found": "Archivo de configuración no encontrado",
    "conf_initialized": "Configuración inicializada en",
    "backup_init": "=== Copia iniciada ===",
    "backup_ended": "=== Copia terminada ===",
    "starting_backup": "Copia de:",
    "to": "a:",
    "copying_file": "Copiando:",
    "invalid_source": "Error: la fuente no es un directorio válido.",
    "language_not_supported": "Idioma \"{}\" no soportado, usando \"en\" como fallback.",
    "files_total": "Archivos totales: {}",
    "files_copied": "Copiados: {}",
    "files_skipped": "Omitidos o fallidos: {}",
    "copy_progress": "Progreso de copia:",
    "copied_file": "Copiado.",
    "skipped_file": "Omitido.",
    "generic_error": "Error durante la copia",
    "error_exclude_parsing": "Error al analizar los patrones de exclusión"
  }
}
```

Insert the object at the top-level (ensure the whole `translations.json` remains valid JSON). If you use a JSON editor,
verify syntax and commas between top-level language objects.

## Tips and validation

- Keep translations consistent: the application expects all keys to be present. If you add a language with missing keys,
  the code may still run but some messages will be missing or fallback to the English key behavior.
- Preserve placeholders: where the English string contains `"{}"`, keep that exact substring in the translation; the
  application replaces it using simple string replacement.
- Use short phrases for status lines: some messages are used in terminal UI and should be reasonably short to avoid
  wrapping (e.g. `copied_file`, `skipped_file`).
- Encoding: save `translations.json` as UTF-8. Non-UTF-8 encodings may cause parsing failures at runtime.
- Testing locally: after editing `translations.json`, run the test suite to verify doctests and any examples that rely
  on translations:

```bash
# from repository root
cargo check
cargo test
```

- If you want to preview text rendering in the app, run the binary with a sample command and the configuration
  `language` set to the new language code or let `language: auto` detect a matching system locale.

## Contribution guidelines (for PRs)

- Add the new language entry to `assets/translations.json`.
- Prefer a single commit that adds the language and optionally a short note in `README.md` or `CHANGELOG.md` explaining
  the addition.
- Run `cargo test` locally and include a short description of testing steps in your PR.

## Advanced: splitting translations

If the file grows large, you may consider splitting translations into separate files (for example
`assets/translations/en.json`, `assets/translations/it.json`) and updating the build logic to merge them at compile
time. Currently the project expects a single `assets/translations.json` file embedded at compile time, so any
reorganization requires code changes.

## QA checklist for translators

Use this checklist to validate translations before opening a pull request. The items below are quick, pragmatic checks
that catch the most common issues.

1. Placeholders and formatting
    - Ensure every placeholder in the English source uses the exact token `{}` and that the translated string contains
      the same number of `{}` tokens in the proper positions.
    - Do not change or add numbered placeholders like `{0}` — the code expects simple `{}` tokens.
    - Preserve punctuation that the program relies on (for example periods at the end of short status messages:
      `Copied.` / `Skipped.`).

2. Encoding and escapes
    - Save the file as UTF-8 without BOM.
    - Escape double quotes inside JSON strings as `\"` and backslashes as `\\`.
    - If a translation contains JSON-sensitive characters (newlines, tabs), prefer to keep them simple; avoid embedding
      raw newlines in a single translation value.

3. String length and UI considerations
    - Keep status strings short (single words or short phrases) when they are used in progress UIs (e.g. `copied_file`,
      `skipped_file`, `copy_progress`).
    - Avoid overly long sentences for titles and single-line messages to prevent terminal wrapping.

4. Consistency and tone
    - Match the tone and formality used in the other translations (e.g. use informal vs formal consistently across the
      file).
    - Keep terminology consistent (e.g. choose one translation for `backup`, `copy`, `log` and reuse it).

5. Placeholders with surrounding text
    - When the English message is something like `Language "{}" not supported, falling back to "en".`, keep the `{}`
      position and surrounding quotes exactly as in the original, translating only the surrounding words.

6. Validation commands (quick, local)
    - After editing `assets/translations.json`, run the translations consistency check:

    ```bat
    cd scripts\translations_tool
    cargo run -- validate
    ```

    - Optionally generate a template to compare keys:

    ```bat
    cd scripts\translations_tool
    cargo run -- template es --fill-en
    ```

    - If you used `apply`, a backup file is created under `assets/` before changes. Verify the change and run the
      validate
      step above.

7. Review before PR
    - Run a JSON linter or formatter (e.g. `jq . assets/translations.json` or your editor's JSON formatter) to ensure
      valid syntax.
    - Verify there are no trailing commas or missing commas between top-level language objects.
    - Confirm the file remains UTF-8 encoded.

8. PR checklist (what to include in the PR description)
    - Language code added (e.g. `es`).
    - Short note: whether you used `--fill-en` and whether strings were edited manually afterwards.
    - Confirmation that `cargo run -- validate` passed locally.
    - Optional: mention any strings you intentionally left in English (for clarification).

---

If you want, I can also add an automated pre-commit hook (a small script) that runs the `validate` command and checks
JSON formatting before allowing a commit. Would you like me to add that as well?
