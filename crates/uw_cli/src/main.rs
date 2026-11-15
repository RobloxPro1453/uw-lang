//! The `uw` command line tool.

mod explain;
mod model;
mod tui;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const USAGE: &str = "usage:
  uw browse [--atlas <atlas.json>] [search]
      look around Native in a window: type to search, Enter to follow who uses what
  uw explain [--all] [--atlas <atlas.json>] <thing>
      <thing>: a name or part of one (praven, p_town_6, mnu_camp, merchant), a global
      ($g_player_luck), an operation (call_script) or a constant
  uw --version";

/// Where the atlas lives unless `--atlas` / `UW_ATLAS` says otherwise (docs/DEVELOPMENT.md).
const ATLAS_REL: &str = "target/uw-staging/atlas.json";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--version") | Some("-V") => {
            println!("uw {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some("explain") => cmd_explain(&args[1..]),
        Some("browse") => cmd_browse(&args[1..]),
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

fn load(path: Option<PathBuf>) -> Result<model::Model, String> {
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
