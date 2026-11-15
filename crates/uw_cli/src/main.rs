//! The `uw` command line tool.

mod build;
mod explain;
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
  uw graph <thing> [--depth N] [--mermaid] [--html <file.html>]
      what <thing> leads to, as a tree (or a Mermaid chart / an HTML page with the drawing)
  uw browse [--atlas <atlas.json>] [search]
      look around Native in a window: type to search, Enter to follow who uses what
  uw explain [--all] [--atlas <atlas.json>] <thing>
      <thing>: a name or part of one (praven, p_town_6, mnu_camp, merchant), a global
      ($g_player_luck), an operation (call_script) or a constant
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
    words: Vec<String>,
}

fn parse(args: &[String]) -> Result<Common, String> {
    let mut common = Common { all: false, atlas: None, words: Vec::new() };
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--all" => common.all = true,
            "--atlas" => match it.next() {
                Some(path) => common.atlas = Some(PathBuf::from(path)),
                None => return Err("--atlas needs a path".into()),
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

fn cmd_explain(args: &[String]) -> ExitCode {
    let common = match parse(args) {
        Ok(common) => common,
        Err(message) => return fail(&message),
    };
    if common.words.is_empty() {
        return fail(&format!("what should I explain?\n{USAGE}"));
    }
    let thing = common.words.join(" ");
    let model = match load(common.atlas) {
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
    let model = match load(common.atlas) {
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
    let mut atlas = None;
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
            "--atlas" => atlas = it.next().map(PathBuf::from),
            _ => words.push(arg.clone()),
        }
    }
    if words.is_empty() {
        return fail(&format!("what should I draw?\n{USAGE}"));
    }
    let model = match load(atlas) {
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

fn find_atlas() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    cwd.ancestors()
        .map(|dir: &Path| dir.join(ATLAS_REL))
        .find(|path| path.is_file())
}

fn fail(message: &str) -> ExitCode {
    eprintln!("error: {message}");
    ExitCode::FAILURE
}
