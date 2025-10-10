use serde_json::Value;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

fn usage() {
    eprintln!("translations_tool - utility to validate and generate translation templates");
    eprintln!("Usage:");
    eprintln!("  validate                - validate that all languages have the same keys");
    eprintln!("  template <lang> [--fill-en] - print a JSON template for language <lang> to stdout; use --fill-en to copy English text where available");
}

fn load_translations(path: &Path) -> Result<Value, String> {
    let s = fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    let v: Value = serde_json::from_str(&s).map_err(|e| format!("Failed to parse JSON: {}", e))?;
    Ok(v)
}

fn cmd_validate(path: &Path) -> Result<(), String> {
    let v = load_translations(path)?;
    let obj = v
        .as_object()
        .ok_or_else(|| "Top-level translations.json must be an object mapping language codes to objects".to_string())?;

    let mut reference: Option<HashSet<String>> = None;
    let mut all_ok = true;

    for (lang, val) in obj.iter() {
        let map = val
            .as_object()
            .ok_or_else(|| format!("Language '{}' must map to an object", lang))?;
        let keys: HashSet<String> = map.keys().cloned().collect();
        if let Some(ref ref_keys) = reference {
            let missing: Vec<_> = ref_keys.difference(&keys).cloned().collect();
            let extra: Vec<_> = keys.difference(ref_keys).cloned().collect();
            if !missing.is_empty() || !extra.is_empty() {
                all_ok = false;
                eprintln!("Language '{}' has inconsistent keys:", lang);
                if !missing.is_empty() {
                    eprintln!("  Missing: {:?}", missing);
                }
                if !extra.is_empty() {
                    eprintln!("  Extra:   {:?}", extra);
                }
            }
        } else {
            reference = Some(keys);
        }
    }

    if all_ok {
        println!("OK: all languages contain the same keys ({})", reference.map(|s| s.len()).unwrap_or(0));
        Ok(())
    } else {
        Err("Validation failed: inconsistent keys found".to_string())
    }
}

fn cmd_template(path: &Path, lang: &str, fill_en: bool) -> Result<(), String> {
    let v = load_translations(path)?;
    let obj = v
        .as_object()
        .ok_or_else(|| "Top-level translations.json must be an object mapping language codes to objects".to_string())?;

    // Build reference key set from English if present, otherwise first language
    let reference_keys: Vec<String> = if let Some(en) = obj.get("en") {
        en.as_object()
            .ok_or_else(|| "English 'en' entry must be an object".to_string())?
            .keys()
            .cloned()
            .collect()
    } else {
        let first = obj
            .values()
            .next()
            .ok_or_else(|| "No languages found in translations.json".to_string())?;
        first
            .as_object()
            .ok_or_else(|| "Language entry must be an object".to_string())?
            .keys()
            .cloned()
            .collect()
    };

    // If fill_en and en present, capture English values
    let en_map = obj.get("en").and_then(|v| v.as_object());

    let mut out = serde_json::map::Map::new();
    for k in reference_keys.iter() {
        let val = if fill_en {
            if let Some(en) = en_map {
                en.get(k).cloned().unwrap_or(Value::String(String::new()))
            } else {
                Value::String(String::new())
            }
        } else {
            Value::String(String::new())
        };
        out.insert(k.clone(), val);
    }

    let mut root = serde_json::map::Map::new();
    root.insert(lang.to_string(), Value::Object(out));

    let s = serde_json::to_string_pretty(&Value::Object(root)).map_err(|e| format!("Serialize error: {}", e))?;
    println!("{}", s);
    Ok(())
}

fn main() {
    // Expected execution dir: scripts/translations_tool/
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        exit(1);
    }

    let cmd = args[1].as_str();

    // translations.json path, relative to this script dir
    let translations_path = Path::new("../../assets/translations.json");

    match cmd {
        "validate" => match cmd_validate(translations_path) {
            Ok(_) => exit(0),
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(2)
            }
        },
        "template" => {
            if args.len() < 3 {
                eprintln!("template requires a language code (e.g. template es)");
                usage();
                exit(1);
            }
            let lang = &args[2];
            let fill_en = args.iter().any(|a| a == "--fill-en");
            match cmd_template(translations_path, lang, fill_en) {
                Ok(_) => exit(0),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(2)
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", cmd);
            usage();
            exit(1);
        }
    }
}

