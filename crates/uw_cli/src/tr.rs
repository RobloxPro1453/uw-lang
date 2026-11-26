//! Warband's official Turkish texts: `Modules/Native/languages/tr/*.csv`, one `key|text` entry per
//! line, so the tools can show the Turkish text next to a Native thing
//!.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Reads every `*.csv` directly in `dir` into one map, files in file-name order.
/// Each line splits at the FIRST `|`; a UTF-8 BOM and a trailing `\r` are dropped, empty lines and
/// lines without `|` are skipped, and a key that repeats keeps its first value.
pub fn load_dir(dir: &Path) -> std::io::Result<HashMap<String, String>> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_file() && p.extension().is_some_and(|x| x == "csv"))
        .collect();
    paths.sort();
    let mut map = HashMap::new();
    for path in paths {
        let text = String::from_utf8_lossy(&fs::read(&path)?).into_owned();
        for raw in text.split('\n') {
            let line = raw.strip_prefix('\u{feff}').unwrap_or(raw);
            let line = line.strip_suffix('\r').unwrap_or(line);
            if line.is_empty() {
                continue;
            }
            let Some((key, value)) = line.split_once('|') else { continue };
            map.entry(key.to_string()).or_insert_with(|| value.to_string());
        }
    }
    Ok(map)
}

/// `<Warband>/Modules/Native/languages/tr` in the default Steam library, if present.
pub fn default_dir() -> Option<PathBuf> {
    let path = PathBuf::from(
        r"C:\Program Files (x86)\Steam\steamapps\common\MountBlade Warband\Modules\Native\languages\tr",
    );
    path.is_dir().then_some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("uw-tr-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn load_dir_reads_both_files_in_name_order() {
        let root = temp("load");
        // BOM on the first line, CRLF endings, a text with `|`, an empty line, a line without `|`.
        fs::write(
            root.join("a_dialogs.csv"),
            "\u{feff}dlga_start:one|Merhaba|dünya\r\ndlga_start:two|İki\r\n\r\nno pipe here\r\n",
        )
        .unwrap();
        // `dlga_start:two` repeats here: the first file's value must win.
        fs::write(
            root.join("b_menus.csv"),
            "menu_start_game_0|Oyuna başla|şimdi\r\ndlga_start:two|ikinci kez\r\nmno_start_male|Erkek\r\n",
        )
        .unwrap();

        let map = load_dir(&root).unwrap();
        let expected: HashMap<String, String> = [
            ("dlga_start:one", "Merhaba|dünya"),
            ("dlga_start:two", "İki"),
            ("menu_start_game_0", "Oyuna başla|şimdi"),
            ("mno_start_male", "Erkek"),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
        assert_eq!(map, expected);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn default_dir_ends_with_tr() {
        if let Some(dir) = default_dir() {
            assert!(dir.is_dir(), "{}", dir.display());
            assert_eq!(dir.file_name().and_then(|name| name.to_str()), Some("tr"));
        }
    }
}
