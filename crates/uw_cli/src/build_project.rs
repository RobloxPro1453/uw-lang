//! P3b (docs/meclis/YON2-KARAR.md §3.3): build a whole-table project with the kit and, when asked,
//! compare every output `.txt` with the Native baseline (`kit-meta/native.baseline.json`, sha256 per
//! file).
//!
//! `uw build-project --native-check`: the 27 modules are written from the atlas values themselves.
//! This proves the module writer (`uw_decomp::project::pymodule`) before a project folder feeds it.
//! `uw build-project <dir>`: the same modules from the project folder (`project::load`), installed
//! with `uw build`'s safety rules.
//! The kit folder is never written (staging copy, `uw_kitrun`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::Value as Json;
use uw_decomp::project::pymodule::module_text;
use uw_decomp::records::{self, Schema};

const OVERLAY_MARKER: &str = ".uw-overlay";
const USAGE: &str = "usage:
  uw build-project <dir> [--compare-native] [--no-install]
      write the project folder's whole tables, build them with the kit and install the mod;
      --compare-native: say how many output files are byte-identical to Native
  uw build-project --native-check
      the same, but the modules come from the atlas values themselves (no project folder)";

/// The module files of a whole-table build: `module_<t>.py` text per atlas table, records in order.
pub fn modules(atlas: &uw_atlas::Atlas, records: &BTreeMap<String, Vec<Json>>) -> Vec<(String, String)> {
    atlas
        .tables
        .iter()
        .filter_map(|(name, table)| {
            let values = records.get(name)?;
            // The kit's own module comes along as `native_<module>.py`: its imports and helpers
            // (`find_troop`, `level_bits` …) stay visible to `process_*`; only the list is replaced.
            let text = format!("from native_{} import *
{}", table.module, module_text(&table.var, values));
            Some((format!("{}.py", table.module), text))
        })
        .collect()
}

/// A finished kit run and where the files it wrote are.
pub struct Built {
    pub report: uw_kitrun::Report,
    /// `<staging>/out`, the files the kit wrote (what `install` copies).
    pub out: PathBuf,
}

/// Writes `modules` (and the kit's own `native_<module>.py` next to them) into an overlay under
/// `<root>/target/uw-build/<name>` and runs the kit on it.
pub fn build(
    root: &Path,
    kit: &Path,
    python: &Path,
    name: &str,
    modules: &[(String, String)],
) -> Result<Built, String> {
    let work = root.join("target").join("uw-build").join(name);
    let overlay = work.join("overlay");
    crate::build::reset_dir(&overlay, OVERLAY_MARKER)?;
    for (file, text) in modules {
        let path = overlay.join(file);
        std::fs::write(&path, text.as_bytes()).map_err(|e| format!("{}: {e}", path.display()))?;
        let native = kit.join(file);
        let copy = overlay.join(format!("native_{file}"));
        std::fs::copy(&native, &copy).map_err(|e| format!("{}: {e}", native.display()))?;
    }
    let stage = uw_kitrun::StageOptions {
        kit: kit.to_path_buf(),
        staging: work.join("staging"),
        no_ids: false,
        overlay: Some(overlay),
    };
    let report = uw_kitrun::build(&stage, python).map_err(|e| e.to_string())?;
    let out = work.join("staging").join("out");
    Ok(Built { report, out })
}

/// The kit's failed steps, one block each (its last stderr lines).
pub fn failure_lines(report: &uw_kitrun::Report) -> Vec<String> {
    let mut lines = Vec::new();
    for step in report.steps.iter().filter(|s| s.exit != 0) {
        lines.push(format!("kit step {} failed:", step.step));
        lines.extend(step.stderr.lines().rev().take(8).collect::<Vec<_>>().into_iter().rev().map(|l| format!("    {l}")));
    }
    lines
}

/// Compares the kit's outputs with Native: the differing files' lines, how many outputs are
/// byte-identical and how many the baseline holds.
pub fn compare(root: &Path, report: &uw_kitrun::Report) -> Result<(Vec<String>, usize, usize), String> {
    let baseline_path = root.join("kit-meta").join("native.baseline.json");
    let baseline: Json = serde_json::from_str(
        &std::fs::read_to_string(&baseline_path).map_err(|e| format!("{}: {e}", baseline_path.display()))?,
    )
    .map_err(|e| format!("{}: {e}", baseline_path.display()))?;
    let want = baseline["outputs"].as_object().ok_or("baseline has no outputs")?;
    let mut same = 0;
    let mut lines = Vec::new();
    for (file, info) in want {
        match report.outputs.get(file) {
            Some(got) if Some(got.sha256.as_str()) == info["sha256"].as_str() => same += 1,
            Some(got) => lines.push(format!("differs  {file} ({} bytes, Native {})", got.bytes, info["bytes"])),
            None => lines.push(format!("missing  {file}")),
        }
    }
    for file in report.outputs.keys().filter(|f| !want.contains_key(*f)) {
        lines.push(format!("extra    {file}"));
    }
    Ok((lines, same, want.len()))
}

/// `outputs N/32 byte-identical to Native` (and the time the kit took).
pub fn outputs_line(same: usize, want: usize, seconds: f64) -> String {
    format!("outputs  {same}/{want} byte-identical to Native ({seconds:.0} s)")
}

/// Runs the kit on `modules` (overlay) and compares the outputs with the baseline. Returns the lines of
/// the report and whether every output is byte-identical.
pub fn build_and_compare(
    root: &Path,
    kit: &Path,
    python: &Path,
    name: &str,
    modules: &[(String, String)],
) -> Result<(Vec<String>, bool), String> {
    let built = build(root, kit, python, name, modules)?;
    let report = &built.report;
    let mut lines = failure_lines(report);
    let (mut differences, same, want) = compare(root, report)?;
    lines.append(&mut differences);
    let ok = report.failed_steps.is_empty() && same == want && report.outputs.len() == want;
    lines.insert(0, outputs_line(same, want, report.total_seconds));
    Ok((lines, ok))
}

/// What the command line asked for.
#[derive(Debug, PartialEq)]
pub struct Args {
    /// The project folder (`--native-check` has none).
    pub dir: Option<PathBuf>,
    /// Build the atlas values themselves, no project folder.
    pub native_check: bool,
    /// Compare every output with Native.
    pub compare_native: bool,
    /// Install the result as `<Warband>/Modules/<project folder name>`.
    pub install: bool,
}

pub fn parse_args(args: &[String]) -> Result<Args, String> {
    let mut parsed = Args { dir: None, native_check: false, compare_native: false, install: true };
    for arg in args {
        match arg.as_str() {
            "--native-check" => parsed.native_check = true,
            "--compare-native" => parsed.compare_native = true,
            "--no-install" => parsed.install = false,
            other if other.starts_with('-') => return Err(format!("unknown argument `{other}`\n{USAGE}")),
            other if parsed.dir.is_none() => parsed.dir = Some(PathBuf::from(other)),
            other => return Err(format!("unexpected argument `{other}`\n{USAGE}")),
        }
    }
    if parsed.native_check && parsed.dir.is_some() {
        return Err(format!("--native-check builds the atlas values themselves; no project folder is needed\n{USAGE}"));
    }
    if !parsed.native_check && parsed.dir.is_none() {
        return Err(format!("which project folder should I build?\n{USAGE}"));
    }
    Ok(parsed)
}

pub fn cmd(args: &[String]) -> ExitCode {
    let args = match parse_args(args) {
        Ok(args) => args,
        Err(message) => return crate::fail(&message),
    };
    if args.native_check {
        return native_check();
    }
    let Some(dir) = args.dir.as_deref() else {
        return crate::fail(&format!("which project folder should I build?\n{USAGE}"));
    };
    build_project(dir, &args)
}

/// `uw build-project --native-check`: the modules are the atlas values themselves.
fn native_check() -> ExitCode {
    let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let model = match crate::load(None) {
        Ok(model) => model,
        Err(message) => return crate::fail(&message),
    };
    let Some(kit) = crate::build::find_kit(&root) else {
        return crate::fail("Module System kit not found");
    };
    let python = PathBuf::from(uw_kitrun::DEFAULT_PYTHON27);
    let records: BTreeMap<String, Vec<Json>> = model
        .atlas
        .tables
        .iter()
        .map(|(name, table)| (name.clone(), table.entries.iter().map(|e| e.value.clone()).collect()))
        .collect();
    let modules = modules(&model.atlas, &records);
    println!("modules  {} written from the atlas", modules.len());
    match build_and_compare(&root, &kit, &python, "_native_check", &modules) {
        Ok((lines, ok)) => {
            for line in lines {
                println!("{line}");
            }
            if ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        Err(message) => crate::fail(&message),
    }
}

/// The `fn` sources a project's record blocks may call: `lib/*.uw` (UW's own words) and the
/// project's `fns/*.uw` files, in that order so a project `fn` cannot replace a lib one (T-084).
fn word_sources(dir: &Path) -> Result<Vec<uw_check::Source>, String> {
    let mut entries: Vec<(PathBuf, bool)> =
        crate::build::lib_files(dir)?.into_iter().map(|path| (path, true)).collect();
    let words = dir.join("fns");
    if words.is_dir() {
        let mut paths: Vec<PathBuf> = std::fs::read_dir(&words)
            .map_err(|e| format!("{}: {e}", words.display()))?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| path.extension().is_some_and(|ext| ext == "uw"))
            .collect();
        paths.sort();
        entries.extend(paths.into_iter().map(|path| (path, true)));
    }
    crate::build::parse_sources(&entries)
}

/// Translation keys whose Native text no longer applies because the project renamed the record,
/// with the record's new text: `id` for a changed `name`, `id_pl` for a changed `plural_name`.
fn renamed_keys(
    atlas: &uw_atlas::Atlas,
    schemas: &[Schema],
    records: &BTreeMap<String, Vec<Json>>,
) -> BTreeMap<String, String> {
    let mut keys = BTreeMap::new();
    for schema in schemas {
        let Some(table) = atlas.tables.get(&schema.table) else { continue };
        let Some(values) = records.get(&schema.table) else { continue };
        let name = schema.fields.iter().position(|field| field.name == "name");
        let plural = schema.fields.iter().position(|field| field.name == "plural_name");
        if name.is_none() && plural.is_none() {
            continue;
        }
        for (entry, value) in table.entries.iter().zip(values) {
            let Some(id) = &entry.id else { continue };
            if let Some(index) = name {
                if field(value, index) != field(&entry.value, index) {
                    keys.insert(id.clone(), text_of(value, index));
                }
            }
            if let Some(index) = plural {
                if field(value, index) != field(&entry.value, index) {
                    keys.insert(format!("{id}_pl"), text_of(value, index));
                }
            }
        }
    }
    keys
}

/// One field as text (`""` when it is not a string).
fn text_of(value: &Json, index: usize) -> String {
    field(value, index).and_then(Json::as_str).unwrap_or_default().to_owned()
}

/// A town/castle/village name: what the project calls it and what Native called it (T-094).
#[derive(Debug)]
struct TownName {
    current: String,
    native: String,
}

/// Every town/castle/village with its project name and its Native name: town-ness is the record's
/// own folder (`parties/towns/…`, `layout::path_for`), the name is the schema's `name` field.
fn town_names(
    atlas: &uw_atlas::Atlas,
    schemas: &[Schema],
    records: &BTreeMap<String, Vec<Json>>,
) -> BTreeMap<String, TownName> {
    let mut towns = BTreeMap::new();
    for schema in schemas.iter().filter(|schema| schema.table == "parties") {
        let Some(table) = atlas.tables.get(&schema.table) else { continue };
        let Some(values) = records.get(&schema.table) else { continue };
        let Some(index) = schema.fields.iter().position(|field| field.name == "name") else { continue };
        for (entry, value) in table.entries.iter().zip(values) {
            let Some(id) = &entry.id else { continue };
            let path = uw_decomp::project::layout::path_for(atlas, schema, entry);
            if !["parties/towns/", "parties/castles/", "parties/villages/"]
                .iter()
                .any(|folder| path.starts_with(folder))
            {
                continue;
            }
            let current = text_of(value, index);
            if current.is_empty() {
                continue;
            }
            towns.insert(id.clone(), TownName { current, native: text_of(&entry.value, index) });
        }
    }
    towns
}

/// A record field by schema index: a tuple is a JSON array, a `list=true` table is `{"list": …}`.
fn field(value: &Json, index: usize) -> Option<&Json> {
    match value {
        Json::Array(values) => values.get(index),
        Json::Object(map) => map.get("list")?.as_array()?.get(index),
        _ => None,
    }
}

/// What installing a project's translations changes besides copying: the translation keys of
/// renamed records, and the per-language town suffix from `uw.toml` (T-094).
#[derive(Debug, Default)]
struct LanguageRules {
    /// Translation key (`p_town_6`, `trp_x_pl`) -> the record's new text.
    renamed: BTreeMap<String, String>,
    /// Town/castle/village id -> its project name and its Native name.
    towns: BTreeMap<String, TownName>,
    /// Language folder name (`tr`) -> the configured town suffix.
    suffixes: BTreeMap<String, String>,
}

/// [`LanguageRules`] from the project: its renamed records, its towns and its `uw.toml` setting.
fn language_rules(
    atlas: &uw_atlas::Atlas,
    schemas: &[Schema],
    records: &BTreeMap<String, Vec<Json>>,
    project: &Path,
) -> Result<LanguageRules, String> {
    Ok(LanguageRules {
        renamed: renamed_keys(atlas, schemas, records),
        towns: town_names(atlas, schemas, records),
        suffixes: uw_decomp::project::town_suffixes(project)?,
    })
}

/// Replaces `<dest>/languages` with the project's own `languages/`, leaving out the lines of the
/// renamed records in `rules`. The project's files are only read. Returns one warning per dropped
/// line. With no project `languages/`, Native's own (`<Modules>/Native/languages`) is used, filtered the same way.
fn install_languages(project: &Path, dest: &Path, rules: &LanguageRules) -> Result<Vec<String>, String> {
    let to = dest.join("languages");
    // The project's own translations; a project made before it had them (or with none) uses Native's,
    // so the game never loses its language (the owner's test: the whole game turned English).
    let own = project.join("languages");
    let native = dest.parent().map(|modules| modules.join("Native").join("languages"));
    let from = if own.is_dir() {
        own
    } else if let Some(native) = native.filter(|path| path.is_dir()) {
        native
    } else {
        return Ok(Vec::new()); // nothing to install from: leave the installed copy as it is
    };
    if to.exists() {
        std::fs::remove_dir_all(&to).map_err(|e| format!("{}: {e}", to.display()))?;
    }
    let mut warnings = Vec::new();
    copy_languages_filtered(&from, &to, &from, rules, &mut warnings)?;
    Ok(warnings)
}

fn copy_languages_filtered(
    from: &Path,
    to: &Path,
    source_root: &Path,
    rules: &LanguageRules,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    for entry in std::fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let target = to.join(entry.file_name());
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            copy_languages_filtered(&path, &target, source_root, rules, warnings)?;
        } else if path.extension().is_some_and(|ext| ext == "csv") {
            let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            let relative = path.strip_prefix(source_root).unwrap_or(&path);
            let file = format!("languages/{}", relative.to_string_lossy().replace('\\', "/"));
            // The language is the first folder under `languages/` (`languages/tr/parties.csv`).
            let language = relative.components().next().and_then(|part| part.as_os_str().to_str());
            let kept = filter_translation_lines(&bytes, rules, language, &file, warnings);
            std::fs::write(&target, kept).map_err(|e| format!("{}: {e}", target.display()))?;
        } else {
            std::fs::copy(&path, &target).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    Ok(())
}

/// Keeps every line except the `id|text` lines of a renamed record whose text does not say the new
/// name yet; a line that already does is kept (T-094). Untouched lines stay byte-identical (BOM,
/// CRLF and a last line without a newline included). A town line gets the language's `town_suffix`.
fn filter_translation_lines(
    bytes: &[u8],
    rules: &LanguageRules,
    language: Option<&str>,
    file: &str,
    warnings: &mut Vec<String>,
) -> Vec<u8> {
    let suffix = language.and_then(|language| rules.suffixes.get(language));
    let mut kept = Vec::with_capacity(bytes.len());
    for line in bytes.split_inclusive(|&byte| byte == b'\n') {
        if let Some(key) = translation_key(line).and_then(|key| std::str::from_utf8(key).ok()) {
            if let Some(name) = rules.renamed.get(key) {
                let says_new_name = !name.is_empty()
                    && translation_text(line).is_some_and(|text| crate::rename::contains_word(text, name));
                if !says_new_name {
                    warnings.push(format!("translation dropped: {key} ({file}): renamed in the project"));
                    continue;
                }
            }
            if let (Some(suffix), Some(town)) = (suffix, rules.towns.get(key)) {
                kept.extend_from_slice(&with_town_suffix(line, town, suffix));
                continue;
            }
        }
        kept.extend_from_slice(line);
    }
    kept
}

/// One `id|text` line with the town's Native suffix replaced by `suffix` (T-094): the text reads
/// `<name><suffix>`, the name being the project's or Native's, and the new text is the project's
/// name + `suffix`. A text that starts with neither name (a fully localized one) stays as it is.
fn with_town_suffix(line: &[u8], town: &TownName, suffix: &str) -> Vec<u8> {
    let Some(bar) = line.iter().position(|&byte| byte == b'|') else { return line.to_vec() };
    let Some(text) = std::str::from_utf8(&line[bar + 1..]).ok() else { return line.to_vec() };
    if !starts_with_word(text, &town.current) && !starts_with_word(text, &town.native) {
        return line.to_vec();
    }
    let ending = &text[text.trim_end_matches(['\r', '\n']).len()..];
    let mut out = line[..=bar].to_vec();
    out.extend_from_slice(town.current.as_bytes());
    out.extend_from_slice(suffix.as_bytes());
    out.extend_from_slice(ending.as_bytes());
    out
}

/// True when `text` starts with `word` as a whole word (`Praven Şehri` for `Praven`).
fn starts_with_word(text: &str, word: &str) -> bool {
    !word.is_empty()
        && text.starts_with(word)
        && text[word.len()..].chars().next().is_none_or(|c| !crate::rename::is_word_char(c))
}

fn translation_key(line: &[u8]) -> Option<&[u8]> {
    let line = line.strip_prefix(&b"\xef\xbb\xbf"[..]).unwrap_or(line);
    let end = line.iter().position(|&byte| byte == b'|')?;
    Some(&line[..end])
}

/// The text half of an `id|text` line.
fn translation_text(line: &[u8]) -> Option<&str> {
    let line = line.strip_prefix(&b"\xef\xbb\xbf"[..]).unwrap_or(line);
    let end = line.iter().position(|&byte| byte == b'|')?;
    std::str::from_utf8(&line[end + 1..]).ok()
}

/// The project's atlas (with its game names) and record schemas, without the kit.
pub fn project_atlas(dir: &Path, atlas: Option<PathBuf>) -> Result<(uw_atlas::Atlas, Vec<Schema>), String> {
    let (mut atlas, schemas) = crate::new::atlas_and_schemas(atlas)?;
    crate::new::install_names(&mut atlas, dir)?;
    Ok((atlas, schemas))
}

/// Why a project's records did not load: the word sources failed (one message), or the records had
/// problems (one line each).
pub enum LoadFailure {
    Words(String),
    Records(Vec<String>),
}

/// Loads the project folder's records (parse + check through its lib/fns words), without the kit.
pub fn load_records(
    dir: &Path,
    atlas: &uw_atlas::Atlas,
    schemas: &[Schema],
) -> Result<BTreeMap<String, Vec<Json>>, LoadFailure> {
    let words = word_sources(dir).map_err(LoadFailure::Words)?;
    let mut compile =
        |text: &str| crate::roundtrip::compile_with(atlas, text, &words).map(|ops| records::ops_to_json(&ops));
    let mut parse = |schema: &Schema, text: &str| records::parse_with_blocks(atlas, schema, text, &mut compile);
    uw_decomp::project::load::load(atlas, schemas, dir, &mut parse).map_err(LoadFailure::Records)
}

/// `uw build-project <dir>`: the same build, from the project folder.
fn build_project(dir: &Path, args: &Args) -> ExitCode {
    let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let (atlas, schemas) = match project_atlas(dir, None) {
        Ok(found) => found,
        Err(message) => return crate::fail(&message),
    };
    let records = match load_records(dir, &atlas, &schemas) {
        Ok(records) => records,
        Err(LoadFailure::Words(message)) => return crate::fail(&message),
        Err(LoadFailure::Records(errors)) => {
            for line in &errors {
                eprintln!("  {line}");
            }
            return crate::fail(&format!(
                "{} problem{} in {}; nothing was built",
                errors.len(),
                if errors.len() == 1 { "" } else { "s" },
                dir.display()
            ));
        }
    };
    // The mod name is the project folder's name, checked like `uw build` checks it.
    let Some(name) = dir.file_name().map(|n| n.to_string_lossy().into_owned()) else {
        return crate::fail(&format!("{} has no folder name to use as the mod name", dir.display()));
    };
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return crate::fail(&format!("mod name '{name}' may only use letters, digits and _"));
    }
    let modules = modules(&atlas, &records);
    println!("modules  {} written from {}", modules.len(), dir.display());
    let Some(kit) = crate::build::find_kit(dir).or_else(|| crate::build::find_kit(&root)) else {
        return crate::fail("Module System kit not found");
    };
    let python = PathBuf::from(uw_kitrun::DEFAULT_PYTHON27);
    let built = match build(&root, &kit, &python, &name, &modules) {
        Ok(built) => built,
        Err(message) => return crate::fail(&message),
    };
    for line in failure_lines(&built.report) {
        eprintln!("{line}");
    }
    for problem in built.report.steps.iter().flat_map(|s| &s.problems) {
        eprintln!("kit warning: {problem}");
    }
    println!("built     {} files in {:.0} s", built.report.outputs.len(), built.report.total_seconds);
    let mut ok = built.report.failed_steps.is_empty();
    if args.compare_native {
        match compare(&root, &built.report) {
            Ok((lines, same, want)) => {
                println!("{}", outputs_line(same, want, built.report.total_seconds));
                for line in lines {
                    println!("{line}");
                }
                ok = ok && same == want && built.report.outputs.len() == want;
            }
            Err(message) => return crate::fail(&message),
        }
    }
    if !ok {
        return crate::fail("the build did not pass (see above); nothing was installed");
    }
    if !args.install {
        println!("done      output in {} (not installed: --no-install)", built.out.display());
        return ExitCode::SUCCESS;
    }
    let Some(modules_dir) = crate::build::find_modules() else {
        return crate::fail("Warband's Modules folder not found; pass --no-install");
    };
    match crate::build::install(&built.out, &modules_dir, &name) {
        Ok(dest) => {
            let rules = match language_rules(&atlas, &schemas, &records, dir) {
                Ok(rules) => rules,
                Err(message) => return crate::fail(&message),
            };
            match install_languages(dir, &dest, &rules) {
                Ok(lines) => {
                    for line in lines {
                        println!("{line}");
                    }
                }
                Err(message) => return crate::fail(&message),
            }
            println!("installed {}", dest.display());
            println!("done      pick '{}' in the Warband launcher", name);
            ExitCode::SUCCESS
        }
        Err(message) => crate::fail(&message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two tables; the module texts and the mod name come from these.
    const TINY_ATLAS: &str = r#"{
      "schema": 1, "kit_hash": "t057", "ops": [], "hooks": [], "ranges": [],
      "tables": {
        "factions": {"module":"module_factions","var":"factions","id_file":"ID_factions","id_prefix":"fac_",
          "entries": [
            {"index":0,"name":"no_faction","id":"fac_no_faction","dup":null,"line":1,
              "value":["no_faction","No Faction",0,10,[],[]]}
          ]},
        "items": {"module":"module_items","var":"items","id_file":"ID_items","id_prefix":"itm_",
          "entries": [
            {"index":0,"name":"sword","id":"itm_sword","dup":null,"line":2,
              "value":["sword","Sword",0,0,0]}
          ]}
      }
    }"#;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    /// The atlas' own values per table, like `--native-check` builds them.
    fn atlas_records(atlas: &uw_atlas::Atlas) -> BTreeMap<String, Vec<Json>> {
        atlas
            .tables
            .iter()
            .map(|(name, table)| (name.clone(), table.entries.iter().map(|e| e.value.clone()).collect()))
            .collect()
    }

    #[test]
    fn arguments_take_a_folder_and_the_flags() {
        let parsed = parse_args(&args(&["modum"])).unwrap();
        assert_eq!(parsed.dir, Some(PathBuf::from("modum")));
        assert!(parsed.install && !parsed.compare_native && !parsed.native_check, "{parsed:?}");

        let parsed = parse_args(&args(&["modum", "--compare-native", "--no-install"])).unwrap();
        assert_eq!(parsed.dir, Some(PathBuf::from("modum")));
        assert!(parsed.compare_native && !parsed.install, "{parsed:?}");

        // `--native-check` keeps working and needs no folder.
        let parsed = parse_args(&args(&["--native-check"])).unwrap();
        assert!(parsed.native_check && parsed.dir.is_none() && parsed.install, "{parsed:?}");
    }

    #[test]
    fn arguments_reject_unknown_flags_and_missing_folder() {
        assert!(parse_args(&args(&[])).is_err());
        let err = parse_args(&args(&["--what"])).unwrap_err();
        assert!(err.contains("unknown argument `--what`"), "{err}");
        let err = parse_args(&args(&["one", "two"])).unwrap_err();
        assert!(err.contains("unexpected argument `two`"), "{err}");
        let err = parse_args(&args(&["--native-check", "modum"])).unwrap_err();
        assert!(err.contains("no project folder is needed"), "{err}");
    }

    #[test]
    fn module_texts_import_the_native_module_and_name_their_table() {
        let atlas = uw_atlas::Atlas::from_str(TINY_ATLAS).unwrap();
        let modules = modules(&atlas, &atlas_records(&atlas));
        let mut files: Vec<&str> = modules.iter().map(|(file, _)| file.as_str()).collect();
        files.sort();
        assert_eq!(files, ["module_factions.py", "module_items.py"]);
        for (file, text) in &modules {
            let module = file.strip_suffix(".py").unwrap();
            assert!(text.starts_with(&format!("from native_{module} import *\n")), "{text}");
            let table = atlas.tables.values().find(|t| t.module == module).unwrap();
            assert!(text.contains(&format!("\n{} = [\n", table.var)), "{text}");
        }
        let factions = &modules.iter().find(|(file, _)| file == "module_factions.py").unwrap().1;
        assert!(factions.contains(r#"("no_faction", "No Faction", 0, 10, (), ())"#), "{factions}");
    }

    #[test]
    fn a_table_without_records_gets_no_module() {
        let atlas = uw_atlas::Atlas::from_str(TINY_ATLAS).unwrap();
        let mut records = atlas_records(&atlas);
        records.remove("items");
        let modules = modules(&atlas, &records);
        assert_eq!(modules.len(), 1, "{modules:?}");
        assert_eq!(modules[0].0, "module_factions.py");
    }

    #[test]
    fn compare_counts_identical_outputs_and_lists_the_others() {
        let root = std::env::temp_dir().join(format!("uw-build-project-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("kit-meta")).unwrap();
        std::fs::write(
            root.join("kit-meta").join("native.baseline.json"),
            r#"{"outputs": {"menus.txt": {"bytes": 5, "sha256": "aa"}, "scripts.txt": {"bytes": 3, "sha256": "bb"}}}"#,
        )
        .unwrap();
        let mut outputs = BTreeMap::new();
        outputs.insert("menus.txt".to_owned(), uw_kitrun::OutputFile { bytes: 5, sha256: "aa".into() });
        outputs.insert("scripts.txt".to_owned(), uw_kitrun::OutputFile { bytes: 3, sha256: "cc".into() });
        outputs.insert("extra.txt".to_owned(), uw_kitrun::OutputFile { bytes: 1, sha256: "dd".into() });
        let report = uw_kitrun::Report {
            kit_hash: "t057".into(),
            no_ids: false,
            overlay: None,
            steps: Vec::new(),
            failed_steps: Vec::new(),
            total_seconds: 1.0,
            outputs,
        };
        let (lines, same, want) = compare(&root, &report).unwrap();
        assert_eq!((same, want), (1, 2));
        assert!(lines.iter().any(|l| l.contains("differs  scripts.txt")), "{lines:?}");
        assert!(lines.iter().any(|l| l.contains("extra    extra.txt")), "{lines:?}");
        assert_eq!(outputs_line(same, want, 1.0), "outputs  1/2 byte-identical to Native (1 s)");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn a_project_block_calls_a_word_from_lib_and_its_fns_folder() {
        let root = std::env::temp_dir().join(format!("uw-build-project-words-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("lib")).unwrap();
        std::fs::create_dir_all(root.join("fns")).unwrap();
        std::fs::write(
            root.join("lib").join("core.uw"),
            "fn give_gold(:amount) {\n    troop_add_gold(trp_player, :amount)\n}\n",
        )
        .unwrap();
        std::fs::write(
            root.join("fns").join("mine.uw"),
            "fn tip(:amount) {\n    give_gold(:amount)\n}\n\
             fn player.gold.add(:value) {\n    give_gold(:value)\n}\n",
        )
        .unwrap();

        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_golden.json");
        let atlas = uw_atlas::Atlas::from_path(path).unwrap();
        let words = word_sources(&root).expect("lib and fns parse");
        let names: Vec<&str> = words.iter().map(|source| source.file.name.as_str()).collect();
        assert_eq!(names, ["core.uw", "mine.uw"]);

        let ops = crate::roundtrip::compile_with(&atlas, "tip(5)\nplayer.gold += 5", &words)
            .expect("the project's words inline");
        let text = uw_decomp::decompile::decompile(&atlas, &ops, uw_decomp::decompile::Mode::Structured);
        assert_eq!(text.matches("troop_add_gold").count(), 2, "{text}");

        // The module built from the record holds the operation the word inlined to (1528).
        let value = serde_json::json!([null, null, null, null, [], records::ops_to_json(&ops)]);
        let records = BTreeMap::from([("menus".to_owned(), vec![value])]);
        let modules = modules(&atlas, &records);
        assert!(modules[0].1.contains("1528"), "{}", modules[0].1);

        let _ = std::fs::remove_dir_all(&root);
    }

    /// One parties record and one troop, enough for the translation rename check.
    const LANGUAGE_ATLAS: &str = r#"{
      "schema": 1, "kit_hash": "t089", "ops": [], "hooks": [], "ranges": [],
      "tables": {
        "parties": {"module":"module_parties","var":"parties","id_file":"ID_parties","id_prefix":"p_",
          "entries": [
            {"index":0,"name":"town_6","id":"p_town_6","dup":null,"line":1,
              "value":["town_6","Praven",0]}
          ]},
        "troops": {"module":"module_troops","var":"troops","id_file":"ID_troops","id_prefix":"trp_",
          "entries": [
            {"index":0,"name":"swadian_recruit","id":"trp_swadian_recruit","dup":null,"line":2,
              "value":{"list":["swadian_recruit","Swadian Recruit","Swadian Recruits",0]}}
          ]}
      }
    }"#;

    const PARTY_SCHEMA: &str = r#"table = "parties"
kind = "party"
id_prefix = "p_"

[[fields]]
name = "id"
ty = "raw"
source = "module_parties.py:10"

[[fields]]
name = "name"
ty = "raw"
source = "module_parties.py:12"

[[fields]]
name = "x"
ty = "raw"
source = "module_parties.py:13"
"#;

    const TROOP_SCHEMA: &str = r#"table = "troops"
kind = "troop"
id_prefix = "trp_"
list = true

[[fields]]
name = "id"
ty = "raw"
source = "module_troops.py:13"

[[fields]]
name = "name"
ty = "raw"
source = "module_troops.py:14"

[[fields]]
name = "plural_name"
ty = "raw"
source = "module_troops.py:15"

[[fields]]
name = "x"
ty = "raw"
source = "module_troops.py:16"
"#;

    fn language_scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("uw-build-project-language-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn an_installed_renamed_record_loses_its_line_and_the_project_keeps_it() {
        let root = language_scratch("party");
        let project = root.join("proje");
        let dest = root.join("Modules").join("proje");
        std::fs::create_dir_all(project.join("languages").join("tr")).unwrap();
        std::fs::create_dir_all(dest.join("languages").join("tr")).unwrap();
        let line = "p_town_6|Praven Şehri\n";
        std::fs::write(project.join("languages").join("tr").join("parties.csv"), line).unwrap();
        // Native's copy in the installed folder (what the bug came from) and a file only it has.
        std::fs::write(dest.join("languages").join("tr").join("parties.csv"), line).unwrap();
        std::fs::write(dest.join("languages").join("tr").join("old.csv"), "p_town_6|eski\n").unwrap();

        let atlas = uw_atlas::Atlas::from_str(LANGUAGE_ATLAS).unwrap();
        let schemas = [Schema::from_toml(PARTY_SCHEMA).unwrap()];
        let records = BTreeMap::from([(
            "parties".to_owned(),
            vec![serde_json::json!(["town_6", "Adam Siken", 0])],
        )]);
        let rules = LanguageRules { renamed: renamed_keys(&atlas, &schemas, &records), ..LanguageRules::default() };
        assert_eq!(rules.renamed.keys().cloned().collect::<Vec<_>>(), ["p_town_6"]);
        assert_eq!(rules.renamed["p_town_6"], "Adam Siken");

        let warnings = install_languages(&project, &dest, &rules).unwrap();
        assert_eq!(
            warnings,
            ["translation dropped: p_town_6 (languages/tr/parties.csv): renamed in the project"]
        );
        assert_eq!(std::fs::read_to_string(dest.join("languages").join("tr").join("parties.csv")).unwrap(), "");
        assert!(!dest.join("languages").join("tr").join("old.csv").exists(), "Native's copy is replaced");
        // The project's own file is untouched.
        assert_eq!(std::fs::read_to_string(project.join("languages").join("tr").join("parties.csv")).unwrap(), line);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn a_changed_plural_drops_only_the_plural_line() {
        let root = language_scratch("plural");
        let project = root.join("proje");
        let dest = root.join("Modules").join("proje");
        std::fs::create_dir_all(project.join("languages").join("tr")).unwrap();
        std::fs::write(
            project.join("languages").join("tr").join("troops.csv"),
            "trp_swadian_recruit|Swadian Recruit\ntrp_swadian_recruit_pl|Swadian Recruits\n",
        )
        .unwrap();

        let atlas = uw_atlas::Atlas::from_str(LANGUAGE_ATLAS).unwrap();
        let schemas = [Schema::from_toml(TROOP_SCHEMA).unwrap()];
        let records = BTreeMap::from([(
            "troops".to_owned(),
            vec![serde_json::json!({"list": ["swadian_recruit", "Swadian Recruit", "Swadian Askerler", 0]})],
        )]);
        let rules = LanguageRules { renamed: renamed_keys(&atlas, &schemas, &records), ..LanguageRules::default() };
        assert_eq!(rules.renamed.keys().cloned().collect::<Vec<_>>(), ["trp_swadian_recruit_pl"]);

        let warnings = install_languages(&project, &dest, &rules).unwrap();
        assert_eq!(
            warnings,
            ["translation dropped: trp_swadian_recruit_pl (languages/tr/troops.csv): renamed in the project"]
        );
        assert_eq!(
            std::fs::read_to_string(dest.join("languages").join("tr").join("troops.csv")).unwrap(),
            "trp_swadian_recruit|Swadian Recruit\n"
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn translation_filtering_keeps_untouched_lines_byte_identical() {
        let mut warnings = Vec::new();
        let bytes = b"\xef\xbb\xbfp_town_5|Uxkhal\r\np_town_6|Praven\r\np_town_6|tekrar";
        let rules = LanguageRules {
            renamed: BTreeMap::from([("p_town_6".to_owned(), "Adam Siken".to_owned())]),
            ..LanguageRules::default()
        };
        let kept = filter_translation_lines(bytes, &rules, None, "languages/tr/parties.csv", &mut warnings);
        assert_eq!(kept, b"\xef\xbb\xbfp_town_5|Uxkhal\r\n");
        assert_eq!(
            warnings,
            [
                "translation dropped: p_town_6 (languages/tr/parties.csv): renamed in the project",
                "translation dropped: p_town_6 (languages/tr/parties.csv): renamed in the project"
            ]
        );
    }

    #[test]
    fn a_translation_that_already_says_the_new_name_is_kept() {
        let mut warnings = Vec::new();
        let bytes = "p_town_6|Praven Şehri\np_town_6|Adam Siken Şehri\np_town_6|Pravenfall\n".as_bytes();
        let rules = LanguageRules {
            renamed: BTreeMap::from([("p_town_6".to_owned(), "Adam Siken".to_owned())]),
            ..LanguageRules::default()
        };
        let kept = filter_translation_lines(bytes, &rules, None, "languages/tr/parties.csv", &mut warnings);
        assert_eq!(kept, "p_town_6|Adam Siken Şehri\n".as_bytes());
        assert_eq!(warnings.len(), 2, "{warnings:?}");
    }

    #[test]
    fn a_town_suffix_replaces_the_native_suffix_of_a_town_name() {
        const TOWNS: &str = r#"{
          "schema": 1, "kit_hash": "t094", "ops": [], "hooks": [],
          "ranges": [
            {"name": "towns_begin", "value": "p_town_6", "line": 1552},
            {"name": "towns_end", "value": 1, "line": 1556}
          ],
          "tables": {
            "parties": {"module":"module_parties","var":"parties","id_file":"ID_parties","id_prefix":"p_",
              "entries": [
                {"index":0,"name":"town_6","id":"p_town_6","dup":null,"line":1,
                  "value":["town_6","Praven",0]}
              ]}
          }
        }"#;
        let atlas = uw_atlas::Atlas::from_str(TOWNS).unwrap();
        let schemas = [Schema::from_toml(PARTY_SCHEMA).unwrap()];
        let records = BTreeMap::from([(
            "parties".to_owned(),
            vec![serde_json::json!(["town_6", "Adam Siken", 0])],
        )]);
        let towns = town_names(&atlas, &schemas, &records);
        assert_eq!(towns["p_town_6"].current, "Adam Siken");
        assert_eq!(towns["p_town_6"].native, "Praven");

        let mut warnings = Vec::new();
        let bytes = "p_town_6|Praven Şehri\np_town_6|Adam Siken Kalesi\np_town_5|Uxkhal\n".as_bytes();
        let renamed = LanguageRules {
            towns: town_names(&atlas, &schemas, &records),
            suffixes: BTreeMap::from([("tr".to_owned(), String::new())]),
            ..LanguageRules::default()
        };
        assert_eq!(
            filter_translation_lines(bytes, &renamed, Some("tr"), "languages/tr/parties.csv", &mut warnings),
            "p_town_6|Adam Siken\np_town_6|Adam Siken\np_town_5|Uxkhal\n".as_bytes()
        );
        let suffixed = LanguageRules {
            towns: town_names(&atlas, &schemas, &records),
            suffixes: BTreeMap::from([("tr".to_owned(), " Şehri".to_owned())]),
            ..LanguageRules::default()
        };
        assert_eq!(
            filter_translation_lines(bytes, &suffixed, Some("tr"), "languages/tr/parties.csv", &mut warnings),
            "p_town_6|Adam Siken Şehri\np_town_6|Adam Siken Şehri\np_town_5|Uxkhal\n".as_bytes()
        );
        assert!(warnings.is_empty(), "{warnings:?}");
    }

    #[test]
    fn uw_toml_gives_each_language_its_town_suffix() {
        let root = language_scratch("suffix");
        assert_eq!(uw_decomp::project::town_suffixes(&root).unwrap(), BTreeMap::new());
        std::fs::write(
            root.join("uw.toml"),
            "base = \"native-export\"\n[languages.tr]\ntown_suffix = \"\"\n[languages.en]\ntown_suffix = \" Town\"\n",
        )
        .unwrap();
        let suffixes = uw_decomp::project::town_suffixes(&root).unwrap();
        assert_eq!(suffixes["tr"], "");
        assert_eq!(suffixes["en"], " Town");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn a_project_without_languages_installs_natives_filtered_never_none() {
        // The owner's test: an older project had no languages/, the install removed Native's copy and
        // the whole game turned English. Now Native's own languages are installed, filtered.
        let root = std::env::temp_dir().join(format!("uw-lang-fallback-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let project = root.join("proje_src");
        let modules = root.join("Modules");
        let native_tr = modules.join("Native").join("languages").join("tr");
        let dest = modules.join("proje");
        std::fs::create_dir_all(&project).unwrap();
        std::fs::create_dir_all(&native_tr).unwrap();
        std::fs::create_dir_all(dest.join("languages").join("tr")).unwrap();
        std::fs::write(native_tr.join("parties.csv"), "p_town_5|Uxkhal
p_town_6|Praven Şehri
").unwrap();
        std::fs::write(native_tr.join("ui.csv"), "ui_x|Tamam
").unwrap();
        let rules = LanguageRules {
            renamed: BTreeMap::from([("p_town_6".to_owned(), "Adam Siken".to_owned())]),
            ..LanguageRules::default()
        };
        let warnings = install_languages(&project, &dest, &rules).unwrap();
        assert_eq!(warnings.len(), 1, "{warnings:?}");
        let tr = dest.join("languages").join("tr");
        assert_eq!(std::fs::read_to_string(tr.join("parties.csv")).unwrap(), "p_town_5|Uxkhal
");
        assert_eq!(std::fs::read_to_string(tr.join("ui.csv")).unwrap(), "ui_x|Tamam
");
        // Native's own files are only read.
        assert!(std::fs::read_to_string(native_tr.join("parties.csv")).unwrap().contains("Praven"));
        let _ = std::fs::remove_dir_all(&root);
    }
}