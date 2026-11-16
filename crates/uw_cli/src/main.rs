//! The `uw` command line tool.

mod build;
mod check;
mod explain;
mod flow_json;
mod roundtrip;
mod graph;
mod graph_export;
mod impact;
mod model;
mod new;
mod build_project;
mod rename;
mod tr;
mod tui;
mod words;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const USAGE: &str = "usage:
  uw create <mod name>
      make the folder <mod name> here: all of Native as UW text, with game names (names.uw);
      then `cd <mod name>` and `uw build`
  uw check [folder] [--json] [--atlas <atlas.json>]
      parse and check a mod folder or a `uw create` project without building;
      --json: one JSON object per error, then {\"summary\":{\"errors\":N,\"warnings\":M}}
  uw build [folder] [--no-install] ...
      no folder = this folder; a folder made by `uw create` builds the whole mod and installs it
  uw build <mod folder> [--debug] [--no-install] [--modules <Warband/Modules>] [--kit <kit>] [--python <python27.exe>]
      check the .uw files, build them with the Module System kit and install the mod;
      --debug: in game every menu prints its id, so you see what to override
  uw graph <thing> [--depth N] [--mermaid] [--html <file.html>] [--tr <languages/tr dir>]
      what <thing> leads to, as a tree (or a Mermaid chart / an HTML page with the drawing)
  uw graph --json <flow.json> [<thing>]
      the whole menu flow for graph_viewer (format uw-flow/1, docs/graph_viewer/DATA.md);
      <thing> (optional) becomes the starting point, otherwise the game's own starts
  uw browse [--atlas <atlas.json>] [--tr <languages/tr dir>] [search]
      look around Native in a window: type to search, Enter to follow who uses what
  uw explain [--all] [--atlas <atlas.json>] [--tr <languages/tr dir>] <thing>
      <thing>: a name or part of one (praven, p_town_6, mnu_camp, merchant), a global
      ($g_player_luck), an operation (call_script) or a constant
      --tr: the official Turkish csv folder (Modules/Native/languages/tr), to show the
      Turkish in-game text too (the default install is used when it is not given)
  uw words [folder] [--json] [--atlas <atlas.json>]
      every name a .uw file may use (game names, lib/fns words, object fields, keywords),
      one per line: --json gives {\"label\",\"kind\",\"detail\"} (kind: name|word|field|keyword)
  uw roundtrip [--atlas <atlas.json>] [--out <dir>] [--failures N] [--schemas <dir>] [--names]
      decompile every Native operation block to UW, compile it back and compare;
      prints the share that comes back identical, writes the text to target/uw-native;
      --schemas: the record schema folder (kit-meta/records when not given): every record is
      printed and parsed back too, and written to target/uw-native/records;
      --names: print the game names (`praven`) instead of the kit references (`p_town_6`)
  uw new <dir> --from native [--atlas <atlas.json>]
      write every Native record into a project folder as UW text (an export; not compiled yet, see P3b)
  uw new --check <dir>
      read such a folder back: how many records are still exactly the atlas value
  uw build-project <dir> [--compare-native] [--no-install]
      write the project folder's whole tables, build them with the kit and install the mod;
      --compare-native: say how many output files are byte-identical to Native
  uw build-project --native-check
      the same, but the modules come from the atlas values themselves
  uw rename <record> \"<new name>\" [--list] [--yes]
      rename a record and every text that mentions it: its own name, the project's other record
      texts and its languages/<lang>/*.csv translations (the old name is replaced inside each
      text, so \"Praven Şehri\" becomes \"Adam Siken Şehri\");
      --list: only list what would change; --yes: write every group without asking
  uw rename <dir> <record> \"<new name>\" ...
      the same, for the project folder <dir>
  uw --version";

/// Where the atlas lives unless `--atlas` / `UW_ATLAS` says otherwise (docs/DEVELOPMENT.md).
pub const ATLAS_REL: &str = "target/uw-staging/atlas.json";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--version") | Some("-V") => {
            println!("uw {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some("explain") => cmd_explain(&args[1..]),
        Some("browse") => cmd_browse(&args[1..]),
        Some("create") => cmd_create(&args[1..]),
        Some("check") => check::cmd(&args[1..]),
        Some("build") => cmd_build_any(&args[1..]),
        Some("graph") => cmd_graph(&args[1..]),
        Some("roundtrip") => roundtrip::cmd(&args[1..]),
        Some("new") => new::cmd(&args[1..]),
        Some("build-project") => build_project::cmd(&args[1..]),
        Some("rename") => rename::cmd(&args[1..]),
        Some("words") => words::cmd(&args[1..]),
        _ => {
            println!("uw {}", env!("CARGO_PKG_VERSION"));
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
    }
}

struct Common {
    all: bool,
    atlas: Option<PathBuf>,
    tr: Option<PathBuf>,
    words: Vec<String>,
}

fn parse(args: &[String]) -> Result<Common, String> {
    let mut common = Common { all: false, atlas: None, tr: None, words: Vec::new() };
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--all" => common.all = true,
            "--atlas" => match it.next() {
                Some(path) => common.atlas = Some(PathBuf::from(path)),
                None => return Err("--atlas needs a path".into()),
            },
            "--tr" => match it.next() {
                Some(path) => common.tr = Some(PathBuf::from(path)),
                None => return Err("--tr needs a path".into()),
            },
            _ => common.words.push(arg.clone()),
        }
    }
    Ok(common)
}

/// The atlas file: `--atlas`, else `UW_ATLAS`, else the one found in this or a parent folder.
fn atlas_path(path: Option<PathBuf>) -> Result<PathBuf, String> {
    path.or_else(|| std::env::var_os("UW_ATLAS").map(PathBuf::from))
        .or_else(find_atlas)
        .ok_or_else(|| {
            format!(
                "no atlas found ({ATLAS_REL} in this or a parent folder); \
                 build it as described in docs/DEVELOPMENT.md (Atlas)"
            )
        })
}

pub fn load(path: Option<PathBuf>) -> Result<model::Model, String> {
    let path = atlas_path(path)?;
    let atlas = uw_atlas::Atlas::from_path(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    Ok(model::Model::new(atlas))
}

/// [`load`] with the game names installed before the model is built (rows keep their labels): the
/// working folder's own `names.uw` when it has one, otherwise the derived Native names. A bad
/// `names.uw` is an error, never a silent fallback (same rule as `uw build`).
fn load_named(path: Option<PathBuf>) -> Result<model::Model, String> {
    let path = atlas_path(path)?;
    let mut atlas = uw_atlas::Atlas::from_path(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    let dir = std::env::current_dir().map_err(|err| format!("current folder: {err}"))?;
    if dir.join(uw_decomp::names::FILE).is_file() {
        new::install_names(&mut atlas, &dir)?;
    } else {
        let names = uw_decomp::names::derive(&atlas);
        atlas.set_game_names(&uw_decomp::names::pairs(&names));
    }
    Ok(model::Model::new(atlas))
}

/// Adds the official Turkish texts: from `dir` (`--tr`), else the default install when there is one.
/// A machine without the translation keeps the English-only model (no error).
fn add_tr(model: model::Model, dir: Option<PathBuf>) -> Result<model::Model, String> {
    match dir {
        Some(dir) => tr::load_dir(&dir)
            .map(|map| model.with_tr(map))
            .map_err(|err| format!("{}: {err}", dir.display())),
        None => {
            let Some(dir) = tr::default_dir() else { return Ok(model) };
            match tr::load_dir(&dir) {
                Ok(map) => Ok(model.with_tr(map)),
                // The folder can disappear between the check and the read: English only then.
                Err(_) => Ok(model),
            }
        }
    }
}

fn cmd_explain(args: &[String]) -> ExitCode {
    let common = match parse(args) {
        Ok(common) => common,
        Err(message) => return fail(&message),
    };
    if common.words.is_empty() {
        return fail(&format!("what should I explain?\n{USAGE}"));
    }
    let thing = common.words.join(" ");
    let model = match load_named(common.atlas).and_then(|model| add_tr(model, common.tr)) {
        Ok(model) => model,
        Err(message) => return fail(&message),
    };
    match explain::explain(&model, &thing, common.all) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(message) => fail(&message),
    }
}

fn cmd_browse(args: &[String]) -> ExitCode {
    let common = match parse(args) {
        Ok(common) => common,
        Err(message) => return fail(&message),
    };
    let model = match load_named(common.atlas).and_then(|model| add_tr(model, common.tr)) {
        Ok(model) => model,
        Err(message) => return fail(&message),
    };
    match tui::run(&model, &common.words.join(" ")) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => fail(&format!("terminal: {err}")),
    }
}

fn cmd_build(args: &[String]) -> ExitCode {
    let mut opts = build::Options {
        dir: PathBuf::new(),
        atlas: None,
        kit: None,
        python: None,
        modules: None,
        install: true,
        debug: false,
    };
    let mut dir = None;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        let mut value = |flag: &str| it.next().map(PathBuf::from).ok_or(format!("{flag} needs a path"));
        let result = match arg.as_str() {
            "--debug" => {
                opts.debug = true;
                Ok(())
            }
            "--no-install" => {
                opts.install = false;
                Ok(())
            }
            "--atlas" => value("--atlas").map(|p| opts.atlas = Some(p)),
            "--kit" => value("--kit").map(|p| opts.kit = Some(p)),
            "--python" => value("--python").map(|p| opts.python = Some(p)),
            "--modules" => value("--modules").map(|p| opts.modules = Some(p)),
            _ if dir.is_none() => {
                dir = Some(PathBuf::from(arg));
                Ok(())
            }
            _ => Err(format!("unexpected argument '{arg}'\n{USAGE}")),
        };
        if let Err(message) = result {
            return fail(&message);
        }
    }
    let Some(dir) = dir else {
        return fail(&format!("which mod folder should I build?\n{USAGE}"));
    };
    opts.dir = dir;
    match build::run(opts) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => fail(&message),
    }
}

fn cmd_graph(args: &[String]) -> ExitCode {
    let mut depth = 4usize;
    let mut mermaid = false;
    let mut html: Option<PathBuf> = None;
    let mut json: Option<PathBuf> = None;
    let mut atlas = None;
    let mut tr_dir = None;
    let mut words = Vec::new();
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--depth" => match it.next().and_then(|d| d.parse().ok()) {
                Some(d) => depth = d,
                None => return fail("--depth needs a number"),
            },
            "--mermaid" => mermaid = true,
            "--html" => match it.next() {
                Some(p) => html = Some(PathBuf::from(p)),
                None => return fail("--html needs a file name"),
            },
            "--json" => match it.next() {
                Some(p) => json = Some(PathBuf::from(p)),
                None => return fail("--json needs a file name (for example flow.json)"),
            },
            "--atlas" => atlas = it.next().map(PathBuf::from),
            "--tr" => tr_dir = it.next().map(PathBuf::from),
            _ => words.push(arg.clone()),
        }
    }
    if let Some(path) = json {
        return graph_json(atlas, tr_dir, &words, &path);
    }
    if words.is_empty() {
        return fail(&format!("what should I draw?\n{USAGE}"));
    }
    let model = match load_named(atlas).and_then(|model| add_tr(model, tr_dir)) {
        Ok(model) => model,
        Err(message) => return fail(&message),
    };
    let query = words.join(" ");
    let Some(root) = model.resolve(&query) else {
        // Not exact: the same match list as `uw explain`.
        return match explain::explain(&model, &query, false) {
            Ok(text) => {
                print!("{text}");
                ExitCode::SUCCESS
            }
            Err(message) => fail(&message),
        };
    };
    const MAX_NODES: usize = 120;
    if let Some(path) = html {
        let page = graph_export::html(&model, &root, depth, MAX_NODES);
        if let Err(err) = std::fs::write(&path, page) {
            return fail(&format!("{}: {err}", path.display()));
        }
        println!("wrote {} (open it in a browser)", path.display());
    } else if mermaid {
        print!("{}", graph_export::mermaid(&model, &root, depth, MAX_NODES));
    } else {
        print!("{}", graph::render_text(&graph::tree(&model, &root, depth)));
    }
    ExitCode::SUCCESS
}

/// `uw graph --json <file> [<thing>]`: writes the uw-flow/1 document for graph_viewer.
fn graph_json(atlas: Option<PathBuf>, tr_dir: Option<PathBuf>, words: &[String], path: &Path) -> ExitCode {
    let model = match load_named(atlas).and_then(|model| add_tr(model, tr_dir)) {
        Ok(model) => model,
        Err(message) => return fail(&message),
    };
    let mut roots = Vec::new();
    if !words.is_empty() {
        let query = words.join(" ");
        // A bare menu name (`start_phase_2`) is accepted too: the viewer only starts from menus.
        let found = model.resolve(&query).or_else(|| model.resolve(&format!("menu_{query}")));
        let Some(root) = found else {
            return fail(&format!("no single thing is called \"{query}\"; try `uw explain {query}`"));
        };
        let id = model.label(&root);
        roots.push(flow_json::Root { id, why: "chosen on the command line".into() });
    }
    // descriptions written by py/describe.py, found like the atlas (this or a parent folder)
    let desc = std::env::current_dir()
        .ok()
        .and_then(|cwd| cwd.ancestors().map(|d| d.join("kit-meta/aciklama")).find(|p| p.is_dir()))
        .map(|dir| flow_json::Descriptions::load(&dir))
        .unwrap_or_default();
    let doc = flow_json::flow(&model.atlas, roots, "uw graph --json", &desc);
    let missing: Vec<&str> = doc["roots"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|r| r["id"].as_str())
        .filter(|id| !doc["nodes"].as_array().is_some_and(|ns| ns.iter().any(|n| n["id"] == *id)))
        .collect();
    if !missing.is_empty() {
        return fail(&format!(
            "{} is not on the menu flow (graph_viewer starts from menus); try a menu_... name",
            missing.join(", ")
        ));
    }
    let text = match serde_json::to_string_pretty(&doc) {
        Ok(text) => text,
        Err(err) => return fail(&err.to_string()),
    };
    if let Err(err) = std::fs::write(path, text + "
") {
        return fail(&format!("{}: {err}", path.display()));
    }
    println!("{} -> {} (open it in graph_viewer)", flow_json::summary(&doc), path.display());
    ExitCode::SUCCESS
}

/// The UW folder (atlas, kit-meta, kit): an ancestor of the working folder that has `kit-meta/`,
/// else the folder this `uw` was built from. Lets `uw create` / `uw build` run from any folder.
fn uw_home() -> Option<PathBuf> {
    let has_meta = |dir: &Path| dir.join("kit-meta").is_dir();
    if let Some(found) = std::env::current_dir().ok().and_then(|cwd| cwd.ancestors().find(|d| has_meta(d)).map(Path::to_path_buf)) {
        return Some(found);
    }
    let built_from = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    has_meta(&built_from).then(|| plain(built_from.canonicalize().unwrap_or(built_from)))
}

/// A Windows `\\?\` long-path prefix removed (canonicalize adds it; it only confuses readers).
pub(crate) fn plain(path: PathBuf) -> PathBuf {
    match path.to_str().and_then(|text| text.strip_prefix(r"\\?\")) {
        Some(rest) => PathBuf::from(rest),
        None => path,
    }
}

/// `dir` made absolute against the working folder, then the working folder moved to [`uw_home`]
/// (every lookup — atlas, schemas, kit, `target/` — starts there).
pub(crate) fn enter_home(dir: &Path) -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let absolute = if dir.is_absolute() { dir.to_path_buf() } else { cwd.join(dir) };
    let home = uw_home().ok_or("the UW folder (with kit-meta/) was not found; run uw from inside it once")?;
    std::env::set_current_dir(&home).map_err(|e| format!("{}: {e}", home.display()))?;
    Ok(absolute)
}

/// `uw create <name>`: a new mod folder `<name>` here, holding all of Native as UW text + names.uw.
fn cmd_create(args: &[String]) -> ExitCode {
    let [name] = args else {
        return fail("usage: uw create <mod name>   (makes the folder <mod name> in this folder)");
    };
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return fail(&format!("mod name '{name}' may only use letters, digits and _"));
    }
    let dir = match enter_home(Path::new(name)) {
        Ok(dir) => dir,
        Err(message) => return fail(&message),
    };
    let code = new::cmd(&[dir.display().to_string(), "--from".into(), "native".into()]);
    if code == ExitCode::SUCCESS {
        println!("next:  cd {name}   then   uw build");
    }
    code
}

/// `uw build [dir] [flags]`: no folder = this folder. A project folder (made by `uw create` /
/// `uw new`, it has uw.lock) builds with `build-project`; any other folder is a `uw build` mod.
fn cmd_build_any(args: &[String]) -> ExitCode {
    let mut rest: Vec<String> = args.to_vec();
    // The folder is the first word that is not a flag nor a flag's value.
    let mut given = None;
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "--atlas" | "--kit" | "--python" | "--modules" => i += 2,
            flag if flag.starts_with('-') => i += 1,
            _ => {
                given = Some(i);
                break;
            }
        }
    }
    let dir = match given {
        Some(i) => PathBuf::from(rest.remove(i)),
        None => PathBuf::from("."),
    };
    let dir = match enter_home(&dir) {
        Ok(dir) => dir,
        Err(message) => return fail(&message),
    };
    let dir = dir.canonicalize().unwrap_or(dir);
    let dir_text = dir.display().to_string();
    let dir_text = dir_text.strip_prefix(r"\\?\").unwrap_or(&dir_text).to_string();
    let mut forwarded = vec![dir_text];
    forwarded.extend(rest);
    if dir.join("uw.lock").is_file() {
        build_project::cmd(&forwarded)
    } else {
        cmd_build(&forwarded)
    }
}

pub(crate) fn find_atlas() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    cwd.ancestors()
        .map(|dir: &Path| dir.join(ATLAS_REL))
        .find(|path| path.is_file())
}

pub(crate) fn fail(message: &str) -> ExitCode {
    eprintln!("error: {message}");
    ExitCode::FAILURE
}
