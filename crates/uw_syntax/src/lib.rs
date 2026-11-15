//! UW source text -> syntax tree (Faz 1 subset). Braces for blocks, newline or `;` between
//! statements, `//` `/* */` `#` comments, Native operations called as `name(a, b)`.
//! Decision and its error-message duties: `.evrim/otonom/KARARNAMELER.md` (Sözdizimi, 2026-11-15).
//!
//! ```text
//! mod praven_start
//!
//! override menu start_phase_2 {
//!     on enter {
//!         party_relocate_near_party(p_main_party, p_town_6, 3)
//!         change_screen_map()
//!     }
//! }
//!
//! on game.start {
//!     assign($g_x, 1)
//! }
//! ```
//!
//! Pasted code is welcome: tabs, CRLF, no-break spaces and curly quotes read the same as the plain
//! ones. Errors never say only "unexpected end of file": an unclosed `{` is reported at its own line.

use uw_diag::{Diagnostic, Span};

/// Syntax error codes (messages are English).
pub const E0100_UNEXPECTED: &str = "E0100";
pub const E0101_UNCLOSED: &str = "E0101";
pub const E0102_STRAY_CLOSE: &str = "E0102";
pub const E0103_BAD_TOKEN: &str = "E0103";
pub const E0104_PYTHON_BLOCK: &str = "E0104";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    pub text: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    /// `mod <name>`, if the file has one.
    pub module: Option<Ident>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// `override <kind> <name> { handlers }`
    Override { kind: Ident, name: Ident, handlers: Vec<Handler>, span: Span },
    /// `on a.b { body }`
    On { path: Vec<Ident>, body: Block, span: Span },
}

/// `on <event> { body }` inside an override.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handler {
    pub event: Ident,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// A Native operation call: `name(args)` (or a bare `name` for an operation without operands).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stmt {
    pub op: Ident,
    pub args: Vec<Arg>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arg {
    /// A Native name or kit constant: `p_town_6`, `reg0`, `tf_male`.
    Name(Ident),
    /// `$name`, kept with its `$`.
    Global(Ident),
    /// `:name`, kept with its `:`.
    Local(Ident),
    Int(i64, Span),
    Str(String, Span),
}

impl Arg {
    pub fn span(&self) -> Span {
        match self {
            Arg::Name(i) | Arg::Global(i) | Arg::Local(i) => i.span,
            Arg::Int(_, s) | Arg::Str(_, s) => *s,
        }
    }
}

// ---------------------------------------------------------------- lexer

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Ident(String),
    Global(String),
    Local(String),
    Int(i64),
    Str(String),
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comma,
    Semi,
    Dot,
    Colon,
    Newline,
    Eof,
}

#[derive(Debug, Clone)]
struct Token {
    tok: Tok,
    span: Span,
}

fn err(code: &'static str, message: impl Into<String>, span: Span) -> Diagnostic {
    Diagnostic { code, message: message.into(), span, notes: Vec::new(), help: None }
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn lex(text: &str) -> Result<Vec<Token>, Diagnostic> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let end_of = |i: usize| chars.get(i).map_or(text.len(), |&(o, _)| o);
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let (start, c) = chars[i];
        let one = |tok| Token { tok, span: Span { start, end: end_of(i + 1) } };
        match c {
            '\n' => {
                out.push(one(Tok::Newline));
                i += 1;
            }
            // Pasted whitespace of any kind (tabs, CR, no-break space, BOM).
            c if c.is_whitespace() || c == '\u{feff}' => i += 1,
            '#' => {
                while i < chars.len() && chars[i].1 != '\n' {
                    i += 1;
                }
            }
            '/' if chars.get(i + 1).map(|p| p.1) == Some('/') => {
                while i < chars.len() && chars[i].1 != '\n' {
                    i += 1;
                }
            }
            '/' if chars.get(i + 1).map(|p| p.1) == Some('*') => {
                let mut j = i + 2;
                while j + 1 < chars.len() && !(chars[j].1 == '*' && chars[j + 1].1 == '/') {
                    j += 1;
                }
                if j + 1 >= chars.len() {
                    return Err(err(E0101_UNCLOSED, "this comment is never closed", Span { start, end: end_of(i + 2) })
                        .with_help("close it with */"));
                }
                // Keep line breaks inside the comment as statement separators.
                if chars[i..j].iter().any(|p| p.1 == '\n') {
                    out.push(Token { tok: Tok::Newline, span: Span { start, end: start } });
                }
                i = j + 2;
            }
            '{' => {
                out.push(one(Tok::LBrace));
                i += 1;
            }
            '}' => {
                out.push(one(Tok::RBrace));
                i += 1;
            }
            '(' => {
                out.push(one(Tok::LParen));
                i += 1;
            }
            ')' => {
                out.push(one(Tok::RParen));
                i += 1;
            }
            ',' => {
                out.push(one(Tok::Comma));
                i += 1;
            }
            ';' => {
                out.push(one(Tok::Semi));
                i += 1;
            }
            '.' => {
                out.push(one(Tok::Dot));
                i += 1;
            }
            '"' | '\u{201c}' | '\u{201d}' | '\u{201e}' => {
                let mut value = String::new();
                let mut j = i + 1;
                loop {
                    match chars.get(j).map(|p| p.1) {
                        None | Some('\n') => {
                            return Err(err(E0101_UNCLOSED, "this text is never closed", Span { start, end: end_of(i + 1) })
                                .with_help("end it with \" on the same line"));
                        }
                        Some('"' | '\u{201c}' | '\u{201d}') => break,
                        Some('\\') => {
                            match chars.get(j + 1).map(|p| p.1) {
                                Some('n') => value.push('\n'),
                                Some(other @ ('"' | '\\')) => value.push(other),
                                _ => {
                                    return Err(err(
                                        E0103_BAD_TOKEN,
                                        "unknown escape in text",
                                        Span { start: chars[j].0, end: end_of(j + 2) },
                                    )
                                    .with_help("use \\\" for a quote, \\\\ for a backslash, \\n for a new line"));
                                }
                            }
                            j += 2;
                            continue;
                        }
                        Some(ch) => value.push(ch),
                    }
                    j += 1;
                }
                out.push(Token { tok: Tok::Str(value), span: Span { start, end: end_of(j + 1) } });
                i = j + 1;
            }
            '$' | ':' if chars.get(i + 1).is_some_and(|p| is_ident_start(p.1)) => {
                let mut j = i + 1;
                while j < chars.len() && is_ident(chars[j].1) {
                    j += 1;
                }
                let name: String = chars[i..j].iter().map(|p| p.1).collect();
                let tok = if c == '$' { Tok::Global(name) } else { Tok::Local(name) };
                out.push(Token { tok, span: Span { start, end: end_of(j) } });
                i = j;
            }
            ':' => {
                out.push(one(Tok::Colon));
                i += 1;
            }
            c if c.is_ascii_digit() || (c == '-' && chars.get(i + 1).is_some_and(|p| p.1.is_ascii_digit())) => {
                let mut j = i + 1;
                while j < chars.len() && chars[j].1.is_ascii_digit() {
                    j += 1;
                }
                let span = Span { start, end: end_of(j) };
                let digits = &text[span.start..span.end];
                let value = digits
                    .parse::<i64>()
                    .map_err(|_| err(E0103_BAD_TOKEN, format!("number {digits} is too large"), span))?;
                out.push(Token { tok: Tok::Int(value), span });
                i = j;
            }
            c if is_ident_start(c) => {
                let mut j = i;
                while j < chars.len() && is_ident(chars[j].1) {
                    j += 1;
                }
                // `name#2` (the 2nd Native entry with a repeated name) is one name, not a comment.
                if chars.get(j).map(|p| p.1) == Some('#') && chars.get(j + 1).is_some_and(|p| p.1.is_ascii_digit()) {
                    j += 1;
                    while j < chars.len() && chars[j].1.is_ascii_digit() {
                        j += 1;
                    }
                }
                let name: String = chars[i..j].iter().map(|p| p.1).collect();
                out.push(Token { tok: Tok::Ident(name), span: Span { start, end: end_of(j) } });
                i = j;
            }
            other => {
                return Err(err(E0103_BAD_TOKEN, format!("'{other}' is not part of UW"), Span { start, end: end_of(i + 1) }));
            }
        }
    }
    out.push(Token { tok: Tok::Eof, span: Span { start: text.len(), end: text.len() } });
    Ok(out)
}

trait WithHelp {
    fn with_help(self, help: impl Into<String>) -> Self;
    fn with_note(self, note: impl Into<String>) -> Self;
}

impl WithHelp for Diagnostic {
    fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }
    fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
}

// ---------------------------------------------------------------- parser

struct Parser {
    toks: Vec<Token>,
    pos: usize,
    text: String,
}

fn describe(tok: &Tok) -> String {
    match tok {
        Tok::Ident(s) => format!("'{s}'"),
        Tok::Global(s) | Tok::Local(s) => format!("'{s}'"),
        Tok::Int(n) => format!("number {n}"),
        Tok::Str(_) => "a text".into(),
        Tok::LBrace => "'{'".into(),
        Tok::RBrace => "'}'".into(),
        Tok::LParen => "'('".into(),
        Tok::RParen => "')'".into(),
        Tok::Comma => "','".into(),
        Tok::Semi => "';'".into(),
        Tok::Dot => "'.'".into(),
        Tok::Colon => "':'".into(),
        Tok::Newline => "the end of the line".into(),
        Tok::Eof => "the end of the file".into(),
    }
}

impl Parser {
    fn line_of(&self, offset: usize) -> usize {
        self.text[..offset.min(self.text.len())].matches('\n').count() + 1
    }

    fn peek(&self) -> &Token {
        &self.toks[self.pos]
    }

    fn bump(&mut self) -> Token {
        let t = self.toks[self.pos].clone();
        if self.pos + 1 < self.toks.len() {
            self.pos += 1;
        }
        t
    }

    fn skip_separators(&mut self) {
        while matches!(self.peek().tok, Tok::Newline | Tok::Semi) {
            self.bump();
        }
    }

    fn skip_newlines(&mut self) {
        while self.peek().tok == Tok::Newline {
            self.bump();
        }
    }

    fn ident(&mut self, what: &str) -> Result<Ident, Diagnostic> {
        let t = self.peek().clone();
        match t.tok {
            Tok::Ident(text) => {
                self.bump();
                Ok(Ident { text, span: t.span })
            }
            other => Err(err(E0100_UNEXPECTED, format!("expected {what}, found {}", describe(&other)), t.span)),
        }
    }

    /// `{` that opens a block after `head` (e.g. "on enter"). A `:` here is Python style.
    fn open_brace(&mut self, head: &str) -> Result<Span, Diagnostic> {
        self.skip_newlines();
        let t = self.peek().clone();
        match t.tok {
            Tok::LBrace => {
                self.bump();
                Ok(t.span)
            }
            Tok::Colon => Err(err(E0104_PYTHON_BLOCK, "UW blocks use { }, not ':'", t.span)
                .with_help(format!("write `{head} {{` and close the block with `}}`"))),
            other => Err(err(E0100_UNEXPECTED, format!("expected '{{' after `{head}`, found {}", describe(&other)), t.span)
                .with_help(format!("write `{head} {{ ... }}`"))),
        }
    }

    fn unclosed(&self, open: Span, what: &str) -> Diagnostic {
        let eof = self.toks.last().map_or(open, |t| t.span);
        err(E0101_UNCLOSED, format!("this '{{' of {what} is never closed"), open)
            .with_help("add the missing '}'")
            .with_note(format!("the file ends at line {} without it", self.line_of(eof.start)))
    }

    fn file(&mut self) -> Result<File, Diagnostic> {
        let mut file = File { module: None, items: Vec::new() };
        loop {
            self.skip_separators();
            let t = self.peek().clone();
            match &t.tok {
                Tok::Eof => return Ok(file),
                Tok::Ident(kw) if kw == "mod" => {
                    self.bump();
                    let name = self.ident("a mod name after `mod`")?;
                    if file.module.is_some() {
                        return Err(err(E0100_UNEXPECTED, "a file names its mod once", name.span));
                    }
                    file.module = Some(name);
                }
                Tok::Ident(kw) if kw == "override" => {
                    self.bump();
                    let kind = self.ident("what to override, e.g. `menu`")?;
                    let name = self.ident(&format!("the Native {} name", kind.text))?;
                    let head = format!("override {} {}", kind.text, name.text);
                    let open = self.open_brace(&head)?;
                    let mut handlers = Vec::new();
                    loop {
                        self.skip_separators();
                        let t = self.peek().clone();
                        match &t.tok {
                            Tok::RBrace => {
                                self.bump();
                                file.items.push(Item::Override {
                                    kind,
                                    name,
                                    handlers,
                                    span: Span { start: open.start, end: t.span.end },
                                });
                                break;
                            }
                            Tok::Eof => return Err(self.unclosed(open, &format!("`{head}`"))),
                            // A top-level item inside the block: the block lost its `}`.
                            Tok::Ident(kw)
                                if kw == "override"
                                    || kw == "mod"
                                    || (kw == "on"
                                        && self.toks.get(self.pos + 2).is_some_and(|t| t.tok == Tok::Dot)) =>
                            {
                                return Err(self
                                    .unclosed(open, &format!("`{head}`"))
                                    .with_note(format!("line {} starts `{kw}` while this block is still open", self.line_of(t.span.start))));
                            }
                            Tok::Ident(kw) if kw == "on" => {
                                self.bump();
                                let event = self.ident("an event after `on`, e.g. `enter`")?;
                                let body = self.block(&format!("on {}", event.text))?;
                                handlers.push(Handler { event, body });
                            }
                            other => {
                                return Err(err(
                                    E0100_UNEXPECTED,
                                    format!("expected `on <event> {{` inside `{head}`, found {}", describe(other)),
                                    t.span,
                                )
                                .with_help("for example `on enter { ... }`"));
                            }
                        }
                    }
                }
                Tok::Ident(kw) if kw == "on" => {
                    let start = t.span.start;
                    self.bump();
                    let mut path = vec![self.ident("what to hook, e.g. `game.start`")?];
                    while self.peek().tok == Tok::Dot {
                        self.bump();
                        path.push(self.ident("a name after '.'")?);
                    }
                    let head = format!("on {}", path.iter().map(|p| p.text.as_str()).collect::<Vec<_>>().join("."));
                    let body = self.block(&head)?;
                    let end = body.span.end;
                    file.items.push(Item::On { path, body, span: Span { start, end } });
                }
                Tok::RBrace => {
                    return Err(err(E0102_STRAY_CLOSE, "this '}' has no matching '{'", t.span)
                        .with_help("remove it, or check which block above was closed early"));
                }
                other => {
                    return Err(err(
                        E0100_UNEXPECTED,
                        format!("expected `mod`, `override` or `on` at the top level, found {}", describe(other)),
                        t.span,
                    ));
                }
            }
        }
    }

    fn block(&mut self, head: &str) -> Result<Block, Diagnostic> {
        let open = self.open_brace(head)?;
        let mut stmts = Vec::new();
        loop {
            self.skip_separators();
            let t = self.peek().clone();
            match &t.tok {
                Tok::RBrace => {
                    self.bump();
                    return Ok(Block { stmts, span: Span { start: open.start, end: t.span.end } });
                }
                Tok::Eof => return Err(self.unclosed(open, &format!("`{head}`"))),
                Tok::Ident(_) => stmts.push(self.stmt()?),
                other => {
                    return Err(err(
                        E0100_UNEXPECTED,
                        format!("expected an operation like `name(a, b)`, found {}", describe(other)),
                        t.span,
                    ));
                }
            }
        }
    }

    fn stmt(&mut self) -> Result<Stmt, Diagnostic> {
        let op = self.ident("an operation name")?;
        if matches!(op.text.as_str(), "if" | "elif" | "else" | "while" | "for") {
            let python = self.toks[self.pos..]
                .iter()
                .take_while(|t| t.tok != Tok::Newline && t.tok != Tok::Eof)
                .any(|t| t.tok == Tok::Colon);
            let d = err(
                if python { E0104_PYTHON_BLOCK } else { E0100_UNEXPECTED },
                format!("`{}` is not in UW yet (Faz 1 has `on`, `override` and operation calls)", op.text),
                op.span,
            );
            return Err(if python {
                d.with_help("UW blocks use { }, not ':' — and for now use Native's try_begin()/try_end() operations")
            } else {
                d.with_help("for now use Native's try_begin() ... try_end() operations")
            });
        }
        let mut args = Vec::new();
        let mut end = op.span.end;
        match self.peek().tok {
            Tok::LParen => {
                let open = self.bump().span;
                loop {
                    self.skip_newlines();
                    let t = self.peek().clone();
                    let arg = match t.tok {
                        Tok::RParen => {
                            self.bump();
                            end = t.span.end;
                            break;
                        }
                        Tok::Ident(text) => Arg::Name(Ident { text, span: t.span }),
                        Tok::Global(text) => Arg::Global(Ident { text, span: t.span }),
                        Tok::Local(text) => Arg::Local(Ident { text, span: t.span }),
                        Tok::Int(n) => Arg::Int(n, t.span),
                        Tok::Str(s) => Arg::Str(s, t.span),
                        Tok::Eof | Tok::LBrace | Tok::RBrace => {
                            return Err(err(E0101_UNCLOSED, format!("this '(' of `{}` is never closed", op.text), open)
                                .with_help("add the missing ')'"));
                        }
                        other => {
                            return Err(err(E0100_UNEXPECTED, format!("expected a value, found {}", describe(&other)), t.span)
                                .with_help("values are names (p_town_6), $globals, :locals, numbers or \"text\""));
                        }
                    };
                    self.bump();
                    args.push(arg);
                    self.skip_newlines();
                    let t = self.peek().clone();
                    match t.tok {
                        Tok::Comma => {
                            self.bump();
                        }
                        Tok::RParen => {}
                        Tok::Eof | Tok::LBrace | Tok::RBrace => {
                            return Err(err(E0101_UNCLOSED, format!("this '(' of `{}` is never closed", op.text), open)
                                .with_help("add the missing ')'"));
                        }
                        other => {
                            return Err(err(E0100_UNEXPECTED, format!("expected ',' or ')', found {}", describe(&other)), t.span));
                        }
                    }
                }
            }
            Tok::Newline | Tok::Semi | Tok::RBrace | Tok::Eof => {}
            Tok::Colon => {
                return Err(err(E0104_PYTHON_BLOCK, format!("`{}` is not a UW block; blocks use {{ }}", op.text), self.peek().span)
                    .with_help("UW has no `if ...:` yet; Faz 1 has `on`, `override` and operation calls"));
            }
            ref other => {
                return Err(err(
                    E0100_UNEXPECTED,
                    format!("expected '(' after `{}`, found {}", op.text, describe(other)),
                    self.peek().span,
                )
                .with_help(format!("call it as `{}(a, b)`", op.text)));
            }
        }
        // One statement per line (or `;`).
        match self.peek().tok {
            Tok::Newline | Tok::Semi | Tok::RBrace | Tok::Eof => {}
            ref other => {
                return Err(err(E0100_UNEXPECTED, format!("expected a new line or ';' after `{}(...)`, found {}", op.text, describe(other)), self.peek().span));
            }
        }
        Ok(Stmt { span: Span { start: op.span.start, end }, op, args })
    }
}

/// Parses one source file. The first error stops the parse.
pub fn parse(text: &str) -> Result<File, Diagnostic> {
    let toks = lex(text)?;
    Parser { toks, pos: 0, text: text.to_owned() }.file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use uw_diag::SourceFile;

    const PRAVEN: &str = "mod praven_start\n\n// prologue off\noverride menu start_phase_2 {\n    on enter {\n        party_relocate_near_party(p_main_party, p_town_6, 3)\n        change_screen_map()\n    }\n}\n\non game.start { assign($g_x, 1); display_message(\"@hi\") }\n";

    #[test]
    fn parses_the_praven_example() {
        let file = parse(PRAVEN).expect("parses");
        assert_eq!(file.module.as_ref().unwrap().text, "praven_start");
        assert_eq!(file.items.len(), 2);
        let Item::Override { kind, name, handlers, .. } = &file.items[0] else { panic!() };
        assert_eq!((kind.text.as_str(), name.text.as_str()), ("menu", "start_phase_2"));
        assert_eq!(handlers[0].event.text, "enter");
        let stmts = &handlers[0].body.stmts;
        assert_eq!(stmts[0].op.text, "party_relocate_near_party");
        assert_eq!(stmts[0].args.len(), 3);
        assert!(matches!(&stmts[0].args[2], Arg::Int(3, _)));
        assert!(stmts[1].args.is_empty());
        let Item::On { path, body, .. } = &file.items[1] else { panic!() };
        assert_eq!(path.iter().map(|p| p.text.as_str()).collect::<Vec<_>>(), ["game", "start"]);
        assert!(matches!(&body.stmts[0].args[0], Arg::Global(g) if g.text == "$g_x"));
        assert!(matches!(&body.stmts[1].args[0], Arg::Str(s, _) if s == "@hi"));
    }

    #[test]
    fn pasted_text_reads_the_same() {
        let pasted = PRAVEN
            .replace('\n', "\r\n")
            .replace("    ", "\t")
            .replace("\"@hi\"", "\u{201c}@hi\u{201d}")
            .replace("mod ", "mod\u{a0}");
        assert_eq!(parse(&pasted).unwrap().items.len(), parse(PRAVEN).unwrap().items.len());
        let a = format!("{:?}", strip(parse(&pasted).unwrap()));
        let b = format!("{:?}", strip(parse(PRAVEN).unwrap()));
        assert_eq!(a, b);
    }

    /// The tree without spans (pasted text has other offsets).
    fn strip(file: File) -> Vec<String> {
        let mut out = vec![file.module.map(|m| m.text).unwrap_or_default()];
        for item in file.items {
            let blocks = match item {
                Item::Override { handlers, .. } => handlers.into_iter().map(|h| h.body).collect(),
                Item::On { body, .. } => vec![body],
            };
            for b in blocks {
                for s in b.stmts {
                    let args: Vec<String> = s
                        .args
                        .iter()
                        .map(|a| match a {
                            Arg::Name(i) | Arg::Global(i) | Arg::Local(i) => i.text.clone(),
                            Arg::Int(n, _) => n.to_string(),
                            Arg::Str(t, _) => format!("{t:?}"),
                        })
                        .collect();
                    out.push(format!("{}({})", s.op.text, args.join(",")));
                }
            }
        }
        out
    }

    fn error(text: &str) -> (Diagnostic, String) {
        let d = parse(text).expect_err("should fail");
        let rendered = d.render(&SourceFile::new("t.uw", text));
        (d, rendered)
    }

    #[test]
    fn unclosed_brace_points_at_its_own_line() {
        let (d, r) = error("override menu x {\n  on enter {\n    a()\n  }\n\n\n");
        assert_eq!(d.code, E0101_UNCLOSED);
        assert!(r.contains("--> t.uw:1:17"), "{r}");
        assert!(r.contains("never closed"), "{r}");
        assert!(!r.contains("unexpected end"), "{r}");
        let (d, r) = error("on game.start {\n  a(1, 2\n}\n");
        assert_eq!(d.code, E0101_UNCLOSED);
        assert!(r.contains("--> t.uw:2:4"), "{r}");
    }

    #[test]
    fn missing_close_before_next_item_names_both_lines() {
        let (d, r) = error("override menu x {
  on enter {
    a()
  }

on game.start {
}
");
        assert_eq!(d.code, E0101_UNCLOSED);
        assert!(r.contains("--> t.uw:1:17"), "{r}");
        assert!(r.contains("line 6 starts `on` while this block is still open"), "{r}");
    }

    #[test]
    fn python_style_block_gets_a_fix() {
        let (d, r) = error("on game.start:\n  a()\n");
        assert_eq!(d.code, E0104_PYTHON_BLOCK);
        assert!(r.contains("write `on game.start {`"), "{r}");
        let (d, _) = error("on game.start {\n  if x:\n}\n");
        assert_eq!(d.code, E0104_PYTHON_BLOCK);
    }

    #[test]
    fn stray_close_and_junk_are_named() {
        let (d, _) = error("mod m\n}\n");
        assert_eq!(d.code, E0102_STRAY_CLOSE);
        let (d, r) = error("on game.start { a() b() }");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("expected a new line or ';'"), "{r}");
        let (d, _) = error("on game.start { a(@) }");
        assert_eq!(d.code, E0103_BAD_TOKEN);
    }

    #[test]
    fn comments_and_wrapped_arguments() {
        let text = "# py comment\n/* block\ncomment */ on game.start { // trailing\n  f(a,\n    b,\n  ) ; g\n}";
        let file = parse(text).unwrap();
        let Item::On { body, .. } = &file.items[0] else { panic!() };
        assert_eq!(body.stmts.len(), 2);
        assert_eq!(body.stmts[0].args.len(), 2);
        assert_eq!(body.stmts[1].op.text, "g");
    }

    #[test]
    fn negative_numbers_and_locals() {
        let file = parse("on game.start { f(-5, :loc) }").unwrap();
        let Item::On { body, .. } = &file.items[0] else { panic!() };
        assert!(matches!(body.stmts[0].args[0], Arg::Int(-5, _)));
        assert!(matches!(&body.stmts[0].args[1], Arg::Local(l) if l.text == ":loc"));
    }
}
