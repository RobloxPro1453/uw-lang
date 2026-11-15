//! Diagnostics: error catalog (codes like E0001, messages in English), `Span`, line/column rendering, did-you-mean suggestions, `.uwmap` types.
//!
//! Everything here is pure: no I/O, no dependencies. A diagnostic points into a `SourceFile`
//! with byte offsets and `render` turns it into the compiler's error text:
//!
//! ```text
//! error[E0001]: unknown name 'pravn'
//!  --> start.uw:3:22
//!   |
//! 3 |   dialogue:  praven, pravn
//!   |                      ^^^^^
//!   |
//!   = help: did you mean 'praven'?
//! ```

/// A byte range in a source file, `start` inclusive, `end` exclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// A named source file; offsets in a `Diagnostic` are byte offsets into `text`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    pub name: String,
    pub text: String,
}

impl SourceFile {
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        SourceFile {
            name: name.into(),
            text: text.into(),
        }
    }

    /// 1-based `(line, column)` of a byte offset. The column counts chars, so UTF-8 text
    /// has one column per visible character. Offsets past the end map to the end position.
    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        let offset = offset.min(self.text.len());
        let mut line = 1;
        let mut col = 1;
        for (index, ch) in self.text.char_indices() {
            if index >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }

    /// Text of a 1-based line, without the line terminator; `""` when out of range.
    fn line_text(&self, line: usize) -> &str {
        self.text
            .split('\n')
            .nth(line.saturating_sub(1))
            .unwrap_or("")
            .trim_end_matches('\r')
    }
}

/// One compiler error: code, English message, the span it points at, extra notes and help.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub message: String,
    pub span: Span,
    pub notes: Vec<String>,
    pub help: Option<String>,
}

impl Diagnostic {
    /// Renders the error as the compiler prints it (no trailing newline).
    pub fn render(&self, file: &SourceFile) -> String {
        let (line, col) = file.line_col(self.span.start);
        let text = file.line_text(line);
        let gutter = line.to_string().len();

        let mut out = Vec::new();
        out.push(format!("error[{}]: {}", self.code, self.message));
        out.push(format!(
            "{:w$}--> {}:{}:{}",
            "",
            file.name,
            line,
            col,
            w = gutter
        ));
        out.push(format!("{:w$} |", "", w = gutter));
        out.push(format!("{:>w$} | {}", line, text, w = gutter));

        let (end_line, end_col) = file.line_col(self.span.end);
        let chars_in_line = text.chars().count();
        let mut carets = if end_line == line {
            end_col.saturating_sub(col)
        } else {
            chars_in_line.saturating_sub(col - 1)
        };
        if carets == 0 {
            carets = 1;
        }
        out.push(format!(
            "{:w$} | {}{}",
            "",
            " ".repeat(col - 1),
            "^".repeat(carets),
            w = gutter
        ));

        if self.help.is_some() || !self.notes.is_empty() {
            out.push(format!("{:w$} |", "", w = gutter));
        }
        if let Some(help) = &self.help {
            out.push(format!("{:w$} = help: {}", "", help, w = gutter));
        }
        for note in &self.notes {
            out.push(format!("{:w$} = note: {}", "", note, w = gutter));
        }

        out.join("\n")
    }
}

/// Code catalog (messages are English).
pub const E0001_UNKNOWN_NAME: &str = "E0001";

/// Closest candidate to `name` by Damerau-Levenshtein distance, if it is close enough
/// (`distance <= max(1, name.len() / 3)`). Ties keep the earlier candidate. Pure function.
pub fn suggest<'a>(name: &str, candidates: impl IntoIterator<Item = &'a str>) -> Option<String> {
    let limit = std::cmp::max(1, name.len() / 3);
    let mut best: Option<(usize, &'a str)> = None;
    for candidate in candidates {
        let distance = edit_distance(name, candidate);
        if distance > limit {
            continue;
        }
        match best {
            // Keep the earlier candidate on a tie.
            Some((best_distance, _)) if best_distance <= distance => {}
            _ => best = Some((distance, candidate)),
        }
    }
    best.map(|(_, candidate)| candidate.to_string())
}

/// Damerau-Levenshtein distance (optimal string alignment) over chars.
fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (n, m) = (a.len(), b.len());
    let mut d = vec![vec![0usize; m + 1]; n + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = i;
    }
    for j in 0..=m {
        d[0][j] = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let mut best = (d[i - 1][j] + 1)
                .min(d[i][j - 1] + 1)
                .min(d[i - 1][j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                best = best.min(d[i - 2][j - 2] + 1);
            }
            d[i][j] = best;
        }
    }
    d[n][m]
}

/// `E0001` for a name that is not defined; `help` comes from the closest candidate, if any.
pub fn unknown_name<'a>(
    name: &str,
    span: Span,
    candidates: impl IntoIterator<Item = &'a str>,
) -> Diagnostic {
    Diagnostic {
        code: E0001_UNKNOWN_NAME,
        message: format!("unknown name '{name}'"),
        span,
        notes: Vec::new(),
        help: suggest(name, candidates).map(|candidate| format!("did you mean '{candidate}'?")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_col_counts_chars_on_later_lines() {
        let file = SourceFile::new("start.uw", "ab\nçğü praven\nxyz");
        assert_eq!(file.line_col(0), (1, 1));
        // "ab\n" is 3 bytes; the non-ASCII prefix "çğü " is 7 bytes / 4 chars.
        assert_eq!(file.line_col(10), (2, 5));
        assert_eq!(file.line_col(16), (2, 11)); // one past "praven"
        assert_eq!(file.line_col(17), (3, 1));
        assert_eq!(file.line_col(file.text.len()), (3, 4));
        assert_eq!(file.line_col(9999), (3, 4));
    }

    #[test]
    fn render_matches_golden_output() {
        let text = "// start.uw\nmission start\n  dialogue:  praven, pravn\n";
        let start = text.find("pravn").unwrap();
        let file = SourceFile::new("start.uw", text);
        let diag = unknown_name(
            "pravn",
            Span {
                start,
                end: start + "pravn".len(),
            },
            ["praven", "paris"],
        );
        let expected = "\
error[E0001]: unknown name 'pravn'
 --> start.uw:3:22
  |
3 |   dialogue:  praven, pravn
  |                      ^^^^^
  |
  = help: did you mean 'praven'?";
        assert_eq!(diag.render(&file), expected);
    }

    #[test]
    fn render_uses_wide_gutter_and_prints_notes() {
        let mut text = String::new();
        for i in 1..=9 {
            text.push_str(&format!("// line {i}\n"));
        }
        text.push_str("talk prasn");
        let start = text.find("prasn").unwrap();
        let file = SourceFile::new("a.uw", text);
        let diag = Diagnostic {
            code: E0001_UNKNOWN_NAME,
            message: "unknown name 'prasn'".to_string(),
            span: Span {
                start,
                end: start + "prasn".len(),
            },
            notes: vec!["declared names: praven".to_string()],
            help: None,
        };
        let expected = "\
error[E0001]: unknown name 'prasn'
  --> a.uw:10:6
   |
10 | talk prasn
   |      ^^^^^
   |
   = note: declared names: praven";
        assert_eq!(diag.render(&file), expected);
    }

    #[test]
    fn suggest_finds_the_close_candidate() {
        assert_eq!(suggest("pravn", ["praven", "paris"]), Some("praven".into()));
    }

    #[test]
    fn suggest_rejects_far_candidates() {
        assert_eq!(suggest("xyz", ["praven"]), None);
    }

    #[test]
    fn suggest_respects_the_threshold_for_longer_names() {
        // len 9 -> limit 3: distance 3 is accepted, distance 4 is not.
        assert_eq!(suggest("abcdefghi", ["abcdefxyz"]), Some("abcdefxyz".into()));
        assert_eq!(suggest("abcdefghi", ["abcwxyzuv"]), None);
        // len 1 -> limit 1.
        assert_eq!(suggest("a", ["b"]), Some("b".into()));
    }

    #[test]
    fn suggest_breaks_ties_by_candidate_order() {
        assert_eq!(suggest("cat", ["cot", "cut"]), Some("cot".into()));
        assert_eq!(suggest("cat", ["cut", "cot"]), Some("cut".into()));
    }

    #[test]
    fn suggest_counts_a_transposition_as_one_edit() {
        assert_eq!(suggest("paris", ["apris"]), Some("apris".into()));
    }
}
