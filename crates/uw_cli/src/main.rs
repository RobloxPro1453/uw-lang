//! The `uw` command line tool.

mod explain;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const USAGE: &str = "usage:
  uw --version
  uw explain [--all] [--atlas <atlas.json>] <thing>
      <thing>: an id (p_town_6, script_game_start, itm_x#1), a code reference
      (mnu_camp), a global ($g_player_luck), an operation (call_script) or a constant";

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
        _ => {
            println!("uw {}", env!("CARGO_PKG_VERSION"));
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
    }
}

fn cmd_explain(args: &[String]) -> ExitCode {
    let mut all = false;
    let mut atlas_path: Option<PathBuf> = None;
    let mut thing: Option<&str> = None;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--all" => all = true,
            "--atlas" => match it.next() {
                Some(path) => atlas_path = Some(PathBuf::from(path)),
                None => return fail("--atlas needs a path"),
            },
            _ if thing.is_none() => thing = Some(arg),
            _ => return fail(&format!("unexpected argument '{arg}'\n{USAGE}")),
        }
    }
    let Some(thing) = thing else {
        return fail(&format!("what should I explain?\n{USAGE}"));
    };
    let path = atlas_path
        .or_else(|| std::env::var_os("UW_ATLAS").map(PathBuf::from))
        .or_else(find_atlas);
    let Some(path) = path else {
        return fail(&format!(
            "no atlas found ({ATLAS_REL} in this or a parent folder); \
             build it as described in docs/DEVELOPMENT.md (Atlas)"
        ));
    };
    let atlas = match uw_atlas::Atlas::from_path(&path) {
        Ok(atlas) => atlas,
        Err(err) => return fail(&format!("{}: {err}", path.display())),
    };
    match explain::explain(&atlas, thing, all) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(message) => fail(&message),
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
