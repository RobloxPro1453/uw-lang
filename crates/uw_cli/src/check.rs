//! `uw check [folder] [--json]`: parse and check a mod folder or a `uw create` project without
//! running the kit. `--json` prints one object per error and a summary line last (T-100, T-101).

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::build::{CheckFailure, Problem};
use crate::build_project::LoadFailure;

const USAGE: &str = "usage:
  uw check [folder] [--json] [--atlas <atlas.json>]
      parse and check a mod folder or a `uw create` project without building;
      --json: one JSON object per error, then {\"summary\":{\"errors\":N,\"warnings\":M}}";

/// One error as `--json` prints it (1-based line and column, end exclusive).
struct Found {
    file: String,
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
    code: String,
    message: String,
    help: String,
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
    if dir.join("uw.lock").is_file() {
        check_project(&dir, atlas, json)
    } else {
        check_folder(&dir, atlas, json)
    }
}

fn check_folder(dir: &Path, atlas: Option<PathBuf>, json: bool) -> ExitCode {
    let (dir, paths) = match crate::build::folder(dir) {
        Ok(found) => found,
        Err(message) => return failed(&message, json),
    };
    match crate::build::check_sources(&dir, &paths, atlas) {
        Ok(_) => {
            summary(json, 0);
            ExitCode::SUCCESS
        }
        Err(CheckFailure::Message(message)) => failed(&message, json),
        Err(CheckFailure::Parse(problems)) | Err(CheckFailure::Errors(problems)) => {
            if json {
                for p in &problems {
                    emit(found(&p.path, &p.file, p));
                }
                summary(true, problems.len());
            } else {
                for p in &problems {
                    eprintln!("{}\n", p.diag.render(&p.file));
                }
            }
            ExitCode::FAILURE
        }
    }
}

/// A project's errors carry their file and (after the record splitter keeps the line) their line;
/// record errors that name no position stay at the top of their file.
fn check_project(dir: &Path, atlas: Option<PathBuf>, json: bool) -> ExitCode {
    let (atlas, schemas) = match crate::build_project::project_atlas(dir, atlas) {
        Ok(found) => found,
        Err(message) => return failed(&message, json),
    };
    match crate::build_project::load_records(dir, &atlas, &schemas) {
        Ok(_) => {
            summary(json, 0);
            ExitCode::SUCCESS
        }
        Err(LoadFailure::Words(message)) => failed(&message, json),
        Err(LoadFailure::Records(errors)) => {
            if json {
                for error in &errors {
                    emit(project_problem(dir, error));
                }
                summary(true, errors.len());
            } else {
                for error in &errors {
                    eprintln!("  {error}");
                }
            }
            ExitCode::FAILURE
        }
    }
}

fn found(path: &Path, file: &uw_diag::SourceFile, p: &Problem) -> Found {
    let (line, column) = file.line_col(p.diag.span.start);
    let (end_line, end_column) = file.line_col(p.diag.span.end);
    Found {
        file: path.display().to_string(),
        line,
        column,
        end_line,
        end_column,
        code: p.diag.code.to_string(),
        message: p.diag.message.clone(),
        help: p.diag.help.clone().unwrap_or_default(),
    }
}

fn emit(p: Found) {
    println!(
        "{}",
        serde_json::json!({
            "file": p.file,
            "line": p.line,
            "column": p.column,
            "end_line": p.end_line,
            "end_column": p.end_column,
            "severity": "error",
            "code": p.code,
            "message": p.message,
            "help": p.help,
        })
    );
}

fn summary(json: bool, errors: usize) {
    if json {
        println!("{}", serde_json::json!({"summary": {"errors": errors, "warnings": 0}}));
    }
}

fn failed(message: &str, json: bool) -> ExitCode {
    eprintln!("error: {message}");
    summary(json, 1);
    ExitCode::FAILURE
}

/// One project error line (`<file>: line N[:C]: error[CODE]: message`) as the JSON object.
fn project_problem(dir: &Path, error: &str) -> Found {
    let files = uw_files(dir);
    let (file, rest) = match files.iter().find_map(|file| {
        error.strip_prefix(&format!("{}: ", file.display())).map(|rest| (file.clone(), rest))
    }) {
        Some(found) => found,
        // Errors that name two files (a duplicate id) point at the first one.
        None => match files.iter().find(|file| error.contains(&file.display().to_string())) {
            Some(file) => (file.clone(), error),
            None => (dir.join("uw.lock"), error),
        },
    };
    let (line, column, rest) = project_line(rest);
    let (code, message) = project_code(rest);
    Found {
        file: file.display().to_string(),
        line,
        column,
        end_line: line,
        end_column: column,
        code,
        message,
        help: String::new(),
    }
}

/// `line N: message` or `line N:C: message`; anything else has no position.
fn project_line(rest: &str) -> (usize, usize, &str) {
    let Some(after) = rest.strip_prefix("line ") else { return (1, 1, rest) };
    let Some((head, tail)) = after.split_once(':') else { return (1, 1, rest) };
    let Ok(line) = head.parse::<usize>() else { return (1, 1, rest) };
    if let Some((column, tail)) = tail.split_once(':') {
        if !column.is_empty() && column.chars().all(|c| c.is_ascii_digit()) {
            return (line, column.parse().unwrap_or(1), tail.trim_start());
        }
    }
    (line, 1, tail.trim_start())
}

fn project_code(rest: &str) -> (String, String) {
    let Some(after) = rest.strip_prefix("error[") else { return (String::new(), rest.to_string()) };
    let Some((code, message)) = after.split_once("]: ") else { return (String::new(), rest.to_string()) };
    (code.to_string(), message.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_project_error_line_splits_into_position_and_code() {
        assert_eq!(project_line("line 4: expected `}`"), (4, 1, "expected `}`"));
        assert_eq!(project_line("line 4:9: error[E0001]: unknown name"), (4, 9, "error[E0001]: unknown name"));
        assert_eq!(project_code("error[E0001]: unknown name"), ("E0001".to_string(), "unknown name".to_string()));
        let plain = "factions x: kayıt silinmiş";
        assert_eq!(project_line(plain), (1, 1, plain));
        assert_eq!(project_code(plain), (String::new(), plain.to_string()));
    }
}

/// Every `.uw` file under `dir`, sorted.
fn uw_files(dir: &Path) -> Vec<PathBuf> {
    fn walk(dir: &Path, files: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, files);
            } else if path.extension().is_some_and(|ext| ext == "uw") {
                files.push(path);
            }
        }
    }
    let mut files = Vec::new();
    walk(dir, &mut files);
    files.sort();
    files
}
