// This test ensures that all language objects in assets/translations.json
// contain the same set of keys. It helps catch missing keys when adding
// new translations.

use std::collections::HashSet;

#[test]
fn translations_have_same_keys() {
    let data = include_str!("../assets/translations.json");
    let v: serde_json::Value = serde_json::from_str(data).expect("translations.json must be valid JSON");
    let obj = v.as_object().expect("translations.json must be a JSON object at top-level");

    let mut expected: Option<HashSet<String>> = None;

    for (lang, val) in obj.iter() {
        let map = val.as_object().unwrap_or_else(|| panic!("language '{}' must map to an object", lang));
        let keys: HashSet<String> = map.keys().cloned().collect();
        if let Some(ref e) = expected {
            // compare sets
            let missing: Vec<_> = e.difference(&keys).cloned().collect();
            let extra: Vec<_> = keys.difference(e).cloned().collect();
            assert!(missing.is_empty() && extra.is_empty(),
                "Language '{}' has inconsistent keys. Missing: {:?}, Extra: {:?}", lang, missing, extra);
        } else {
            expected = Some(keys);
        }
    }
}

