//! The `uw` command line tool.

mod build;
mod explain;
mod flow_json;
mod roundtrip;
mod graph;
mod graph_export;
mod impact;
mod model;
mod tr;
mod tui;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const USAGE: &str = "usage:
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
  uw roundtrip [--atlas <atlas.json>] [--out <dir>] [--failures N]
      decompile every Native operation block to UW, compile it back and compare;
      prints the share that comes back identical, writes the text to target/uw-native
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
        Some("build") => cmd_build(&args[1..]),
        Some("graph") => cmd_graph(&args[1..]),
        Some("roundtrip") => roundtrip::cmd(&args[1..]),
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

pub fn load(path: Option<PathBuf>) -> Result<model::Model, String> {
    let path = path
        .or_else(|| std::env::var_os("UW_ATLAS").map(PathBuf::from))
        .or_else(find_atlas)
        .ok_or_else(|| {
            format!(
                "no atlas found ({ATLAS_REL} in this or a parent folder); \
                 build it as described in docs/DEVELOPMENT.md (Atlas)"
            )
        })?;
    let atlas = uw_atlas::Atlas::from_path(&path).map_err(|err| format!("{}: {err}", path.display()))?;
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
    let model = match load(common.atlas).and_then(|model| add_tr(model, common.tr)) {
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
    let model = match load(common.atlas).and_then(|model| add_tr(model, common.tr)) {
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
    let model = match load(atlas).and_then(|model| add_tr(model, tr_dir)) {
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
    let model = match load(atlas).and_then(|model| add_tr(model, tr_dir)) {
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
