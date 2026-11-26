//! Lexer of the UW core language v0 (docs/dil/CORE-v0.md §2): source text → tokens with spans and lines.
//!
//! Numbers are already fixed-point here (`12` → 12000 milli, `0.5` → 500) and a number's suffix is folded
//! into one token: `150%` is a [`Tok::Percent`], `15 per second` a [`Tok::Rate`]. A `.` is a decimal point
//! only when a digit follows it, so the range `0..100` stays three tokens.

use crate::Error;

use crate::ast::{RateUnit, Span, TimeUnit};

/// One token of the core language. Words are not split into keywords: §2's keyword list is short and the
/// parser checks the few places that need it (`while in`, `not`, `true`, …), so `ui` and `when` still work
/// as words.
#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    /// An identifier or keyword.
    Word(String),
    /// A number in thousandths: `12` → 12000, `0.5` → 500 (§4: every UW number is ×1000).
    Num(i64),
    /// `150%` → 150.
    Percent(i64),
    /// `15 per second` → 15000 milli per [`RateUnit`].
    Rate { milli: i64, unit: RateUnit },
    /// `6 hours` → 6000 milli of [`TimeUnit::Hour`].
    Duration { milli: i64, unit: TimeUnit },
    /// `#3FA34D` → 0x3FA34D.
    Color(u32),
    /// A string literal: the byte range of its inner text in the source, quotes excluded.
    Str { start: usize, end: usize },
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comma,
    Dot,
    DotDot,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    EqEq,
    NotEq,
    Lt,
    Le,
    Gt,
    Ge,
    Plus,
    Minus,
    Star,
    Slash,
    /// A line break: statements have no terminator (§3), so the parser needs the line ends.
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub tok: Tok,
    pub span: Span,
}

/// Lexes a whole file.
pub fn lex(file: &str, text: &str) -> (Vec<Token>, Vec<Error>) {
    lex_from(file, text, 0, text.len(), 1)
}

/// Lexes `text[from..to]`, counting lines as if the slice started on `line` (used for `{expr}` inside a
/// string). Spans stay byte offsets into the whole `text`.
pub fn lex_from(file: &str, text: &str, from: usize, to: usize, line: u32) -> (Vec<Token>, Vec<Error>) {
    let mut lx = Lexer { file, text, pos: from, end: to.min(text.len()), line, toks: Vec::new(), errors: Vec::new() };
    lx.run();
    (lx.toks, lx.errors)
}

/// 1-based line of `offset` in `text` (how a `{expr}` inside a string finds its line).
pub fn line_at(text: &str, offset: usize) -> u32 {
    text[..offset.min(text.len())].matches('\n').count() as u32 + 1
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r'
}

struct Lexer<'a> {
    file: &'a str,
    text: &'a str,
    pos: usize,
    end: usize,
    line: u32,
    toks: Vec<Token>,
    errors: Vec<Error>,
}

impl<'a> Lexer<'a> {
    fn run(&mut self) {
        while let Some(c) = self.peek() {
            let start = self.pos;
            let line = self.line;
            // `//` to the end of the line (§2); `#` starts a color, not a comment.
            if c == '/' && self.rest().starts_with("//") {
                while self.peek().is_some_and(|c| c != '\n') {
                    self.advance();
                }
            } else if c == '\n' {
                self.advance();
                self.push(Tok::Newline, start, line);
            } else if is_space(c) || c == '\u{feff}' {
                self.advance();
            } else if let Some(tok) = self.punct(c) {
                self.push(tok, start, line);
            } else if c.is_ascii_digit() {
                self.number();
            } else if is_ident_start(c) {
                let word = self.take_word();
                self.push(Tok::Word(word), start, line);
            } else if c == '#' {
                self.color(start, line);
            } else if c == '"' {
                if let Some(tok) = self.string(line) {
                    self.push(tok, start, line);
                }
            } else {
                self.advance();
                self.errors.push(self.error(line, format!("unknown character '{c}'")));
            }
        }
        let (pos, line) = (self.pos, self.line);
        self.push(Tok::Eof, pos, line);
    }

    fn peek(&self) -> Option<char> {
        if self.pos >= self.end {
            return None;
        }
        self.text[self.pos..].chars().next()
    }

    fn rest(&self) -> &'a str {
        &self.text[self.pos.min(self.end)..self.end]
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        if c == '\n' {
            self.line += 1;
        }
        Some(c)
    }

    fn push(&mut self, tok: Tok, start: usize, line: u32) {
        self.toks.push(Token { tok, span: Span { start, end: self.pos, line } });
    }

    fn error(&self, line: u32, message: String) -> Error {
        Error { file: self.file.to_string(), line, message }
    }

    /// Operators and punctuation; `None` when `c` does not start one.
    fn punct(&mut self, c: char) -> Option<Tok> {
        let rest = self.rest();
        let tok = match c {
            '{' => Tok::LBrace,
            '}' => Tok::RBrace,
            '(' => Tok::LParen,
            ')' => Tok::RParen,
            ',' => Tok::Comma,
            '.' if rest.starts_with("..") => Tok::DotDot,
            '.' => Tok::Dot,
            '=' if rest.starts_with("==") => Tok::EqEq,
            '=' => Tok::Assign,
            '!' if rest.starts_with("!=") => Tok::NotEq,
            '<' if rest.starts_with("<=") => Tok::Le,
            '<' => Tok::Lt,
            '>' if rest.starts_with(">=") => Tok::Ge,
            '>' => Tok::Gt,
            '+' if rest.starts_with("+=") => Tok::PlusAssign,
            '+' => Tok::Plus,
            '-' if rest.starts_with("-=") => Tok::MinusAssign,
            '-' => Tok::Minus,
            '*' if rest.starts_with("*=") => Tok::StarAssign,
            '*' => Tok::Star,
            '/' if rest.starts_with("/=") => Tok::SlashAssign,
            '/' => Tok::Slash,
            _ => return None,
        };
        let n = match tok {
            Tok::DotDot | Tok::EqEq | Tok::NotEq | Tok::Le | Tok::Ge | Tok::PlusAssign | Tok::MinusAssign
            | Tok::StarAssign | Tok::SlashAssign => 2,
            _ => 1,
        };
        for _ in 0..n {
            self.advance();
        }
        Some(tok)
    }

    fn take_word(&mut self) -> String {
        let start = self.pos;
        while self.peek().is_some_and(is_ident) {
            self.advance();
        }
        self.text[start..self.pos].to_string()
    }

    fn skip_space(&mut self) {
        while self.peek().is_some_and(is_space) {
            self.advance();
        }
    }

    /// True when the text here is the word `word` on its own (`per`, not `percent`).
    /// The identifier starting here, without moving.
    fn word_ahead(&self) -> Option<String> {
        let word: String = self.rest().chars().take_while(|c| is_ident(*c)).collect();
        (!word.is_empty()).then_some(word)
    }

    fn word_here(&self, word: &str) -> bool {
        let rest = self.rest();
        rest.starts_with(word) && !rest[word.len()..].chars().next().is_some_and(is_ident)
    }

    /// `12`, `0.5`, `150%`, `15 per second` (§2).
    fn number(&mut self) {
        let start = self.pos;
        let line = self.line;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }
        let mut milli = self.text[start..self.pos].parse::<i64>().unwrap_or(0).saturating_mul(1000);
        // `0..100`: a `.` is a decimal point only when a digit follows it.
        if self.rest().starts_with('.') && self.rest()[1..].chars().next().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
            let frac_start = self.pos;
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
            let digits = &self.text[frac_start..self.pos];
            let mut scaled = 0i64;
            for (i, c) in digits.chars().take(3).enumerate() {
                scaled += c.to_digit(10).unwrap_or(0) as i64 * 10i64.pow(2 - i as u32);
            }
            milli = milli.saturating_add(scaled);
        }
        let number_end = self.pos;
        self.skip_space();
        if self.rest().starts_with('%') {
            self.advance();
            // §2 only writes whole percents; a fractional one is rounded to the nearest whole.
            let value = if milli % 1000 == 0 { milli / 1000 } else { (milli + 500) / 1000 };
            self.push(Tok::Percent(value), start, line);
            return;
        }
        if self.word_here("per") {
            self.take_word();
            self.skip_space();
            let unit_start = self.pos;
            let unit_line = self.line;
            let unit = self.take_word();
            let unit = match unit.as_str() {
                "second" => Some(RateUnit::Second),
                "minute" => Some(RateUnit::Minute),
                "hour" => Some(RateUnit::Hour),
                _ => None,
            };
            match unit {
                Some(unit) => {
                    self.push(Tok::Rate { milli, unit }, start, line);
                    return;
                }
                None => {
                    let found = if self.pos == unit_start { "nothing".to_string() } else { format!("'{}'", &self.text[unit_start..self.pos]) };
                    self.errors.push(self.error(
                        unit_line,
                        format!("expected a rate unit after 'per' (second, minute, hour), found {found}"),
                    ));
                }
            }
        }
        if let Some(unit) = self.word_ahead().as_deref().and_then(TimeUnit::from_word) {
            self.take_word();
            self.push(Tok::Duration { milli, unit }, start, line);
            return;
        }
        // No suffix: put back the spaces we looked through, so spans and the next token stay exact.
        self.pos = number_end;
        self.push(Tok::Num(milli), start, line);
    }

    /// `#3FA34D`.
    fn color(&mut self, start: usize, line: u32) {
        self.advance();
        let digits_start = self.pos;
        while self.peek().is_some_and(|c| c.is_ascii_hexdigit()) {
            self.advance();
        }
        let digits = &self.text[digits_start..self.pos];
        match u32::from_str_radix(digits, 16) {
            Ok(rgb) if digits.len() == 6 => self.push(Tok::Color(rgb), start, line),
            _ => self.errors.push(self.error(
                line,
                format!("expected a color as '#' and six hex digits (e.g. #3FA34D), found '#{digits}'"),
            )),
        }
    }

    /// `"…"`, with the inner range kept so the parser can split `{expr}` parts out of it.
    fn string(&mut self, line: u32) -> Option<Tok> {
        self.advance();
        let inner = self.pos;
        let mut depth = 0usize;
        loop {
            let Some(c) = self.peek() else {
                self.errors.push(self.error(line, "this text is never closed".to_string()));
                return None;
            };
            match c {
                '\\' => {
                    self.advance();
                    self.advance();
                }
                '"' if depth == 0 => {
                    let end = self.pos;
                    self.advance();
                    return Some(Tok::Str { start: inner, end });
                }
                // A text inside `{ … }`: its own quotes and braces do not end the outer text.
                '"' => self.skip_inner_string(),
                '{' => {
                    depth += 1;
                    self.advance();
                }
                '}' if depth > 0 => {
                    depth -= 1;
                    self.advance();
                }
                '\n' if depth == 0 => {
                    self.errors.push(self.error(line, "this text is never closed".to_string()));
                    return None;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn skip_inner_string(&mut self) {
        self.advance();
        while let Some(c) = self.peek() {
            match c {
                '\\' => {
                    self.advance();
                    self.advance();
                }
                '"' => {
                    self.advance();
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }
}
