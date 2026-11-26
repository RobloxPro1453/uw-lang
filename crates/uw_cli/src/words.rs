//! `uw words [folder] [--json]`: every name a `.uw` file may use — the project's game names, the
//! `fn` words of `lib/*.uw` and `fns/*.uw`, the object fields (`player.gold`) those fns define and
//! the language keywords — one line each, for the VS Code completion (T-104).
//!
//! The names are not copied here: game names come from `names.uw` (or the `uw new` rule), words
//! from the same `lib`/`fns` parse `uw build-project` uses, and the hints from the atlas entries.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use uw_atlas::Atlas;

use crate::model::{entry_hint, kind_of};

const USAGE: &str = "usage:
  uw words [folder] [--json] [--atlas <atlas.json>]
      list every name a .uw file may use, one per line;
      --json: {\"label\":\"praven\",\"kind\":\"name|word|field|keyword\",\"detail\":\"…\"}";

/// The language keywords (`uw_syntax` and the TextMate grammar) with a one-line Turkish hint.
const KEYWORDS: &[(&str, &str)] = &[
    ("mod", "modun adını yazar"),
    ("override", "var olan bir kaydı değiştirir"),
    ("new", "yeni bir kayıt (örneğin menü) tanımlar"),
    ("troop", "asker kaydı; `like` ile Native askerini kopyalar"),
    ("town", "şehir kaydı; `like` ile Native şehrini kopyalar"),
    ("castle", "kale kaydı; `like` ile Native kalesini kopyalar"),
    ("village", "köy kaydı; `like` ile Native köyünü kopyalar"),
    ("party", "parti kaydı; `like` ile Native partisini kopyalar"),
    ("like", "kopyalanacak Native kaydını gösterir"),
    ("on", "bir olay işleyicisi açar (`on game.start`)"),
    ("fn", "kendi komutunuzu tanımlar"),
    ("save", "kayıtlı oyunda yaşayan bir `$global` tanımlar"),
    ("every", "her N saatte çalışan basit tetikleyici yazar"),
    ("if", "koşullu blok"),
    ("else", "koşulun öbür kolu"),
    ("for", "bir aralıkta/listede döngü kurar"),
    ("in", "`for` döngüsünde aralığı/listeyi verir"),
    ("require", "koşul sağlanmazsa bloğu durdurur"),
    ("attempt", "kolları sırayla dener (try/else_try)"),
    ("option", "menüye bir seçenek ekler"),
    ("when", "seçeneğin ne zaman görüneceği"),
    ("after", "seçeneği başka bir seçenekten sonra yerleştirir"),
    ("before", "seçeneği başka bir seçenekten önce yerleştirir"),
    ("text", "menü metni satırı"),
    ("change", "var olan bir seçeneği değiştirir"),
    ("remove", "var olan bir seçeneği kaldırır"),
    ("do", "menü seçeneğinin yapılacak işi"),
];

/// The verbs an object word (`player.gold` -> `fn player.gold.<verb>`) may stand for, as
/// `uw_check` reads them (T-082).
const VERBS: [&str; 4] = ["get", "add", "remove", "set"];

/// One completion word: what is written, what it is and a one-line hint.
struct Word {
    label: String,
    kind: &'static str,
    detail: String,
}

pub fn cmd(args: &[String]) -> ExitCode {
    let mut json = false;
    let mut atlas = None;
    let mut dir = None;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--json" => json = true,
            "--atlas" => match it.next() {
                Some(path) => atlas = Some(PathBuf::from(path)),
                None => return crate::fail("--atlas needs a path"),
            },
            _ if dir.is_none() => dir = Some(PathBuf::from(arg)),
            _ => return crate::fail(&format!("unexpected argument '{arg}'\n{USAGE}")),
        }
    }
    let dir = dir.unwrap_or_else(|| PathBuf::from("."));
    let dir = match crate::enter_home(&dir) {
        Ok(dir) => dir,
        Err(message) => return crate::fail(&message),
    };
    let dir = dir.canonicalize().unwrap_or(dir);
    let words = match collect(&dir, atlas) {
        Ok(words) => words,
        Err(message) => return crate::fail(&message),
    };
    for word in &words {
        if json {
            println!(
                "{}",
                serde_json::json!({
                    "label": &word.label,
                    "kind": word.kind,
                    "detail": &word.detail,
                })
            );
        } else {
            println!("{:<28} {:<8} {}", word.label, word.kind, word.detail);
        }
    }
    ExitCode::SUCCESS
}

/// Every word of `dir`, in that order: game names, `fn` words, object fields, keywords.
fn collect(dir: &Path, atlas: Option<PathBuf>) -> Result<Vec<Word>, String> {
    let path = atlas
        .or_else(|| std::env::var_os("UW_ATLAS").map(PathBuf::from))
        .or_else(|| crate::build::find_up(dir, crate::ATLAS_REL))
        .ok_or_else(|| {
            format!(
                "no atlas found ({ATLAS_REL} in this or a parent folder); \
                 build it as described in docs/DEVELOPMENT.md (Atlas)",
                ATLAS_REL = crate::ATLAS_REL
            )
        })?;
    let atlas = Atlas::from_path(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    let mut words = Vec::new();
    let mut seen = BTreeSet::new();

    for name in game_names(&atlas, dir)? {
        let detail = match atlas.entry_by_id(&name.reference) {
            Some((table, entry)) => match entry_hint(table, entry) {
                Some(hint) => format!("oyun adı: {hint} ({})", kind_of(table)),
                None => format!("oyun adı: {}", kind_of(table)),
            },
            None => format!("oyun adı: {}", kind_of(&name.table)),
        };
        push(&mut words, &mut seen, Word { label: name.name, kind: "name", detail });
    }

    for name in fn_names(dir)? {
        push(
            &mut words,
            &mut seen,
            Word { label: name.clone(), kind: "word", detail: "kendi `fn` kelimeniz".into() },
        );
        if let Some(field) = object_field(&name) {
            push(
                &mut words,
                &mut seen,
                Word {
                    label: field.to_string(),
                    kind: "field",
                    detail: "nesne alanı (.get/.add/.remove/.set)".into(),
                },
            );
        }
    }

    for (label, detail) in KEYWORDS {
        push(&mut words, &mut seen, Word { label: (*label).into(), kind: "keyword", detail: (*detail).into() });
    }
    Ok(words)
}

/// Adds `word` when its (kind, label) was not seen yet.
fn push(words: &mut Vec<Word>, seen: &mut BTreeSet<(String, String)>, word: Word) {
    if seen.insert((word.kind.to_string(), word.label.clone())) {
        words.push(word);
    }
}

/// The project's game names: its `names.uw` when it has one, otherwise the ones `uw new` derives.
/// The atlas must not have names installed yet (`names::parse` checks against the kit's own names).
fn game_names(atlas: &Atlas, dir: &Path) -> Result<Vec<uw_decomp::names::GameName>, String> {
    let path = dir.join(uw_decomp::names::FILE);
    if !path.is_file() {
        return Ok(uw_decomp::names::derive(atlas));
    }
    let text = std::fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    uw_decomp::names::parse(atlas, &text).map_err(|errors| {
        errors.iter().map(|error| format!("{}: {error}", path.display())).collect::<Vec<_>>().join("\n")
    })
}

/// Every `fn` name of the nearest `lib` folder and of the project's `fns` folder, sorted.
fn fn_names(dir: &Path) -> Result<Vec<String>, String> {
    let mut entries: Vec<(PathBuf, bool)> =
        crate::build::lib_files(dir)?.into_iter().map(|path| (path, true)).collect();
    let words = dir.join("fns");
    if words.is_dir() {
        let mut paths: Vec<PathBuf> = std::fs::read_dir(&words)
            .map_err(|err| format!("{}: {err}", words.display()))?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| path.extension().is_some_and(|ext| ext == "uw"))
            .collect();
        paths.sort();
        entries.extend(paths.into_iter().map(|path| (path, true)));
    }
    let mut names = Vec::new();
    for source in crate::build::parse_sources(&entries)? {
        for item in &source.tree.items {
            if let uw_syntax::Item::Fn { name, .. } = item {
                names.push(name.text.clone());
            }
        }
    }
    names.sort();
    names.dedup();
    Ok(names)
}

/// `player.gold.get` -> `player.gold`: the object word whose verb fn this is. `None` when the last
/// part is not one of [`VERBS`] or what remains is not a dotted word (`player.gold` itself).
fn object_field(name: &str) -> Option<&str> {
    let (field, verb) = name.rsplit_once('.')?;
    (field.contains('.') && VERBS.contains(&verb)).then_some(field)
}

#[cfg(test)]
mod tests {
    use super::object_field;

    #[test]
    fn an_object_field_is_the_fn_name_without_its_verb() {
        assert_eq!(object_field("player.gold.get"), Some("player.gold"));
        assert_eq!(object_field("player.gold.remove"), Some("player.gold"));
        assert_eq!(object_field("player.gold"), None);
        assert_eq!(object_field("say"), None);
        assert_eq!(object_field("thing.get"), None);
    }
}
