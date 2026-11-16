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
    /// `override <kind> <name> { … }` (Faz 1 form) or `override <id> { … }` with the kind taken
    /// from the id prefix (`override menu_start_phase_2`, docs/meclis/gramer-2-oneri.md §3): then
    /// `kind` is `None` and `name` holds the full id.
    /// `handlers` are the `on <event>` blocks, `members` the other menu lines (text, options).
    Override { kind: Option<Ident>, name: Ident, handlers: Vec<Handler>, members: Vec<Member>, span: Span },
    /// `new <id> "<text>" { … }`: a new record (`new menu_uw_ask "…" { … }`, §4).
    New { id: Ident, text: Option<(String, Span)>, handlers: Vec<Handler>, members: Vec<Member>, span: Span },
    /// `on a.b { body }`
    On { path: Vec<Ident>, body: Block, span: Span },
    /// `fn <name>(:p1, :p2) { body }`: a named piece of code used as one line; a call to it is
    /// replaced by its body at check time (T-078).
    Fn { name: Ident, params: Vec<Ident>, body: Block, span: Span },
    /// `save <name> = <int | true | false>`: a named global that lives in the save game, set to
    /// its start value once at game start (T-085).
    Save { name: Ident, value: Expr, span: Span },
    /// `every day { body }` / `every <N> hours { body }`: one new simple trigger whose operation
    /// block runs every `interval` game hours (`24` = a day, fractions allowed). The interval is
    /// kept as Python writes it (`24`, `6`, `0.5`): the kit's simple_triggers record is
    /// `(check_interval, ops)` and the interval is a plain number there (T-088).
    Every { interval: String, body: Block, span: Span },
    /// `area <name> [(x, y), …]`: a map polygon, corners in world-map units × 100 (the fixed-point
    /// multiplier the position operations are read with), so the tree keeps no float (T-109).
    Area { name: Ident, points: Vec<AreaPoint>, span: Span },
    /// `on enter <area> [when <cond>] { body }`: a simple trigger that runs `body` when the
    /// player's party goes from outside the area to inside it (T-109).
    AreaEnter { area: Ident, when: Option<When>, body: Block, span: Span },
    /// `<kind> <new_id> like <existing> { <field> <value> … }`: a copy of the Native record
    /// `<existing>` with some fields changed, added at the end of its list (T-092 troops, T-099
    /// towns, castles, villages and parties). `kind` is the keyword as written (`troop`, `town`,
    /// `castle`, `village`, `party`); the table it copies from is the check's business. A field
    /// value is written as record text (`level 22`, `name "Elite Crossbowman"`): the field layout
    /// is kit-meta/records/<table>.toml.
    Like { kind: Ident, id: Ident, like: Ident, fields: Vec<FieldValue>, span: Span },
    /// `dialog <troop> <name> { <speaker>: "text" … }`: the talk with one troop, as Native dialog
    /// lines (docs/bilgi/anlam/diyaloglar.md, T-102). The first line is the NPC's opening, every
    /// `player:` line after it is one of the player's choices; `name` says what the dialog is
    /// (`first_meeting` is the greeting of the first talk).
    Dialog { troop: Ident, name: Ident, lines: Vec<DialogLine>, span: Span },
}

/// The top-level keywords that copy a Native record (`troop … like …`, `town … like …`, …); a copy
/// only ever reads the table of its own kind, which the check decides ([`Item::Like`]).
pub const COPY_KINDS: [&str; 5] = ["troop", "town", "castle", "village", "party"];

/// True when `kw` is one of [`COPY_KINDS`].
pub fn is_copy_kind(kw: &str) -> bool {
    COPY_KINDS.contains(&kw)
}

/// One corner of an `area` polygon, in world-map units × 100 (the fixed point
/// `set_fixed_point_multiplier, 100` makes the position operations return).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AreaPoint {
    pub x: i64,
    pub y: i64,
}

/// One `<field> <value>` line of a `<kind> … like …` body; `value` is the record text as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldValue {
    pub name: Ident,
    pub value: String,
    pub span: Span,
}

/// One line of a `dialog <troop> <name> { … }` body (T-102):
/// `<speaker>: "<text>" [when <cond>] [{ <block> }] [-> end]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogLine {
    /// The word before ':' as written (`harlaus`, `player`); `player` is the player's own line.
    pub speaker: Ident,
    pub text: String,
    /// The condition the line only shows under (`when player.renown >= 300`).
    pub when: Option<When>,
    /// What picking the line does.
    pub body: Option<Block>,
    /// The word after `->`: `end` closes the talk. Absent on the NPC's opening line, which leads
    /// to the player's choices.
    pub to: Option<Ident>,
    pub span: Span,
}

/// `on <event> { body }` inside an override.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handler {
    pub event: Ident,
    pub body: Block,
}

/// Menu lines other than `on` handlers (gramer-2-oneri.md §4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Member {
    /// `text "…"`: the record's text field.
    Text { text: String, span: Span },
    /// `option [<id>] "<text>" [when …] [after|before <id>] { body }`: a NEW option; without an id
    /// the id is made from the text (see [`option_slug`]).
    Option { id: Ident, text: String, when: Option<When>, place: Option<Place>, body: Block, span: Span },
    /// `change option <id> { text "…" / when … / do { … } }`: change parts of an existing option.
    ChangeOption { id: Ident, text: Option<(String, Span)>, when: Option<When>, body: Option<Block>, span: Span },
    /// `remove option <id>`
    RemoveOption { id: Ident, span: Span },
}

/// An option's condition: `when <cond>` or `when { statements }`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum When {
    Cond(Cond),
    Block(Block),
}

/// `after <id>` / `before <id>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Place {
    After(Ident),
    Before(Ident),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// One statement of a block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    /// A Native operation: `name(a, b)`.
    Call(Call),
    /// `target = expr`, `target += expr` …; `target` is `$global`, `:local` or a register name.
    Assign { target: Arg, op: AssignOp, value: Expr, span: Span },
    /// `if c { } else if c { } else { }`: `arms` in order, `otherwise` the final `else`.
    If { arms: Vec<(Cond, Block)>, otherwise: Option<Block>, span: Span },
    /// `require c`: if `c` fails, the rest of the enclosing arm/block does not run (Native: a condition
    /// operation in the middle of a block). docs/meclis/YON2-KARAR.md §4.2.
    Require { cond: Cond, span: Span },
    /// `attempt { } else attempt { } else { }`: Native `try_begin / else_try / try_end`; the first arm
    /// whose `require`s all hold runs. A final `else { }` is one more arm. §4.3.
    Attempt { arms: Vec<Block>, span: Span },
    /// `for :i in range(0, 10) { }` = `try_for_range(:i, 0, 10)` … `try_end`. `kind` is the kit op name
    /// without `try_for_` (`range`, `range_backwards`, `parties`, `agents`, `prop_instances`, `players`);
    /// `args` follow the loop variable. P2 (2026-11-15).
    For { var: Arg, kind: Ident, args: Vec<Arg>, body: Block, span: Span },
}

impl Stmt {
    /// The operation call, if this statement is one.
    pub fn as_call(&self) -> Option<&Call> {
        match self {
            Stmt::Call(call) => Some(call),
            _ => None,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Stmt::Call(call) => call.span,
            Stmt::Assign { span, .. }
            | Stmt::If { span, .. }
            | Stmt::Require { span, .. }
            | Stmt::Attempt { span, .. }
            | Stmt::For { span, .. } => *span,
        }
    }
}

/// A Native operation call: `name(args)` (or a bare `name` for an operation without operands).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub op: Ident,
    pub args: Vec<Arg>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    /// `=`
    Set,
    /// `+=` `-=` `*=` `/=` `%=`
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

/// An integer expression. Leaves are the same values an operation takes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Value(Arg),
    /// Unary minus on a non-literal (`-$x`); `-5` stays `Value(Arg::Int(-5))`.
    Neg(Box<Expr>, Span),
    Bin { op: BinOp, left: Box<Expr>, right: Box<Expr>, span: Span },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

/// A condition (`if`, `when`). Precedence `not` > `and` > `or` (Python), §2 and §7.3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cond {
    /// `a < b`
    Cmp { op: CmpOp, left: Expr, right: Expr, span: Span },
    /// A Native condition operation used as a condition: `party_is_active(p_town_6)`.
    Call(Call),
    Not(Box<Cond>, Span),
    And(Vec<Cond>),
    Or(Vec<Cond>),
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
    LBracket,
    RBracket,
    Comma,
    Semi,
    Dot,
    Colon,
    /// `+ - * / %` (also the unary `-`).
    Bin(BinOp),
    /// `== != > >= < <=`
    Cmp(CmpOp),
    /// `=` `+=` `-=` `*=` `/=` `%=`
    Assign(AssignOp),
    /// `->` (where a dialog line leads, `-> end`).
    Arrow,
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
    let mut out: Vec<Token> = Vec::new();
    let mut i = 0;
    // A new line inside parentheses does not end the statement (§7.5).
    let mut depth = 0usize;
    while i < chars.len() {
        let (start, c) = chars[i];
        // `-` starts a number only where a value may begin, so `a - 5` keeps its minus.
        let prev_is_value = matches!(
            out.last().map(|t| &t.tok),
            Some(Tok::Ident(_) | Tok::Global(_) | Tok::Local(_) | Tok::Int(_) | Tok::Str(_) | Tok::RParen | Tok::RBrace)
        );
        let one = |tok| Token { tok, span: Span { start, end: end_of(i + 1) } };
        match c {
            '\n' => {
                if depth == 0 {
                    out.push(one(Tok::Newline));
                }
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
                if depth == 0 && chars[i..j].iter().any(|p| p.1 == '\n') {
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
            // `->` (a dialog line's ending, `-> end`).
            '-' if chars.get(i + 1).map(|p| p.1) == Some('>') => {
                out.push(one(Tok::Arrow));
                i += 2;
            }
            '(' => {
                out.push(one(Tok::LParen));
                depth += 1;
                i += 1;
            }
            ')' => {
                out.push(one(Tok::RParen));
                depth = depth.saturating_sub(1);
                i += 1;
            }
            // `area <name> [(x, y), …]`: the corner list, wrapping over lines like `(` (T-109).
            '[' => {
                out.push(one(Tok::LBracket));
                depth += 1;
                i += 1;
            }
            ']' => {
                out.push(one(Tok::RBracket));
                depth = depth.saturating_sub(1);
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
                                Some(other @ ('"' | '\\' | '\u{201c}' | '\u{201d}')) => value.push(other),
                                _ => {
                                    return Err(err(
                                        E0103_BAD_TOKEN,
                                        "unknown escape in text",
                                        Span { start: chars[j].0, end: end_of(j + 2) },
                                    )
                                    .with_help("use \\\" for a quote, \\\\ for a backslash, \\n for a new line, \\\u{201c} for a curly quote"));
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
            // Native names may start with a digit after the sigil (`:400_plus_random_200`,
            // `:2ary_inputs_taken_from_market`); the kit takes any text after `$` / `:`.
            '$' | ':' if chars.get(i + 1).is_some_and(|p| is_ident(p.1)) => {
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
            c if c.is_ascii_digit()
                || (c == '-' && !prev_is_value && chars.get(i + 1).is_some_and(|p| p.1.is_ascii_digit())) =>
            {
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
            c @ ('+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>') => {
                let two = chars.get(i + 1).map(|p| p.1) == Some('=');
                let tok = match (c, two) {
                    ('+', false) => Tok::Bin(BinOp::Add),
                    ('+', true) => Tok::Assign(AssignOp::Add),
                    ('-', false) => Tok::Bin(BinOp::Sub),
                    ('-', true) => Tok::Assign(AssignOp::Sub),
                    ('*', false) => Tok::Bin(BinOp::Mul),
                    ('*', true) => Tok::Assign(AssignOp::Mul),
                    ('/', false) => Tok::Bin(BinOp::Div),
                    ('/', true) => Tok::Assign(AssignOp::Div),
                    ('%', false) => Tok::Bin(BinOp::Mod),
                    ('%', true) => Tok::Assign(AssignOp::Mod),
                    ('=', false) => Tok::Assign(AssignOp::Set),
                    ('=', true) => Tok::Cmp(CmpOp::Eq),
                    ('!', true) => Tok::Cmp(CmpOp::Ne),
                    ('<', false) => Tok::Cmp(CmpOp::Lt),
                    ('<', true) => Tok::Cmp(CmpOp::Le),
                    ('>', false) => Tok::Cmp(CmpOp::Gt),
                    ('>', true) => Tok::Cmp(CmpOp::Ge),
                    _ => {
                        return Err(err(E0103_BAD_TOKEN, "'!' is not part of UW; write '!='", Span { start, end: end_of(i + 1) })
                            .with_help("write `!=` for 'not equal'"));
                    }
                };
                let len = if two { 2 } else { 1 };
                out.push(Token { tok, span: Span { start, end: end_of(i + len) } });
                i += len;
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
                // `neg|eq` / `neg|this_or_next|eq`: the part after `|` belongs to the same name.
                while chars.get(j).map(|p| p.1) == Some('|') && chars.get(j + 1).is_some_and(|p| is_ident_start(p.1)) {
                    j += 1;
                    while j < chars.len() && is_ident(chars[j].1) {
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
        Tok::LBracket => "'['".into(),
        Tok::RBracket => "']'".into(),
        Tok::Comma => "','".into(),
        Tok::Semi => "';'".into(),
        Tok::Dot => "'.'".into(),
        Tok::Colon => "':'".into(),
        Tok::Bin(op) => match op {
            BinOp::Add => "'+'",
            BinOp::Sub => "'-'",
            BinOp::Mul => "'*'",
            BinOp::Div => "'/'",
            BinOp::Mod => "'%'",
        }
        .into(),
        Tok::Cmp(op) => match op {
            CmpOp::Eq => "'=='",
            CmpOp::Ne => "'!='",
            CmpOp::Gt => "'>'",
            CmpOp::Ge => "'>='",
            CmpOp::Lt => "'<'",
            CmpOp::Le => "'<='",
        }
        .into(),
        Tok::Assign(op) => match op {
            AssignOp::Set => "'='",
            AssignOp::Add => "'+='",
            AssignOp::Sub => "'-='",
            AssignOp::Mul => "'*='",
            AssignOp::Div => "'/='",
            AssignOp::Mod => "'%='",
        }
        .into(),
        Tok::Newline => "the end of the line".into(),
        Tok::Arrow => "'->'".into(),
        Tok::Eof => "the end of the file".into(),
    }
}

/// The id an option written without one gets (§2, T-091): `"Party morale report"` ->
/// `uw_party_morale_report` — lower case, a run of other characters becomes one `_`, the prefix is
/// `uw_`. `None` when nothing usable is left (`"…"`, `"2 Hand sword"`: an id cannot start with a
/// digit), so the caller falls back to `uw_option`.
fn option_slug(text: &str) -> Option<String> {
    let mut out = String::new();
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.is_empty() && !out.ends_with('_') {
            out.push('_');
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    if out.is_empty() || out.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }
    Some(format!("uw_{out}"))
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

    /// `a.b.c`: a `.` after a name continues it. A dotted word is one name — `fn player.gold.add`,
    /// the target `player.gold`, the value `player.gold` (T-082).
    fn dotted_after(&mut self, first: Ident) -> Result<Ident, Diagnostic> {
        let start = first.span.start;
        let mut text = first.text;
        let mut end = first.span.end;
        while self.peek().tok == Tok::Dot {
            self.bump();
            let part = self.ident("a name after '.'")?;
            text.push('.');
            text.push_str(&part.text);
            end = part.span.end;
        }
        Ok(Ident { text, span: Span { start, end } })
    }

    /// `neg|eq` / `this_or_next|party_is_active`: only `neg` and `this_or_next` may come before `|`.
    fn check_flag_prefix(&self, op: &Ident) -> Result<(), Diagnostic> {
        let parts: Vec<&str> = op.text.split('|').collect();
        let mut at = op.span.start;
        for part in &parts[..parts.len().saturating_sub(1)] {
            if !matches!(*part, "neg" | "this_or_next") {
                return Err(err(
                    E0100_UNEXPECTED,
                    format!("`{part}` is not allowed before '|' in an operation name"),
                    Span { start: at, end: at + part.len() },
                )
                .with_help("only `neg` and `this_or_next` may come before '|', e.g. `neg|eq(a, b)`"));
            }
            // Past the `|`.
            at += part.len() + 1;
        }
        Ok(())
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
                    let first = self.ident("what to override, e.g. `menu`")?;
                    // `override menu_start_phase_2 { … }` writes the id in one word (§3): the kind
                    // comes from the id prefix, so only `name` is kept.
                    let (kind, name) = if self.peek_after_newlines().tok == Tok::LBrace {
                        (None, first)
                    } else {
                        let name = self.ident(&format!("the Native {} name", first.text))?;
                        (Some(first), name)
                    };
                    let head = match &kind {
                        Some(kind) => format!("override {} {}", kind.text, name.text),
                        None => format!("override {}", name.text),
                    };
                    let (handlers, members, span) = self.record_body(&head)?;
                    file.items.push(Item::Override { kind, name, handlers, members, span });
                }
                // `in menu camp { … }` is the easy-language spelling of `override menu camp { … }`
                // (docs/meclis/kolay-dil-taslak.md §2): same item, same body.
                Tok::Ident(kw) if kw == "in" => {
                    self.bump();
                    let first = self.ident("what to change in, e.g. `in menu camp`")?;
                    let (kind, name) = if self.peek_after_newlines().tok == Tok::LBrace {
                        (None, first)
                    } else {
                        let name = self.ident(&format!("the Native {} name", first.text))?;
                        (Some(first), name)
                    };
                    let head = match &kind {
                        Some(kind) => format!("in {} {}", kind.text, name.text),
                        None => format!("in {}", name.text),
                    };
                    let (handlers, members, span) = self.record_body(&head)?;
                    file.items.push(Item::Override { kind, name, handlers, members, span });
                }
                Tok::Ident(kw) if kw == "new" => {
                    self.bump();
                    let id = self.ident("a record id after `new`, e.g. `menu_uw_ask`")?;
                    let text = match self.peek().tok {
                        Tok::Str(_) => Some(self.text_lit(&format!("the text of `{}` in double quotes", id.text))?),
                        _ => None,
                    };
                    let (handlers, members, span) = self.record_body(&format!("new {}", id.text))?;
                    file.items.push(Item::New { id, text, handlers, members, span });
                }
                Tok::Ident(kw) if kw == "on" => {
                    let start = t.span.start;
                    self.bump();
                    // `on enter <area> [when <cond>] { … }` (T-109) is not an engine hook: an
                    // area name follows `enter`, where `on game.<hook>` has a dot instead.
                    let area_enter = matches!(&self.peek().tok, Tok::Ident(w) if w == "enter") && {
                        let mut i = self.pos + 1;
                        while self.toks[i].tok == Tok::Newline {
                            i += 1;
                        }
                        matches!(self.toks[i].tok, Tok::Ident(_))
                    };
                    if area_enter {
                        self.bump();
                        let area = self.ident("the area name after `on enter`, e.g. `on enter mountain_road`")?;
                        let when = if matches!(&self.peek().tok, Tok::Ident(w) if w == "when") {
                            self.bump();
                            Some(self.when_part()?)
                        } else {
                            None
                        };
                        let body = self.block(&format!("on enter {}", area.text))?;
                        let end = body.span.end;
                        file.items.push(Item::AreaEnter { area, when, body, span: Span { start, end } });
                    } else {
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
                }
                Tok::Ident(kw) if kw == "fn" => {
                    let start = t.span.start;
                    self.bump();
                    let name = self.ident("a name after `fn`, e.g. `fn give_gold(:amount)`")?;
                    let name = self.dotted_after(name)?;
                    let params = self.params(&name.text)?;
                    let body = self.block(&format!("fn {}", name.text))?;
                    let end = body.span.end;
                    file.items.push(Item::Fn { name, params, body, span: Span { start, end } });
                }
                Tok::Ident(kw) if kw == "save" => {
                    let start = t.span.start;
                    self.bump();
                    let name = self.ident("a name after `save`, e.g. `save rich = false`")?;
                    if name.text == "true" || name.text == "false" {
                        return Err(err(
                            E0100_UNEXPECTED,
                            format!("`{}` is 1 or 0 in a value, so it cannot name a save", name.text),
                            name.span,
                        )
                        .with_help(format!("pick another name, e.g. `save rich = {}`", name.text)));
                    }
                    let value = self.save_value(&name.text)?;
                    let end = expr_span(&value).end;
                    self.end_of_stmt("this `save` line")?;
                    file.items.push(Item::Save { name, value, span: Span { start, end } });
                }
                Tok::Ident(kw) if kw == "every" => {
                    let start = t.span.start;
                    self.bump();
                    let interval = self.every_interval()?;
                    let body = self.block(&format!("every {interval} hours"))?;
                    let end = body.span.end;
                    file.items.push(Item::Every { interval, body, span: Span { start, end } });
                }
                // `area <name> [(x, y), …]` (T-109): a map polygon, corners in world-map units.
                Tok::Ident(kw) if kw == "area" => {
                    let start = t.span.start;
                    self.bump();
                    let name = self.ident("an area name after `area`, e.g. `area mountain_road [(10, 20), …]`")?;
                    let (points, span) = self.area_points(&name.text)?;
                    let end = span.end;
                    self.end_of_stmt("this `area` line")?;
                    file.items.push(Item::Area { name, points, span: Span { start, end } });
                }
                Tok::Ident(kw) if is_copy_kind(kw) => {
                    let kind = Ident { text: kw.clone(), span: t.span };
                    let start = t.span.start;
                    self.bump();
                    let id = self.ident(&format!(
                        "the id of the new {}, e.g. `{} riverhold like praven`",
                        kind.text, kind.text
                    ))?;
                    let like_kw = self.ident(&format!(
                        "`like` and the Native {} to copy after the new {} id",
                        kind.text, kind.text
                    ))?;
                    if like_kw.text != "like" {
                        return Err(err(
                            E0100_UNEXPECTED,
                            format!("expected `like` after `{} {}`, found '{}'", kind.text, id.text, like_kw.text),
                            like_kw.span,
                        )
                        .with_help(format!("write `{} {} like <existing> {{ ... }}`", kind.text, id.text)));
                    }
                    let like = self.ident(&format!("the Native {} to copy after `like`", kind.text))?;
                    let (fields, body) = self.copy_body(&format!("{} {} like {}", kind.text, id.text, like.text))?;
                    file.items.push(Item::Like { kind, id, like, fields, span: Span { start, end: body.end } });
                }
                // `dialog <troop> <name> { <speaker>: "text" … }` (T-102): the talk with one troop,
                // as Native dialog lines (docs/bilgi/anlam/diyaloglar.md).
                Tok::Ident(kw) if kw == "dialog" => {
                    let start = t.span.start;
                    self.bump();
                    let troop =
                        self.ident("the troop to talk to after `dialog`, e.g. `dialog king_harlaus first_meeting`")?;
                    let name = self.ident("a name for the dialog, e.g. `first_meeting`")?;
                    let (lines, body) = self.dialog_body(&format!("dialog {} {}", troop.text, name.text))?;
                    file.items.push(Item::Dialog { troop, name, lines, span: Span { start, end: body.end } });
                }
                Tok::RBrace => {
                    return Err(err(E0102_STRAY_CLOSE, "this '}' has no matching '{'", t.span)
                        .with_help("remove it, or check which block above was closed early"));
                }
                other => {
                    return Err(err(
                        E0100_UNEXPECTED,
                        format!("expected `mod`, `override`, `in`, `new`, `troop`, `town`, `castle`, `village`, `party`, `on`, `fn`, `save`, `every`, `area` or `dialog` at the top level, found {}", describe(other)),
                        t.span,
                    ));
                }
            }
        }
    }

    /// `save <name> = <int | true | false>`: the start value written once at game start, so it is a
    /// whole number; `true`/`false` are 1/0 (T-085).
    fn save_value(&mut self, name: &str) -> Result<Expr, Diagnostic> {
        self.skip_newlines();
        let t = self.peek().clone();
        if t.tok != Tok::Assign(AssignOp::Set) {
            return Err(err(
                E0100_UNEXPECTED,
                format!("expected '=' and a start value after `save {name}`, found {}", describe(&t.tok)),
                t.span,
            )
            .with_help("write e.g. `save rich = false`"));
        }
        self.bump();
        self.skip_newlines();
        let t = self.peek().clone();
        let value = match &t.tok {
            Tok::Int(n) => Arg::Int(*n, t.span),
            Tok::Ident(word) if word == "true" => Arg::Int(1, t.span),
            Tok::Ident(word) if word == "false" => Arg::Int(0, t.span),
            other => {
                return Err(err(
                    E0100_UNEXPECTED,
                    format!("expected an integer, `true` or `false` as the start value of `save {name}`, found {}", describe(other)),
                    t.span,
                )
                .with_help("a save starts at a whole number, e.g. `save rich = false`"));
            }
        };
        self.bump();
        Ok(Expr::Value(value))
    }

    /// The interval of `every`: `day` (24 game hours), `hour` (1) or `<N> hours` with `N` whole or
    /// fractional (`0.5`; the lexer has no float token, so it arrives as `Int(0) Dot Int(5)`).
    /// The kit's simple_triggers record takes the interval in game hours: `24` is a day, fractions
    /// are allowed (docs/bilgi/anlam/zaman-ve-olaylar.md:45, :53). Returned as Python writes it.
    fn every_interval(&mut self) -> Result<String, Diagnostic> {
        let t = self.peek().clone();
        let whole = match &t.tok {
            Tok::Ident(word) if word == "day" => {
                self.bump();
                return Ok("24".into());
            }
            Tok::Ident(word) if word == "hour" => {
                self.bump();
                return Ok("1".into());
            }
            Tok::Int(n) => *n,
            other => {
                return Err(err(
                    E0100_UNEXPECTED,
                    format!("expected `day`, `hour` or `<N> hours` after `every`, found {}", describe(other)),
                    t.span,
                )
                .with_help("write `every day { ... }`, `every 6 hours { ... }` or `every hour { ... }`"));
            }
        };
        self.bump();
        let mut hours = whole.to_string();
        if self.peek().tok == Tok::Dot {
            self.bump();
            let frac = self.peek().clone();
            match frac.tok {
                Tok::Int(digits) if digits >= 0 => {
                    self.bump();
                    hours.push('.');
                    hours.push_str(&digits.to_string());
                }
                other => {
                    return Err(err(
                        E0100_UNEXPECTED,
                        format!("expected the digits after '.' in the interval of `every`, found {}", describe(&other)),
                        frac.span,
                    )
                    .with_help("write e.g. `every 0.5 hours { ... }`"));
                }
            }
        }
        let unit = self.ident("`hours` after the number, e.g. `every 6 hours`")?;
        if unit.text != "hours" {
            return Err(err(E0100_UNEXPECTED, format!("`every` counts hours, so the unit is `hours`, not `{}`", unit.text), unit.span)
                .with_help("write e.g. `every 6 hours { ... }`, or `every day { ... }`"));
        }
        Ok(hours)
    }

    /// The `[(x, y), …]` corner list of `area <name>` (T-109). An empty or short list parses; the
    /// check says how many corners a polygon needs.
    fn area_points(&mut self, name: &str) -> Result<(Vec<AreaPoint>, Span), Diagnostic> {
        self.skip_newlines();
        let open = self.peek().clone();
        if open.tok != Tok::LBracket {
            return Err(err(
                E0100_UNEXPECTED,
                format!("expected '[' and the corner list after `area {name}`, found {}", describe(&open.tok)),
                open.span,
            )
            .with_help(format!("write `area {name} [(x, y), (x, y), (x, y)]`")));
        }
        self.bump();
        let mut points = Vec::new();
        loop {
            self.skip_newlines();
            let t = self.peek().clone();
            match t.tok {
                Tok::RBracket => {
                    self.bump();
                    return Ok((points, Span { start: open.span.start, end: t.span.end }));
                }
                Tok::LParen => {
                    self.bump();
                    self.skip_newlines();
                    let x = self.area_coord(name)?;
                    self.skip_newlines();
                    let sep = self.peek().clone();
                    if sep.tok != Tok::Comma {
                        return Err(err(
                            E0100_UNEXPECTED,
                            format!("expected ',' between x and y in `area {name}`, found {}", describe(&sep.tok)),
                            sep.span,
                        )
                        .with_help("write a corner as `(x, y)`"));
                    }
                    self.bump();
                    self.skip_newlines();
                    let y = self.area_coord(name)?;
                    self.skip_newlines();
                    let close = self.peek().clone();
                    if close.tok != Tok::RParen {
                        return Err(err(
                            E0100_UNEXPECTED,
                            format!("expected ')' after a corner of `area {name}`, found {}", describe(&close.tok)),
                            close.span,
                        )
                        .with_help("write a corner as `(x, y)`"));
                    }
                    self.bump();
                    points.push(AreaPoint { x, y });
                    self.skip_newlines();
                    let next = self.peek().clone();
                    match next.tok {
                        Tok::Comma => {
                            self.bump();
                        }
                        Tok::RBracket => {
                            self.bump();
                            return Ok((points, Span { start: open.span.start, end: next.span.end }));
                        }
                        other => {
                            return Err(err(
                                E0100_UNEXPECTED,
                                format!("expected ',' or ']' after a corner of `area {name}`, found {}", describe(&other)),
                                next.span,
                            ));
                        }
                    }
                }
                other => {
                    return Err(err(
                        E0100_UNEXPECTED,
                        format!("expected '(' or ']' in the corner list of `area {name}`, found {}", describe(&other)),
                        t.span,
                    )
                    .with_help("write the corners as `[(x, y), (x, y), (x, y)]`"));
                }
            }
        }
    }

    /// One corner coordinate of an `area` polygon (T-109): a whole or decimal number, possibly
    /// negative (`-96`, `26.4`; the lexer has no float token, so `26.4` is `Int(26) Dot Int(4)`).
    /// Scaled to hundredths, the fixed point `set_fixed_point_multiplier, 100` gives positions.
    fn area_coord(&mut self, name: &str) -> Result<i64, Diagnostic> {
        let t = self.peek().clone();
        if !matches!(t.tok, Tok::Int(_)) {
            return Err(err(
                E0100_UNEXPECTED,
                format!("expected a number in `area {name}`, found {}", describe(&t.tok)),
                t.span,
            )
            .with_help("write a corner as `(x, y)`, e.g. `(10, 20)` or `(-96, 26.4)`"));
        }
        // The token's own text keeps the sign and leading zeros the integer value drops (`-0.5`).
        let mut literal = self.text[t.span.start..t.span.end].to_string();
        self.bump();
        let mut end = t.span.end;
        if self.peek().tok == Tok::Dot {
            self.bump();
            let frac = self.peek().clone();
            if !matches!(frac.tok, Tok::Int(_)) {
                return Err(err(
                    E0100_UNEXPECTED,
                    format!("expected the digits after '.' in a number of `area {name}`, found {}", describe(&frac.tok)),
                    frac.span,
                )
                .with_help("write e.g. `(26.4, -12)`"));
            }
            literal.push('.');
            literal.push_str(&self.text[frac.span.start..frac.span.end]);
            end = frac.span.end;
            self.bump();
        }
        let value: f64 = literal.parse().map_err(|_| {
            err(E0103_BAD_TOKEN, format!("'{literal}' is not a number"), Span { start: t.span.start, end })
        })?;
        Ok((value * 100.0).round() as i64)
    }

    /// The `(:p1, :p2, …)` parameters of `fn <name>`; each one is a `:local` the body uses.
    fn params(&mut self, name: &str) -> Result<Vec<Ident>, Diagnostic> {
        self.skip_newlines();
        let t = self.peek().clone();
        if t.tok != Tok::LParen {
            return Err(err(
                E0100_UNEXPECTED,
                format!("expected '(' after `fn {name}`, found {}", describe(&t.tok)),
                t.span,
            )
            .with_help(format!("write `fn {name}(:amount) {{ … }}`, or `fn {name}() {{ … }}`")));
        }
        self.bump();
        let mut params = Vec::new();
        loop {
            self.skip_newlines();
            let t = self.peek().clone();
            match t.tok {
                Tok::RParen => {
                    self.bump();
                    return Ok(params);
                }
                Tok::Local(text) => {
                    self.bump();
                    params.push(Ident { text, span: t.span });
                    self.skip_newlines();
                    let next = self.peek().clone();
                    match next.tok {
                        Tok::Comma => {
                            self.bump();
                        }
                        Tok::RParen => {}
                        other => {
                            return Err(err(
                                E0100_UNEXPECTED,
                                format!("expected ',' or ')' in the parameters of `fn {name}`, found {}", describe(&other)),
                                next.span,
                            ));
                        }
                    }
                }
                Tok::Eof | Tok::LBrace | Tok::RBrace => {
                    return Err(err(E0101_UNCLOSED, format!("this '(' of `fn {name}` is never closed"), t.span)
                        .with_help("add the missing ')'"));
                }
                other => {
                    return Err(err(
                        E0100_UNEXPECTED,
                        format!("expected a parameter like `:amount` in `fn {name}`, found {}", describe(&other)),
                        t.span,
                    )
                    .with_help("parameters are locals, e.g. `fn give_gold(:amount) { … }`"));
                }
            }
        }
    }

    /// The next token that is not a line end (looking past newlines without reading them).
    fn peek_after_newlines(&self) -> &Token {
        let mut i = self.pos;
        while self.toks[i].tok == Tok::Newline {
            i += 1;
        }
        &self.toks[i]
    }

    /// A `"…"` text literal.
    fn text_lit(&mut self, what: &str) -> Result<(String, Span), Diagnostic> {
        let t = self.peek().clone();
        match t.tok {
            Tok::Str(text) => {
                self.bump();
                Ok((text, t.span))
            }
            other => Err(
                err(E0100_UNEXPECTED, format!("expected {what}, found {}", describe(&other)), t.span)
                    .with_help("write the text in double quotes, e.g. \"New text\""),
            ),
        }
    }

    /// The `{ … }` body of `override` / `new`: `on` handlers and menu members (§4).
    fn record_body(&mut self, head: &str) -> Result<(Vec<Handler>, Vec<Member>, Span), Diagnostic> {
        let open = self.open_brace(head)?;
        let mut handlers = Vec::new();
        let mut members = Vec::new();
        // The option ids written so far in this menu: a text-derived id (`option "Text"`) has to be
        // unique next to them, so it gets `_2`, `_3` … (T-091).
        let mut used: Vec<String> = Vec::new();
        loop {
            self.skip_separators();
            let t = self.peek().clone();
            match &t.tok {
                Tok::RBrace => {
                    self.bump();
                    return Ok((handlers, members, Span { start: open.start, end: t.span.end }));
                }
                Tok::Eof => return Err(self.unclosed(open, &format!("`{head}`"))),
                // A top-level item inside the block: the block lost its `}`.
                Tok::Ident(kw)
                    if kw == "override"
                        || kw == "mod"
                        || kw == "new"
                        || kw == "in"
                        || (kw == "on" && self.toks.get(self.pos + 2).is_some_and(|t| t.tok == Tok::Dot)) =>
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
                Tok::Ident(kw) if matches!(kw.as_str(), "text" | "option" | "change" | "remove") => {
                    members.push(self.member(&mut used)?);
                }
                other => {
                    return Err(err(
                        E0100_UNEXPECTED,
                        format!(
                            "expected `on <event> {{`, `text`, `option`, `change option` or `remove option` inside `{head}`, found {}",
                            describe(other)
                        ),
                        t.span,
                    )
                    .with_help("for example `on enter { ... }`, `text \"New text\"` or `option uw_skip \"Skip\" { ... }`"));
                }
            }
        }
    }

    /// The `{ <field> <value> … }` body of a `<kind> … like …` item (`troop`, `town`, …): one
    /// record field per line (T-092, T-099).
    fn copy_body(&mut self, head: &str) -> Result<(Vec<FieldValue>, Span), Diagnostic> {
        let open = self.open_brace(head)?;
        let mut fields = Vec::new();
        loop {
            self.skip_separators();
            let t = self.peek().clone();
            match &t.tok {
                Tok::RBrace => {
                    self.bump();
                    return Ok((fields, Span { start: open.start, end: t.span.end }));
                }
                Tok::Eof => return Err(self.unclosed(open, &format!("`{head}`"))),
                // A top-level item inside the block: the block lost its `}`.
                Tok::Ident(kw) if kw == "mod" || kw == "override" || kw == "new" || is_copy_kind(kw) => {
                    return Err(self
                        .unclosed(open, &format!("`{head}`"))
                        .with_note(format!("line {} starts `{kw}` while this block is still open", self.line_of(t.span.start))));
                }
                Tok::Ident(_) => fields.push(self.field_value(head)?),
                other => {
                    return Err(err(
                        E0100_UNEXPECTED,
                        format!("expected a field line like `level 22` inside `{head}`, found {}", describe(other)),
                        t.span,
                    )
                    .with_help("write one `<field> <value>` per line, e.g. `name \"Elite Crossbowman\"`"));
                }
            }
        }
    }

    /// One `<field> <value>` line of a `<kind> … like …` body. The value stays exactly as the source
    /// writes it (record text: a number, a quoted text, a `{a 1, b 2}`), so `uw_decomp`'s record
    /// reader of the kit-meta schema reads it the way record text is read.
    fn field_value(&mut self, head: &str) -> Result<FieldValue, Diagnostic> {
        let name = self.ident("a field name, e.g. `name`, `level` or `agility`")?;
        let t = self.peek().clone();
        if matches!(t.tok, Tok::Newline | Tok::Semi | Tok::RBrace | Tok::Eof) {
            return Err(err(E0100_UNEXPECTED, format!("field `{}` of `{head}` has no value", name.text), t.span)
                .with_help("write a value after the field name, e.g. `level 22`"));
        }
        let start = t.span.start;
        let mut end = start;
        let mut depth = 0usize;
        loop {
            let t = self.peek().clone();
            match t.tok {
                Tok::LBrace | Tok::LParen => depth += 1,
                Tok::RBrace | Tok::RParen if depth > 0 => depth -= 1,
                // The line ends here: a `}` at depth 0 closes the body of the item.
                Tok::RBrace | Tok::Newline | Tok::Semi | Tok::Eof => break,
                _ => {}
            }
            end = t.span.end;
            self.bump();
        }
        let value = self.text[start..end].trim().to_owned();
        Ok(FieldValue { name: name.clone(), value, span: Span { start: name.span.start, end } })
    }

    /// The `{ <speaker>: "text" … }` body of a `dialog` item (T-102).
    fn dialog_body(&mut self, head: &str) -> Result<(Vec<DialogLine>, Span), Diagnostic> {
        let open = self.open_brace(head)?;
        let mut lines = Vec::new();
        loop {
            self.skip_separators();
            let t = self.peek().clone();
            match &t.tok {
                Tok::RBrace => {
                    self.bump();
                    return Ok((lines, Span { start: open.start, end: t.span.end }));
                }
                Tok::Eof => return Err(self.unclosed(open, &format!("`{head}`"))),
                // A top-level item inside the block: the block lost its `}`.
                Tok::Ident(kw) if kw == "mod" || kw == "override" || kw == "new" || kw == "in" || kw == "dialog" => {
                    return Err(self
                        .unclosed(open, &format!("`{head}`"))
                        .with_note(format!("line {} starts `{kw}` while this block is still open", self.line_of(t.span.start))));
                }
                Tok::Ident(_) => lines.push(self.dialog_line(head)?),
                other => {
                    return Err(err(
                        E0100_UNEXPECTED,
                        format!("expected a line like `harlaus: \"Who are you?\"` inside `{head}`, found {}", describe(other)),
                        t.span,
                    )
                    .with_help("write `harlaus: \"Who are you?\"` or `player: \"Just a traveller.\" -> end`"));
                }
            }
        }
    }

    /// One `<speaker>: "<text>" [when <cond>] [{ <block> }] [-> end]` line of a `dialog` body.
    fn dialog_line(&mut self, head: &str) -> Result<DialogLine, Diagnostic> {
        let speaker = self.ident("the speaker of this line, e.g. `harlaus` or `player`")?;
        let t = self.peek().clone();
        if t.tok != Tok::Colon {
            return Err(err(
                E0100_UNEXPECTED,
                format!("expected ':' and the text after `{}`, found {}", speaker.text, describe(&t.tok)),
                t.span,
            )
            .with_help(format!("write `{}: \"...\"`", speaker.text)));
        }
        self.bump();
        let (text, lit) = self.text_lit(&format!("the text of this line of `{head}`, in double quotes"))?;
        // `when` may follow on this line or the next one, as in an `option` (`when` ends at the
        // body's `{`, §7.5).
        let save = self.pos;
        self.skip_newlines();
        let mut when = None;
        if matches!(&self.peek().tok, Tok::Ident(word) if word == "when") {
            self.bump();
            when = Some(self.when_part()?);
        } else {
            self.pos = save;
        }
        // A `{ … }` after the condition is what picking this line does.
        let body = {
            let save = self.pos;
            self.skip_newlines();
            if self.peek().tok == Tok::LBrace {
                Some(self.block(&format!("this line of `{head}`"))?)
            } else {
                self.pos = save;
                None
            }
        };
        let save = self.pos;
        self.skip_newlines();
        let mut to = None;
        if self.peek().tok == Tok::Arrow {
            self.bump();
            to = Some(self.ident("`end` after `->`")?);
        } else {
            self.pos = save;
        }
        self.end_of_stmt(&format!("this line of `{head}`"))?;
        let end = match (&to, &body, &when) {
            (Some(to), ..) => to.span.end,
            (None, Some(body), _) => body.span.end,
            (None, None, Some(When::Cond(cond))) => cond_span(cond).end,
            (None, None, Some(When::Block(block))) => block.span.end,
            (None, None, None) => lit.end,
        };
        let start = speaker.span.start;
        Ok(DialogLine { speaker, text, when, body, to, span: Span { start, end } })
    }

    /// One menu line of an `override` / `new` body: `text`, `option`, `change option` or
    /// `remove option` (§4). The keyword is not read yet. `used` holds the option ids already
    /// written in this menu (see [`option_slug`]).
    fn member(&mut self, used: &mut Vec<String>) -> Result<Member, Diagnostic> {
        let kw_tok = self.bump();
        let kw = match &kw_tok.tok {
            Tok::Ident(kw) => kw.clone(),
            _ => return Err(err(E0100_UNEXPECTED, "expected a menu line", kw_tok.span)),
        };
        match kw.as_str() {
            "text" => {
                let (text, lit) = self.text_lit("a text in double quotes after `text`")?;
                self.end_of_stmt("this `text` line")?;
                Ok(Member::Text { text, span: Span { start: kw_tok.span.start, end: lit.end } })
            }
            "option" => {
                // `option "Text" { … }`: without an id, the id is made from the text (§2, T-091).
                let given = if matches!(self.peek().tok, Tok::Str(_)) {
                    None
                } else {
                    Some(self.ident("an option id after `option`, e.g. `option uw_skip \"Skip\" {`")?)
                };
                let what = match &given {
                    Some(id) => format!("a text in double quotes after `option {}`", id.text),
                    None => "a text in double quotes after `option`".to_string(),
                };
                let (text, text_span) = self.text_lit(&what)?;
                let id = match given {
                    Some(id) => {
                        used.push(id.text.clone());
                        id
                    }
                    None => {
                        let base = option_slug(&text).unwrap_or_else(|| "uw_option".to_string());
                        let mut id = base.clone();
                        let mut n = 2;
                        while used.contains(&id) {
                            id = format!("{base}_{n}");
                            n += 1;
                        }
                        used.push(id.clone());
                        Ident { text: id, span: text_span }
                    }
                };
                let when = if matches!(&self.peek().tok, Tok::Ident(w) if w == "when") {
                    self.bump();
                    Some(self.when_part()?)
                } else {
                    None
                };
                let t = self.peek().clone();
                let place = match &t.tok {
                    Tok::Ident(w) if w == "after" => {
                        self.bump();
                        Some(Place::After(self.ident("an option id after `after`")?))
                    }
                    Tok::Ident(w) if w == "before" => {
                        self.bump();
                        Some(Place::Before(self.ident("an option id after `before`")?))
                    }
                    _ => None,
                };
                let body = self.block(&format!("option {}", id.text))?;
                let span = Span { start: kw_tok.span.start, end: body.span.end };
                self.end_of_stmt("this `option` line")?;
                Ok(Member::Option { id, text, when, place, body, span })
            }
            "change" | "remove" => {
                let word = self.ident(&format!("`option` after `{kw}`"))?;
                if word.text != "option" {
                    return Err(err(
                        E0100_UNEXPECTED,
                        format!("expected `option` after `{kw}`, found '{}'", word.text),
                        word.span,
                    )
                    .with_help(format!("write `{kw} option <id>`")));
                }
                let id = self.ident(&format!("an option id after `{kw} option`"))?;
                if kw == "remove" {
                    let span = Span { start: kw_tok.span.start, end: id.span.end };
                    self.end_of_stmt("this `remove option` line")?;
                    return Ok(Member::RemoveOption { id, span });
                }
                let head = format!("change option {}", id.text);
                let open = self.open_brace(&head)?;
                let mut text: Option<(String, Span)> = None;
                let mut when: Option<When> = None;
                let mut body: Option<Block> = None;
                let end;
                loop {
                    self.skip_separators();
                    let t = self.peek().clone();
                    let part = match &t.tok {
                        Tok::RBrace => {
                            end = t.span.end;
                            self.bump();
                            break;
                        }
                        Tok::Eof => return Err(self.unclosed(open, &format!("`{head}`"))),
                        Tok::Ident(w) if matches!(w.as_str(), "text" | "when" | "do") => {
                            self.bump();
                            w.clone()
                        }
                        other => {
                            return Err(err(
                                E0100_UNEXPECTED,
                                format!("expected `text`, `when` or `do` inside `{head}`, found {}", describe(other)),
                                t.span,
                            )
                            .with_help("for example `text \"New text\"`, `when $gold > 0` or `do { ... }`"));
                        }
                    };
                    match part.as_str() {
                        "text" => {
                            if let Some((_, first)) = text.as_ref() {
                                return Err(err(
                                    E0100_UNEXPECTED,
                                    format!("this `{head}` already sets `text` on line {}", self.line_of(first.start)),
                                    t.span,
                                )
                                .with_help("keep one `text` line"));
                            }
                            text = Some(self.text_lit("a text in double quotes after `text`")?);
                        }
                        "when" => {
                            if let Some(first) = when.as_ref() {
                                let first = match first {
                                    When::Cond(cond) => cond_span(cond),
                                    When::Block(block) => block.span,
                                };
                                return Err(err(
                                    E0100_UNEXPECTED,
                                    format!("this `{head}` already sets `when` on line {}", self.line_of(first.start)),
                                    t.span,
                                )
                                .with_help("keep one `when` line"));
                            }
                            when = Some(self.when_part()?);
                        }
                        _ => {
                            if let Some(first) = body.as_ref() {
                                return Err(err(
                                    E0100_UNEXPECTED,
                                    format!("this `{head}` already sets `do` on line {}", self.line_of(first.span.start)),
                                    t.span,
                                )
                                .with_help("keep one `do` block"));
                            }
                            body = Some(self.block("do")?);
                        }
                    }
                }
                self.end_of_stmt(&format!("this `{head}`"))?;
                Ok(Member::ChangeOption { id, text, when, body, span: Span { start: kw_tok.span.start, end } })
            }
            other => Err(err(
                E0100_UNEXPECTED,
                format!("`{other}` is not a menu line"),
                kw_tok.span,
            )
            .with_help("menu lines are `text`, `option`, `change option` and `remove option`")),
        }
    }

    /// `when { … }` (always the block form) or `when <cond>`, which ends at the body's `{` (§7.5).
    /// The `when` keyword is already read.
    fn when_part(&mut self) -> Result<When, Diagnostic> {
        self.skip_newlines();
        let t = self.peek().clone();
        match &t.tok {
            Tok::LBrace => Ok(When::Block(self.block("when")?)),
            Tok::RBrace | Tok::Semi | Tok::Eof => Err(err(
                E0100_UNEXPECTED,
                format!("expected a condition or '{{' after `when`, found {}", describe(&t.tok)),
                t.span,
            )
            .with_help("write `when $gold > 0` or `when { ... }`")),
            _ => Ok(When::Cond(self.cond()?)),
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
                Tok::Ident(_) | Tok::Global(_) | Tok::Local(_) => stmts.push(self.stmt()?),
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

    /// The statements of a block body without its braces (a decompiler prints a body this way).
    fn block_body(&mut self) -> Result<Vec<Stmt>, Diagnostic> {
        let mut stmts = Vec::new();
        loop {
            self.skip_separators();
            let t = self.peek().clone();
            match &t.tok {
                Tok::Eof => return Ok(stmts),
                Tok::RBrace => {
                    return Err(err(E0102_STRAY_CLOSE, "this '}' has no matching '{'", t.span)
                        .with_help("remove it, or check which block above was closed early"));
                }
                Tok::Ident(_) | Tok::Global(_) | Tok::Local(_) => stmts.push(self.stmt()?),
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
        // `$g = …` / `:l += …`
        if matches!(self.peek().tok, Tok::Global(_) | Tok::Local(_)) {
            let t = self.bump();
            let target = match t.tok {
                Tok::Global(text) => Arg::Global(Ident { text, span: t.span }),
                Tok::Local(text) => Arg::Local(Ident { text, span: t.span }),
                _ => unreachable!(),
            };
            return self.assign_after(target);
        }
        let op = self.ident("an operation name")?;
        self.check_flag_prefix(&op)?;
        if op.text == "if" {
            return self.if_stmt(op);
        }
        if op.text == "require" {
            return self.require_stmt(op);
        }
        if op.text == "attempt" {
            return self.attempt_stmt(op);
        }
        if op.text == "for" {
            return self.for_stmt(op);
        }
        if matches!(op.text.as_str(), "elif" | "else" | "while") {
            return Err(self.keyword_error(op));
        }
        // `player.gold += 1000`: a dotted word is one target (T-082).
        let op = self.dotted_after(op)?;
        if matches!(self.peek().tok, Tok::Assign(_)) {
            return self.assign_after(Arg::Name(op));
        }
        let call = self.call_after(op)?;
        self.end_of_stmt(&format!("`{}(...)`", call.op.text))?;
        Ok(Stmt::Call(call))
    }

    /// The arguments of `name(...)`; the name is already read.
    fn call_after(&mut self, op: Ident) -> Result<Call, Diagnostic> {
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
                    .with_help("operation calls are plain lines, e.g. `name(a, b)`"));
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
        let span = Span { start: op.span.start, end };
        Ok(Call { op, args, span })
    }

    /// One statement per line (or `;`).
    fn end_of_stmt(&mut self, what: &str) -> Result<(), Diagnostic> {
        match self.peek().tok {
            Tok::Newline | Tok::Semi | Tok::RBrace | Tok::Eof => Ok(()),
            ref other => Err(err(
                E0100_UNEXPECTED,
                format!("expected a new line or ';' after {what}, found {}", describe(other)),
                self.peek().span,
            )),
        }
    }

    /// `target = expr`: the target is already read, the assignment operator must be next.
    fn assign_after(&mut self, target: Arg) -> Result<Stmt, Diagnostic> {
        let t = self.peek().clone();
        let op = match t.tok {
            Tok::Assign(op) => op,
            ref other => {
                return Err(err(
                    E0100_UNEXPECTED,
                    format!("expected an assignment operator after this target, found {}", describe(other)),
                    t.span,
                )
                .with_help("write `target = value`; `+= -= *= /= %=` also work"));
            }
        };
        self.bump();
        // A line that ends with `=` continues on the next one (§7.5).
        self.skip_newlines();
        let value = self.expr()?;
        let span = Span { start: target.span().start, end: expr_span(&value).end };
        self.end_of_stmt("this assignment")?;
        Ok(Stmt::Assign { target, op, value, span })
    }

    /// `elif` / `else` / `while` written as a statement; each gets a one-line fix.
    fn keyword_error(&self, op: Ident) -> Diagnostic {
        let python = self.colon_on_this_line();
        let d = err(
            if python { E0104_PYTHON_BLOCK } else { E0100_UNEXPECTED },
            format!("`{}` is not a UW statement", op.text),
            op.span,
        );
        match op.text.as_str() {
            "elif" => d.with_help("write `else if <condition> { ... }` instead of `elif`"),
            "else" => d.with_help("write `else { ... }`; `else` must follow the `}` of an `if` block"),
            _ => d.with_help("UW has no `while` loop; count with `for :i in range(0, 10) { ... }`"),
        }
    }

    /// A `:` before the end of this line — the Python block habit (§ Sözdizimi).
    fn colon_on_this_line(&self) -> bool {
        self.toks[self.pos..]
            .iter()
            .take_while(|t| t.tok != Tok::Newline && t.tok != Tok::Eof)
            .any(|t| t.tok == Tok::Colon)
    }

    /// `if <cond> { } else if <cond> { } else { }`.
    fn if_stmt(&mut self, kw: Ident) -> Result<Stmt, Diagnostic> {
        let start = kw.span.start;
        let mut arms = Vec::new();
        let mut otherwise = None;
        loop {
            let cond = self.cond()?;
            let body = self.block("if <condition>")?;
            arms.push((cond, body));
            // `else` may follow the `}` directly or on the next line.
            let save = self.pos;
            self.skip_newlines();
            let t = self.peek().clone();
            if let Tok::Ident(word) = &t.tok {
                if word == "elif" {
                    return Err(self.keyword_error(Ident { text: word.clone(), span: t.span }));
                }
            }
            if !matches!(&t.tok, Tok::Ident(word) if word == "else") {
                self.pos = save;
                break;
            }
            self.bump();
            if matches!(&self.peek().tok, Tok::Ident(word) if word == "if") {
                self.bump();
                continue;
            }
            otherwise = Some(self.block("else")?);
            break;
        }
        let end = match &otherwise {
            Some(body) => body.span.end,
            None => arms.last().map_or(start, |(_, body)| body.span.end),
        };
        self.end_of_stmt("this `if` block")?;
        Ok(Stmt::If { arms, otherwise, span: Span { start, end } })
    }

    /// `require <cond>`: one statement, ends at a new line or `;` (§4.2).
    fn require_stmt(&mut self, kw: Ident) -> Result<Stmt, Diagnostic> {
        let cond = self.cond()?;
        let span = Span { start: kw.span.start, end: cond_span(&cond).end };
        self.end_of_stmt("this `require`")?;
        Ok(Stmt::Require { cond, span })
    }

    /// `attempt { } else attempt { } else { }` (§4.3): each block is one arm, the last one may be
    /// a plain `else { }`.
    fn attempt_stmt(&mut self, kw: Ident) -> Result<Stmt, Diagnostic> {
        let start = kw.span.start;
        let mut arms = vec![self.block("attempt")?];
        loop {
            // `else` may follow the `}` directly or on the next line.
            let save = self.pos;
            self.skip_newlines();
            let t = self.peek().clone();
            if !matches!(&t.tok, Tok::Ident(word) if word == "else") {
                self.pos = save;
                break;
            }
            self.bump();
            // An arm carries no condition of its own: `else if` is an `if` habit.
            let next = self.peek().clone();
            if matches!(&next.tok, Tok::Ident(word) if word == "if" || word == "elif") {
                return Err(err(E0100_UNEXPECTED, "`else if` cannot follow an `attempt` block", next.span)
                    .with_help("write `else attempt { ... }` here and put `require <condition>` inside the arm"));
            }
            if matches!(&next.tok, Tok::Ident(word) if word == "attempt") {
                self.bump();
                arms.push(self.block("attempt")?);
                continue;
            }
            arms.push(self.block("else")?);
            break;
        }
        let end = arms.last().map_or(kw.span.end, |body| body.span.end);
        self.end_of_stmt("this `attempt` block")?;
        Ok(Stmt::Attempt { arms, span: Span { start, end } })
    }

    /// `for <$global|:local> in <kind>(<args>) { … }` (§8); `kind` is a `try_for_` operation
    /// without that prefix (`range`, `range_backwards`, `parties`, `agents` …).
    fn for_stmt(&mut self, kw: Ident) -> Result<Stmt, Diagnostic> {
        let start = kw.span.start;
        self.skip_newlines();
        let t = self.peek().clone();
        let var = match t.tok {
            Tok::Global(text) => Arg::Global(Ident { text, span: t.span }),
            Tok::Local(text) => Arg::Local(Ident { text, span: t.span }),
            Tok::Ident(text) => {
                // A loop variable carries `$` or `:`; a `:` at the end of the line is Python.
                let d = if self.colon_on_this_line() {
                    err(E0104_PYTHON_BLOCK, format!("`for {text} …:` is not a UW loop; there is no ':' before a block"), t.span)
                } else {
                    err(E0100_UNEXPECTED, format!("`{text}` is not a loop variable: it needs `$` or `:`"), t.span)
                };
                return Err(d.with_help(format!("write `for :{text} in range(0, 10) {{ ... }}`")));
            }
            other => {
                return Err(err(
                    E0100_UNEXPECTED,
                    format!("expected a loop variable like `:i` after `for`, found {}", describe(&other)),
                    t.span,
                )
                .with_help("write `for :i in range(0, 10) { ... }`"));
            }
        };
        self.bump();
        self.skip_newlines();
        let t = self.peek().clone();
        if !matches!(&t.tok, Tok::Ident(word) if word == "in") {
            return Err(err(
                E0100_UNEXPECTED,
                format!("expected `in` after the loop variable, found {}", describe(&t.tok)),
                t.span,
            )
            .with_help("write `for :i in range(0, 10) { ... }`"));
        }
        self.bump();
        self.skip_newlines();
        let kind = self.ident("the loop kind after `in`, e.g. `range`")?;
        let call = self.call_after(kind.clone())?;
        let var_text = self.text[var.span().start..var.span().end].to_owned();
        let body = self.block(&format!("for {var_text} in {}", kind.text))?;
        self.end_of_stmt("this `for` loop")?;
        let span = Span { start, end: body.span.end };
        Ok(Stmt::For { var, kind, args: call.args, body, span })
    }

    /// `a < b`, `not c`, `x and y or z` — precedence `not` > `and` > `or` (§7.3).
    fn cond(&mut self) -> Result<Cond, Diagnostic> {
        let mut parts = vec![self.and_cond()?];
        while matches!(&self.peek().tok, Tok::Ident(word) if word == "or") {
            self.bump();
            self.skip_newlines();
            parts.push(self.and_cond()?);
        }
        Ok(if parts.len() == 1 { parts.pop().unwrap() } else { Cond::Or(parts) })
    }

    fn and_cond(&mut self) -> Result<Cond, Diagnostic> {
        let mut parts = vec![self.not_cond()?];
        while matches!(&self.peek().tok, Tok::Ident(word) if word == "and") {
            self.bump();
            self.skip_newlines();
            parts.push(self.not_cond()?);
        }
        Ok(if parts.len() == 1 { parts.pop().unwrap() } else { Cond::And(parts) })
    }

    fn not_cond(&mut self) -> Result<Cond, Diagnostic> {
        if matches!(&self.peek().tok, Tok::Ident(word) if word == "not") {
            let start = self.bump().span.start;
            self.skip_newlines();
            let inner = self.not_cond()?;
            let span = Span { start, end: cond_span(&inner).end };
            return Ok(Cond::Not(Box::new(inner), span));
        }
        if self.peek().tok == Tok::LParen {
            if let Some(grouped) = self.grouped_cond() {
                return Ok(grouped);
            }
        }
        self.cmp_cond()
    }

    /// `( <condition> )`, e.g. `a and (b or c)`. Backtracks (returns `None`, position unchanged)
    /// when the parentheses hold arithmetic instead: `(a + b) > 3`, `(a) == 1`.
    fn grouped_cond(&mut self) -> Option<Cond> {
        let saved = self.pos;
        self.bump();
        self.skip_newlines();
        let inner = self.cond().ok();
        self.skip_newlines();
        if let Some(inner) = inner {
            if self.peek().tok == Tok::RParen {
                self.bump();
                if !matches!(self.peek().tok, Tok::Bin(_) | Tok::Cmp(_)) {
                    return Some(inner);
                }
            }
        }
        self.pos = saved;
        None
    }

    fn cmp_cond(&mut self) -> Result<Cond, Diagnostic> {
        let left = self.cond_operand()?;
        let t = self.peek().clone();
        match left {
            CondOperand::Call(call) => {
                if let Tok::Cmp(_) = t.tok {
                    return Err(err(
                        E0100_UNEXPECTED,
                        format!("expected a full condition after this operation, found {}", describe(&t.tok)),
                        t.span,
                    )
                    .with_help("a condition operation is used on its own, not inside a comparison"));
                }
                Ok(Cond::Call(call))
            }
            CondOperand::Expr(left) => {
                let op = match t.tok {
                    Tok::Cmp(op) => op,
                    ref other => {
                        return Err(err(
                            E0100_UNEXPECTED,
                            format!("a condition needs a comparison, found {}", describe(other)),
                            t.span,
                        )
                        .with_help("write `if <value> < <= > >= == != <value> { ... }`, or call a condition operation"));
                    }
                };
                self.bump();
                self.skip_newlines();
                let right = self.expr()?;
                let span = Span { start: expr_span(&left).start, end: expr_span(&right).end };
                Ok(Cond::Cmp { op, left, right, span })
            }
        }
    }

    /// The left side of a condition: a call, a bare condition operation, or an expression.
    fn cond_operand(&mut self) -> Result<CondOperand, Diagnostic> {
        let t = self.peek().clone();
        if let Tok::Ident(name) = &t.tok {
            let after = self.toks.get(self.pos + 1).map(|t| &t.tok);
            let op = Ident { text: name.clone(), span: t.span };
            self.check_flag_prefix(&op)?;
            if matches!(after, Some(Tok::LParen)) {
                self.bump();
                return Ok(CondOperand::Call(self.call_after(op)?));
            }
            // A bare name is a condition operation without operands (`if game_over {`); a dotted
            // word is a value instead (`if player.gold >= 100`, T-082).
            if !matches!(after, Some(Tok::Bin(_)) | Some(Tok::Cmp(_)) | Some(Tok::Dot)) {
                self.bump();
                return Ok(CondOperand::Call(Call { span: t.span, op, args: Vec::new() }));
            }
        }
        Ok(CondOperand::Expr(self.expr()?))
    }

    /// An integer expression: `+ -` lowest, then `* / %`, then unary `-`.
    fn expr(&mut self) -> Result<Expr, Diagnostic> {
        let mut left = self.mul()?;
        loop {
            let op = match self.peek().tok {
                Tok::Bin(BinOp::Add) => BinOp::Add,
                Tok::Bin(BinOp::Sub) => BinOp::Sub,
                _ => break,
            };
            self.bump();
            self.skip_newlines();
            let right = self.mul()?;
            let span = Span { start: expr_span(&left).start, end: expr_span(&right).end };
            left = Expr::Bin { op, left: Box::new(left), right: Box::new(right), span };
        }
        Ok(left)
    }

    fn mul(&mut self) -> Result<Expr, Diagnostic> {
        let mut left = self.unary()?;
        loop {
            let op = match self.peek().tok {
                Tok::Bin(BinOp::Mul) => BinOp::Mul,
                Tok::Bin(BinOp::Div) => BinOp::Div,
                Tok::Bin(BinOp::Mod) => BinOp::Mod,
                _ => break,
            };
            self.bump();
            self.skip_newlines();
            let right = self.unary()?;
            let span = Span { start: expr_span(&left).start, end: expr_span(&right).end };
            left = Expr::Bin { op, left: Box::new(left), right: Box::new(right), span };
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, Diagnostic> {
        if self.peek().tok == Tok::Bin(BinOp::Sub) {
            let start = self.bump().span.start;
            self.skip_newlines();
            let inner = self.unary()?;
            let end = expr_span(&inner).end;
            return Ok(match inner {
                Expr::Value(Arg::Int(n, _)) => Expr::Value(Arg::Int(-n, Span { start, end })),
                other => Expr::Neg(Box::new(other), Span { start, end }),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, Diagnostic> {
        let t = self.peek().clone();
        // `player.gold`: a dotted word is one value (T-082).
        if let Tok::Ident(text) = &t.tok {
            self.bump();
            let name = self.dotted_after(Ident { text: text.clone(), span: t.span })?;
            return Ok(Expr::Value(Arg::Name(name)));
        }
        let value = match t.tok {
            Tok::Int(n) => Arg::Int(n, t.span),
            Tok::Str(s) => Arg::Str(s, t.span),
            Tok::Global(text) => Arg::Global(Ident { text, span: t.span }),
            Tok::Local(text) => Arg::Local(Ident { text, span: t.span }),
            Tok::LParen => {
                self.bump();
                self.skip_newlines();
                let inner = self.expr()?;
                self.skip_newlines();
                let close = self.peek().clone();
                match close.tok {
                    Tok::RParen => {
                        self.bump();
                        return Ok(inner);
                    }
                    Tok::Eof | Tok::LBrace | Tok::RBrace => {
                        return Err(err(E0101_UNCLOSED, "this '(' is never closed", t.span)
                            .with_help("add the missing ')'"));
                    }
                    other => {
                        return Err(err(E0100_UNEXPECTED, format!("expected ')', found {}", describe(&other)), close.span));
                    }
                }
            }
            other => {
                return Err(err(E0100_UNEXPECTED, format!("expected a value, found {}", describe(&other)), t.span)
                    .with_help("values are names (p_town_6), $globals, :locals, numbers or \"text\""));
            }
        };
        self.bump();
        Ok(Expr::Value(value))
    }
}

/// One side of a condition, before the comparison operator (if any) is known.
enum CondOperand {
    Expr(Expr),
    Call(Call),
}

/// The span of an expression (the AST keeps no helper for it).
fn expr_span(expr: &Expr) -> Span {
    match expr {
        Expr::Value(arg) => arg.span(),
        Expr::Neg(_, span) => *span,
        Expr::Bin { span, .. } => *span,
    }
}

/// The span of a condition (`Cond::And`/`Cond::Or` keep none).
fn cond_span(cond: &Cond) -> Span {
    match cond {
        Cond::Cmp { span, .. } => *span,
        Cond::Call(call) => call.span,
        Cond::Not(_, span) => *span,
        Cond::And(parts) | Cond::Or(parts) => {
            let first = parts.first().map_or(Span { start: 0, end: 0 }, cond_span);
            let last = parts.last().map_or(first, cond_span);
            Span { start: first.start, end: last.end }
        }
    }
}

/// Parses one source file. The first error stops the parse.
pub fn parse(text: &str) -> Result<File, Diagnostic> {
    let toks = lex(text)?;
    Parser { toks, pos: 0, text: text.to_owned() }.file()
}

/// Parses the statements of a block body written without its `{ }`, same rules as inside a block.
pub fn parse_block(text: &str) -> Result<Vec<Stmt>, Diagnostic> {
    let toks = lex(text)?;
    Parser { toks, pos: 0, text: text.to_owned() }.block_body()
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
        let Item::Override { kind: Some(kind), name, handlers, .. } = &file.items[0] else { panic!() };
        assert_eq!((kind.text.as_str(), name.text.as_str()), ("menu", "start_phase_2"));
        assert_eq!(handlers[0].event.text, "enter");
        let stmts = &handlers[0].body.stmts;
        assert_eq!(stmts[0].as_call().unwrap().op.text, "party_relocate_near_party");
        assert_eq!(stmts[0].as_call().unwrap().args.len(), 3);
        assert!(matches!(&stmts[0].as_call().unwrap().args[2], Arg::Int(3, _)));
        assert!(stmts[1].as_call().unwrap().args.is_empty());
        let Item::On { path, body, .. } = &file.items[1] else { panic!() };
        assert_eq!(path.iter().map(|p| p.text.as_str()).collect::<Vec<_>>(), ["game", "start"]);
        assert!(matches!(&body.stmts[0].as_call().unwrap().args[0], Arg::Global(g) if g.text == "$g_x"));
        assert!(matches!(&body.stmts[1].as_call().unwrap().args[0], Arg::Str(s, _) if s == "@hi"));
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
                Item::New { handlers, .. } => handlers.into_iter().map(|h| h.body).collect(),
                Item::Fn { body, .. } => vec![body],
                Item::Save { .. } => Vec::new(),
                Item::Every { body, .. } => vec![body],
                Item::Area { .. } => Vec::new(),
                Item::AreaEnter { body, .. } => vec![body],
                Item::Like { .. } => Vec::new(),
                Item::Dialog { lines, .. } => lines.into_iter().filter_map(|l| l.body).collect(),
            };
            for b in blocks {
                for s in b.stmts.iter().filter_map(Stmt::as_call) {
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
        assert_eq!(body.stmts[0].as_call().unwrap().args.len(), 2);
        assert_eq!(body.stmts[1].as_call().unwrap().op.text, "g");
    }

    #[test]
    fn negative_numbers_and_locals() {
        let file = parse("on game.start { f(-5, :loc) }").unwrap();
        let Item::On { body, .. } = &file.items[0] else { panic!() };
        assert!(matches!(body.stmts[0].as_call().unwrap().args[0], Arg::Int(-5, _)));
        assert!(matches!(&body.stmts[0].as_call().unwrap().args[1], Arg::Local(l) if l.text == ":loc"));
    }

    /// T-088: the interval of `every` in game hours, as the kit's simple_triggers record holds it.
    #[test]
    fn every_reads_a_day_an_hour_or_a_number_of_hours() {
        let file = parse("every day { a() }\nevery hour { b() }\nevery 6 hours { c() }\nevery 0.5 hours { d() }\n").unwrap();
        let intervals: Vec<&str> = file
            .items
            .iter()
            .map(|item| match item {
                Item::Every { interval, .. } => interval.as_str(),
                other => panic!("not an `every`: {other:?}"),
            })
            .collect();
        assert_eq!(intervals, ["24", "1", "6", "0.5"]);
    }

    /// T-092: `troop <new> like <existing> { <field> <value> … }`, the value as record text.
    /// T-099: the party words (`town`, `castle`, `village`, `party`) make the same item.
    #[test]
    fn troop_reads_the_new_id_the_copied_one_and_its_field_lines() {
        let file = parse(
            "troop elite_crossbowman like swadian_crossbowman {\n  name \"Elite Crossbowman\"\n  level 22\n  agility 18\n}\n",
        )
        .unwrap();
        let Item::Like { kind, id, like, fields, .. } = &file.items[0] else {
            panic!("not a copy: {:?}", file.items)
        };
        assert_eq!(kind.text.as_str(), "troop");
        assert_eq!((id.text.as_str(), like.text.as_str()), ("elite_crossbowman", "swadian_crossbowman"));
        let read: Vec<(&str, &str)> = fields.iter().map(|f| (f.name.text.as_str(), f.value.as_str())).collect();
        assert_eq!(read, [("name", "\"Elite Crossbowman\""), ("level", "22"), ("agility", "18")]);

        // T-099: each party word copies a party; the record text of the field stays as written.
        for word in COPY_KINDS.iter().filter(|kw| **kw != "troop") {
            let text = format!("{word} riverhold like praven {{\n  name \"Riverhold\"\n  initial_coordinates (35, -12)\n}}\n");
            let file = parse(&text).unwrap();
            let Item::Like { kind, id, like, fields, .. } = &file.items[0] else {
                panic!("not a copy: {:?}", file.items)
            };
            assert_eq!((kind.text.as_str(), id.text.as_str(), like.text.as_str()), (*word, "riverhold", "praven"));
            let read: Vec<(&str, &str)> = fields.iter().map(|f| (f.name.text.as_str(), f.value.as_str())).collect();
            assert_eq!(read, [("name", "\"Riverhold\""), ("initial_coordinates", "(35, -12)")]);
        }

        // A field line without a value, and a missing `like`, say what to write.
        let (d, r) = error("troop elite like swadian {\n  level\n}\n");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("has no value"), "{r}");
        let (d, r) = error("troop elite swadian { level 22 }\n");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("expected `like` after `troop elite`"), "{r}");
    }

    #[test]
    fn every_needs_a_number_of_hours() {
        let (d, r) = error("every 6 sheep { a() }");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("`every` counts hours"), "{r}");
        let (d, _) = error("every 0. hours { a() }");
        assert_eq!(d.code, E0100_UNEXPECTED);
        let (d, r) = error("every week { a() }");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("expected `day`, `hour` or `<N> hours` after `every`"), "{r}");
    }

    fn on_body(text: &str) -> Vec<Stmt> {        let file = parse(text).expect("parses");
        let Item::On { body, .. } = &file.items[0] else { panic!() };
        body.stmts.clone()
    }

    #[test]
    fn assignment_operators_parse() {
        let stmts = on_body(
            "on game.start {\n$x = 1\n$x += 2\n$x -= 3\n$x *= 4\n$x /= 5\n$x %= 6\nreg0 = 7\n:loc = $x\n}",
        );
        let ops: Vec<AssignOp> = stmts
            .iter()
            .map(|s| match s {
                Stmt::Assign { op, .. } => *op,
                other => panic!("not an assignment: {other:?}"),
            })
            .collect();
        assert_eq!(
            ops,
            [AssignOp::Set, AssignOp::Add, AssignOp::Sub, AssignOp::Mul, AssignOp::Div, AssignOp::Mod, AssignOp::Set, AssignOp::Set]
        );
        assert!(matches!(&stmts[0], Stmt::Assign { target: Arg::Global(g), value: Expr::Value(Arg::Int(1, _)), .. } if g.text == "$x"));
        assert!(matches!(&stmts[6], Stmt::Assign { target: Arg::Name(n), value: Expr::Value(Arg::Int(7, _)), .. } if n.text == "reg0"));
        assert!(matches!(&stmts[7], Stmt::Assign { target: Arg::Local(l), value: Expr::Value(Arg::Global(g)), .. } if l.text == ":loc" && g.text == "$x"));
    }

    #[test]
    fn expression_precedence() {
        let stmts = on_body("on game.start {\n$x = a + b * c\n$y = (a + b) * c\n}");
        let Stmt::Assign { value, .. } = &stmts[0] else { panic!() };
        let Expr::Bin { op: BinOp::Add, left, right, .. } = value else { panic!("{value:?}") };
        assert!(matches!(&**left, Expr::Value(Arg::Name(n)) if n.text == "a"));
        assert!(matches!(&**right, Expr::Bin { op: BinOp::Mul, .. }));
        let Stmt::Assign { value, .. } = &stmts[1] else { panic!() };
        let Expr::Bin { op: BinOp::Mul, left, right, .. } = value else { panic!("{value:?}") };
        assert!(matches!(&**left, Expr::Bin { op: BinOp::Add, .. }));
        assert!(matches!(&**right, Expr::Value(Arg::Name(n)) if n.text == "c"));
    }

    #[test]
    fn condition_operands_and_negative_literals() {
        let stmts = on_body("on game.start {\nif $gold >= 100 and party_is_active(p_town_6) {\n$x = -5\n}\n}");
        let Stmt::If { arms, .. } = &stmts[0] else { panic!("{:?}", stmts[0]) };
        let Cond::And(parts) = &arms[0].0 else { panic!("{:?}", arms[0].0) };
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], Cond::Cmp { op: CmpOp::Ge, right: Expr::Value(Arg::Int(100, _)), .. }));
        assert!(matches!(&parts[1], Cond::Call(c) if c.op.text == "party_is_active" && c.args.len() == 1));
        let Stmt::Assign { value, .. } = &arms[0].1.stmts[0] else { panic!() };
        assert!(matches!(value, Expr::Value(Arg::Int(-5, _))));
    }

    #[test]
    fn condition_precedence_is_not_then_and_then_or() {
        let stmts = on_body("on game.start {\nif a() or b() and c() {\n$x = 1\n}\n}");
        let Stmt::If { arms, .. } = &stmts[0] else { panic!() };
        let Cond::Or(parts) = &arms[0].0 else { panic!("{:?}", arms[0].0) };
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], Cond::Call(c) if c.op.text == "a"));
        let Cond::And(inner) = &parts[1] else { panic!("{:?}", parts[1]) };
        assert_eq!(inner.len(), 2);
        assert!(matches!(&inner[0], Cond::Call(c) if c.op.text == "b"));
        assert!(matches!(&inner[1], Cond::Call(c) if c.op.text == "c"));
    }

    #[test]
    fn parentheses_group_a_condition() {
        let stmts = parse_block("if $a == 1 and ($b == 2 or not party_is_active(p_x)) {\n}").unwrap();
        let Stmt::If { arms, .. } = &stmts[0] else { panic!("{stmts:?}") };
        let Cond::And(parts) = &arms[0].0 else { panic!("{:?}", arms[0].0) };
        assert_eq!(parts.len(), 2);
        let Cond::Or(branches) = &parts[1] else { panic!("{:?}", parts[1]) };
        assert!(matches!(branches[1], Cond::Not(..)));
    }

    #[test]
    fn parentheses_around_arithmetic_still_compare() {
        let stmts = parse_block("if ($a + 1) * 2 > 3 and ($b) == 1 {\n}").unwrap();
        let Stmt::If { arms, .. } = &stmts[0] else { panic!("{stmts:?}") };
        let Cond::And(parts) = &arms[0].0 else { panic!("{:?}", arms[0].0) };
        assert!(parts.iter().all(|p| matches!(p, Cond::Cmp { .. })), "{parts:?}");
    }

    #[test]
    fn not_wraps_a_comparison() {
        let stmts = on_body("on game.start {\nif not $gold < 10 {\n$x = 1\n}\n}");
        let Stmt::If { arms, .. } = &stmts[0] else { panic!() };
        let Cond::Not(inner, _) = &arms[0].0 else { panic!("{:?}", arms[0].0) };
        assert!(matches!(&**inner, Cond::Cmp { op: CmpOp::Lt, .. }));
    }

    #[test]
    fn else_if_chain_parses() {
        let stmts = on_body(
            "on game.start {\nif $g < 1 {\n$x = 1\n} else if $g < 2 {\n$x = 2\n} else {\n$x = 3\n}\n}",
        );
        let Stmt::If { arms, otherwise, .. } = &stmts[0] else { panic!("{:?}", stmts[0]) };
        assert_eq!(arms.len(), 2);
        assert!(matches!(&arms[0].0, Cond::Cmp { op: CmpOp::Lt, .. }));
        assert!(matches!(&arms[1].0, Cond::Cmp { op: CmpOp::Lt, .. }));
        let Some(otherwise) = otherwise else { panic!("no else block") };
        assert_eq!(otherwise.stmts.len(), 1);
    }

    #[test]
    fn condition_continues_on_the_next_line_after_and() {
        let stmts = on_body(
            "on game.start {\nif party_is_active(p_town_6) and\n   $gold > 10 {\n$x = 1\n}\n}",
        );
        let Stmt::If { arms, .. } = &stmts[0] else { panic!("{:?}", stmts[0]) };
        let Cond::And(parts) = &arms[0].0 else { panic!("{:?}", arms[0].0) };
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], Cond::Call(c) if c.op.text == "party_is_active"));
        assert!(matches!(&parts[1], Cond::Cmp { op: CmpOp::Gt, .. }));
    }

    #[test]
    fn a_line_continues_inside_parentheses() {
        let stmts = on_body("on game.start {\n$x = (a\n + b) * c\n}");
        let Stmt::Assign { value, .. } = &stmts[0] else { panic!("{:?}", stmts[0]) };
        let Expr::Bin { op: BinOp::Mul, left, .. } = value else { panic!("{value:?}") };
        assert!(matches!(&**left, Expr::Bin { op: BinOp::Add, .. }));
    }

    #[test]
    fn elif_and_python_else_get_a_one_line_fix() {
        let (d, r) = error("on game.start {\n  if $x < 1 {\n    a()\n  } elif $x < 2 {\n    a()\n  }\n}");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("else if"), "{r}");
        assert!(!r.contains("unexpected end"), "{r}");
        let (d, r) = error("on game.start {\n  if $x < 1 {\n  } else:\n  }\n}");
        assert_eq!(d.code, E0104_PYTHON_BLOCK);
        assert!(r.contains("else {"), "{r}");
    }

    fn menu_members(text: &str) -> Vec<Member> {
        let file = parse(text).expect("parses");
        let Item::Override { members, .. } = &file.items[0] else { panic!("not an override") };
        members.clone()
    }

    #[test]
    fn override_with_a_full_id_has_no_kind() {
        let file = parse("override menu_start_phase_2 {\n  text \"New text\"\n}").unwrap();
        let Item::Override { kind, name, members, .. } = &file.items[0] else { panic!() };
        assert!(kind.is_none());
        assert_eq!(name.text, "menu_start_phase_2");
        assert!(matches!(&members[0], Member::Text { text, .. } if text == "New text"));
    }

    #[test]
    fn new_record_with_or_without_text() {
        let text = "new menu_uw_ask \"Where from?\" {\n  on enter {\n    a()\n  }\n  option north \"North\" {\n    jump_to_menu(mnu_start_phase_2)\n  }\n}\n\nnew menu_uw_other { }\n";
        let file = parse(text).expect("parses");
        assert_eq!(file.items.len(), 2);
        let Item::New { id, text, handlers, members, .. } = &file.items[0] else { panic!() };
        assert_eq!(id.text, "menu_uw_ask");
        assert_eq!(text.as_ref().map(|(t, _)| t.as_str()), Some("Where from?"));
        assert_eq!(handlers[0].event.text, "enter");
        assert!(matches!(&members[0], Member::Option { id, .. } if id.text == "north"));
        let Item::New { text, handlers, members, .. } = &file.items[1] else { panic!() };
        assert!(text.is_none() && handlers.is_empty() && members.is_empty());
    }

    #[test]
    fn option_when_and_place_forms() {
        let members = menu_members(
            "override menu_x {\n  option a \"A\" {\n    a_native()\n  }\n  option b \"B\" when $gold < 10 after a {\n    b_native()\n  }\n  option c \"C\" when { ge(:g, 5) } before a {\n    c_native()\n  }\n}",
        );
        assert_eq!(members.len(), 3);
        let Member::Option { when, place, body, .. } = &members[0] else { panic!("{:?}", members[0]) };
        assert!(when.is_none() && place.is_none());
        assert_eq!(body.stmts.len(), 1);
        let Member::Option { when, place, .. } = &members[1] else { panic!("{:?}", members[1]) };
        assert!(matches!(when, Some(When::Cond(Cond::Cmp { op: CmpOp::Lt, .. }))));
        assert!(matches!(place, Some(Place::After(id)) if id.text == "a"));
        let Member::Option { when, place, .. } = &members[2] else { panic!("{:?}", members[2]) };
        assert!(matches!(when, Some(When::Block(block)) if block.stmts.len() == 1));
        assert!(matches!(place, Some(Place::Before(id)) if id.text == "a"));
    }

    #[test]
    fn change_option_takes_each_part_in_any_order() {
        let members = menu_members(
            "override menu_x {\n  change option camp_wait_here {\n    do {\n      $x = 1\n    }\n    text \"Wait\"\n    when $x > 0\n  }\n  change option only_text { text \"Only\" }\n}",
        );
        let Member::ChangeOption { id, text, when, body, .. } = &members[0] else { panic!("{:?}", members[0]) };
        assert_eq!(id.text, "camp_wait_here");
        assert_eq!(text.as_ref().map(|(t, _)| t.as_str()), Some("Wait"));
        assert!(matches!(when, Some(When::Cond(Cond::Cmp { op: CmpOp::Gt, .. }))));
        assert_eq!(body.as_ref().map(|b| b.stmts.len()), Some(1));
        let Member::ChangeOption { text, when, body, .. } = &members[1] else { panic!("{:?}", members[1]) };
        assert!(text.is_some() && when.is_none() && body.is_none());
    }

    #[test]
    fn remove_option_parses() {
        let members = menu_members("override menu_x {\n  remove option camp_leave\n}");
        assert!(matches!(&members[0], Member::RemoveOption { id, .. } if id.text == "camp_leave"));
    }

    #[test]
    fn option_without_text_is_named() {
        let (d, r) = error("override menu_x {\n  option uw_skip {\n    a()\n  }\n}");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("a text in double quotes after `option uw_skip`"), "{r}");
        assert!(!r.contains("unexpected end"), "{r}");
    }

    /// T-091: `option "Text" { … }` without an id; the id comes from the text and stays unique in
    /// the menu (`_2`, `_3` …), next to the ids written out.
    #[test]
    fn an_option_without_an_id_is_named_after_its_text() {
        let members = menu_members(
            "override menu_x {\n  option uw_go_north \"North\" {\n    a()\n  }\n  option \"Go North!\" {\n    b()\n  }\n  option \"go north\" when $x > 0 after uw_go_north {\n    c()\n  }\n  option \"…\" {\n    d()\n  }\n}",
        );
        let ids: Vec<&str> = members
            .iter()
            .map(|m| match m {
                Member::Option { id, .. } => id.text.as_str(),
                other => panic!("{other:?}"),
            })
            .collect();
        assert_eq!(ids, ["uw_go_north", "uw_go_north_2", "uw_go_north_3", "uw_option"]);
        let Member::Option { when, place, .. } = &members[2] else { panic!() };
        assert!(matches!(when, Some(When::Cond(Cond::Cmp { .. }))));
        assert!(matches!(place, Some(Place::After(id)) if id.text == "uw_go_north"));
    }

    #[test]
    fn in_menu_is_the_same_item_as_override_menu() {
        let file = parse("in menu camp {\n  option \"Party morale report\" {\n    say(\"@Morale is fine.\")\n  }\n}")
            .expect("parses");
        let Item::Override { kind, name, members, .. } = &file.items[0] else { panic!("{:?}", file.items[0]) };
        assert!(matches!(kind, Some(kind) if kind.text == "menu"));
        assert_eq!(name.text, "camp");
        assert!(matches!(&members[0], Member::Option { id, text, .. }
            if id.text == "uw_party_morale_report" && text == "Party morale report"));
    }

    #[test]
    fn a_change_option_takes_each_part_once() {
        let (d, r) = error("override menu_x {\n  change option a {\n    text \"One\"\n    text \"Two\"\n  }\n}");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("already sets `text` on line 3"), "{r}");
        assert!(r.contains("--> t.uw:4:5"), "{r}");
    }

    #[test]
    fn when_with_nothing_after_it_is_named() {
        let (d, r) = error("override menu_x {\n  option a \"A\" when\n}");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("expected a condition or '{' after `when`"), "{r}");
        assert!(!r.contains("unexpected end"), "{r}");
        let (d, r) = error("override menu_x {\n  change option a {\n    when\n  }\n}");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("after `when`"), "{r}");
    }

    #[test]
    fn locals_and_globals_may_start_with_a_digit() {
        let stmts = parse_block("store_add(:400_plus_random_200, 400, :r)
$2ary = 1").expect("parses");
        assert!(matches!(&stmts[0].as_call().expect("a call").args[0], Arg::Local(l) if l.text == ":400_plus_random_200"));
        assert!(matches!(&stmts[1], Stmt::Assign { target: Arg::Global(g), .. } if g.text == "$2ary"));
    }

    #[test]
    fn parse_block_reads_statements() {
        let stmts = parse_block("party_is_active(p)\nif $x < 1 {\n  change_screen_map()\n}\n$x = 2").expect("parses");
        assert_eq!(stmts.len(), 3);
        assert!(matches!(&stmts[0], Stmt::Call(c) if c.op.text == "party_is_active"));
        assert!(matches!(&stmts[1], Stmt::If { arms, .. } if arms[0].1.stmts.len() == 1));
        assert!(matches!(&stmts[2], Stmt::Assign { target: Arg::Global(g), .. } if g.text == "$x"));
        assert!(parse_block("").expect("parses").is_empty());
    }

    #[test]
    fn flagged_call_names() {
        let stmts = parse_block("neg|eq(a, b)\nthis_or_next|party_is_active(p)\nneg|this_or_next|x()").expect("parses");
        let first = stmts[0].as_call().expect("a call");
        assert_eq!(first.op.text, "neg|eq");
        assert_eq!(first.op.span, Span { start: 0, end: 6 });
        assert!(matches!(&first.args[0], Arg::Name(a) if a.text == "a"));
        // the span covers all of `this_or_next|party_is_active`
        let second = stmts[1].as_call().expect("a call");
        assert_eq!(second.op.text, "this_or_next|party_is_active");
        assert_eq!(second.op.span.end - second.op.span.start, "this_or_next|party_is_active".len());
        assert!(matches!(&second.args[0], Arg::Name(p) if p.text == "p"));
        assert_eq!(stmts[2].as_call().expect("a call").op.text, "neg|this_or_next|x");

        let stmts = parse_block("if neg|party_is_active(p) {\n  a()\n}").expect("parses");
        let Stmt::If { arms, .. } = &stmts[0] else { panic!("{:?}", stmts[0]) };
        assert!(matches!(&arms[0].0, Cond::Call(c) if c.op.text == "neg|party_is_active" && c.args.len() == 1));
    }

    #[test]
    fn flag_prefix_unknown_is_error() {
        let d = parse_block("foo|eq(1, 2)").expect_err("should fail");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(d.message.contains("`foo`"), "{}", d.message);
        assert!(d.help.unwrap_or_default().contains("this_or_next"));
        let d = parse_block("if not neg|foo|eq(1, 2) {\n  a()\n}").expect_err("should fail");
        assert_eq!(d.code, E0100_UNEXPECTED);
    }

    #[test]
    fn curly_quote_escape() {
        let stmts = parse_block("display_message(\"\\\u{201c}hi\\\u{201d}\")").expect("parses");
        assert!(matches!(&stmts[0].as_call().unwrap().args[0], Arg::Str(s, _) if s == "\u{201c}hi\u{201d}"));
    }

    #[test]
    fn negative_and_big_ints_in_calls() {
        let stmts = parse_block("f(-1)\nf(72057594037927936)\ntry_begin()\n$x = -5").expect("parses");
        assert!(matches!(&stmts[0].as_call().unwrap().args[0], Arg::Int(-1, _)));
        assert!(matches!(&stmts[1].as_call().unwrap().args[0], Arg::Int(72057594037927936, _)));
        assert!(stmts[2].as_call().unwrap().args.is_empty());
        assert!(matches!(&stmts[3], Stmt::Assign { value: Expr::Value(Arg::Int(-5, _)), .. }));
    }

    #[test]
    fn require_parses_with_its_condition() {
        let text = "require $gold >= 100";
        let stmts = parse_block(text).expect("parses");
        let Stmt::Require { cond, span } = &stmts[0] else { panic!("{:?}", stmts[0]) };
        assert!(
            matches!(cond, Cond::Cmp { op: CmpOp::Ge, left: Expr::Value(Arg::Global(g)), right: Expr::Value(Arg::Int(100, _)), .. } if g.text == "$gold")
        );
        assert_eq!(*span, Span { start: 0, end: text.len() });

        let stmts = parse_block("require party_is_active(p_town_6); a()").expect("parses");
        assert!(matches!(&stmts[0], Stmt::Require { cond: Cond::Call(c), .. } if c.op.text == "party_is_active"));
        assert!(matches!(&stmts[1], Stmt::Call(c) if c.op.text == "a"));
    }

    #[test]
    fn attempt_chain_and_final_else_parse() {
        let text = "attempt {\n  require $g > 1\n} else attempt {\n  require $g > 2\n} else {\n$x = 3\n}";
        let stmts = parse_block(text).expect("parses");
        let Stmt::Attempt { arms, span } = &stmts[0] else { panic!("{:?}", stmts[0]) };
        assert_eq!(arms.len(), 3);
        assert!(matches!(&arms[0].stmts[0], Stmt::Require { cond: Cond::Cmp { .. }, .. }));
        assert!(matches!(&arms[1].stmts[0], Stmt::Require { cond: Cond::Cmp { .. }, .. }));
        assert!(matches!(&arms[2].stmts[0], Stmt::Assign { target: Arg::Global(g), .. } if g.text == "$x"));
        assert_eq!(*span, Span { start: 0, end: text.len() });

        // Without a final `else` the chain stops after the last `else attempt`.
        let stmts = parse_block("attempt {\n  a()\n} else attempt {\n  b()\n}").expect("parses");
        let Stmt::Attempt { arms, .. } = &stmts[0] else { panic!("{:?}", stmts[0]) };
        assert_eq!(arms.len(), 2);
        assert!(matches!(&arms[1].stmts[0], Stmt::Call(c) if c.op.text == "b"));
    }

    #[test]
    fn for_parses_its_variable_kind_and_arguments() {
        let text = "for :i in range(0, 10) {\n  f(:i)\n}";
        let stmts = parse_block(text).expect("parses");
        let Stmt::For { var, kind, args, body, span } = &stmts[0] else { panic!("{:?}", stmts[0]) };
        assert!(matches!(var, Arg::Local(l) if l.text == ":i"));
        assert_eq!(kind.text, "range");
        assert_eq!(args.len(), 2);
        assert!(matches!(&args[0], Arg::Int(0, _)));
        assert!(matches!(&args[1], Arg::Int(10, _)));
        assert!(matches!(&body.stmts[0], Stmt::Call(c) if c.op.text == "f"));
        assert_eq!(*span, Span { start: 0, end: text.len() });

        let stmts = parse_block("for $p in parties() {\n}").expect("parses");
        let Stmt::For { var, kind, args, body, .. } = &stmts[0] else { panic!("{:?}", stmts[0]) };
        assert!(matches!(var, Arg::Global(g) if g.text == "$p"));
        assert_eq!(kind.text, "parties");
        assert!(args.is_empty());
        assert!(body.stmts.is_empty());
    }

    #[test]
    fn attempt_nests_inside_for_and_require_inside_an_arm() {
        let stmts = parse_block(
            "for :p in parties() {\n  attempt {\n    require $ok == 1\n    f(:p)\n  } else {\n    g()\n  }\n}",
        )
        .expect("parses");
        let Stmt::For { body, .. } = &stmts[0] else { panic!("{:?}", stmts[0]) };
        let Stmt::Attempt { arms, .. } = &body.stmts[0] else { panic!("{:?}", body.stmts[0]) };
        assert_eq!(arms.len(), 2);
        assert!(matches!(&arms[0].stmts[0], Stmt::Require { cond: Cond::Cmp { .. }, .. }));
        assert!(matches!(&arms[0].stmts[1], Stmt::Call(c) if c.op.text == "f"));
        assert!(matches!(&arms[1].stmts[0], Stmt::Call(c) if c.op.text == "g"));
    }

    #[test]
    fn for_without_in_is_named() {
        let (d, r) = error("on game.start {\n  for :i range(0, 10) {\n  }\n}");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("expected `in` after the loop variable, found 'range'"), "{r}");
        assert!(r.contains("--> t.uw:2:10"), "{r}");
        assert!(!r.contains("unexpected end"), "{r}");
    }

    #[test]
    fn for_without_a_block_brace_is_named() {
        let (d, r) = error("on game.start {\n  for :i in range(0, 10)\n    f(:i)\n  }\n}");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("expected '{' after `for :i in range`"), "{r}");
        assert!(r.contains("--> t.uw:3:5"), "{r}");
        // An unclosed body points at its own `{`, never only at the file end.
        let (d, r) = error("on game.start {\n  for :i in range(0, 10) {\n");
        assert_eq!(d.code, E0101_UNCLOSED);
        assert!(r.contains("--> t.uw:2:"), "{r}");
        assert!(r.contains("never closed"), "{r}");
    }

    #[test]
    fn else_if_after_attempt_is_an_error_with_a_fix() {
        let (d, r) = error("on game.start {\n  attempt {\n    a()\n  } else if $x < 1 {\n    b()\n  }\n}");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("`else if` cannot follow an `attempt` block"), "{r}");
        assert!(r.contains("else attempt"), "{r}");
        assert!(r.contains("--> t.uw:4:10"), "{r}");
    }

    #[test]
    fn python_style_for_gets_a_one_line_fix() {
        let (d, r) = error("on game.start {\n  for i in range(10):\n    a()\n  }\n}");
        assert_eq!(d.code, E0104_PYTHON_BLOCK);
        assert!(r.contains("`for i …:` is not a UW loop"), "{r}");
        assert!(r.contains("write `for :i in range(0, 10) { ... }`"), "{r}");
        assert!(r.contains("--> t.uw:2:7"), "{r}");
        assert!(!r.contains("unexpected end"), "{r}");
    }

    /// T-109: `area` corners may be negative or decimal, and `on enter` takes an optional `when`.
    #[test]
    fn area_and_on_enter_parse() {
        let file = parse(
            "area mountain_road [(10, 20), (-96, 26.4), (40, 60)]\n\non enter mountain_road when time.hour >= 12 {\n  say(\"@Hi\")\n}\n",
        )
        .expect("parses");
        let Item::Area { name, points, .. } = &file.items[0] else { panic!("{:?}", file.items[0]) };
        assert_eq!(name.text, "mountain_road");
        assert_eq!(points, &[AreaPoint { x: 1000, y: 2000 }, AreaPoint { x: -9600, y: 2640 }, AreaPoint { x: 4000, y: 6000 }]);
        let Item::AreaEnter { area, when, body, .. } = &file.items[1] else { panic!("{:?}", file.items[1]) };
        assert_eq!(area.text, "mountain_road");
        assert!(matches!(when, Some(When::Cond(Cond::Cmp { op: CmpOp::Ge, .. }))));
        assert_eq!(body.stmts.len(), 1);
        // Two corners parse too; the check is the one that asks for three.
        let file = parse("area line [(0, 0), (1, 1)]\n").expect("parses");
        let Item::Area { points, .. } = &file.items[0] else { panic!() };
        assert_eq!(points.len(), 2);
        // The corner list must open with `[`.
        let (d, r) = error("area line (0, 0)\n");
        assert_eq!(d.code, E0100_UNEXPECTED);
        assert!(r.contains("expected '[' and the corner list after `area line`"), "{r}");
    }
}
