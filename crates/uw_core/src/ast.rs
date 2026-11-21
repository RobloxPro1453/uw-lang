//! Syntax tree of the UW core language v0 (docs/dil/CORE-v0.md §3). This file is the contract between
//! the parser (`parse.rs`) and the lowering (`lower.rs`): change it only together with the spec.

/// Byte span in the source file plus the 1-based line, for messages (`file:line`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    /// `main` for main.uw, else the feature name (file stem).
    pub feature: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Const { name: Ident, value: Expr, span: Span },
    Var(VarDecl),
    Fn { name: Ident, params: Vec<Ident>, body: Block, span: Span },
    /// `on <event> (<bound>)? (when <cond>)? { … }`
    On { event: Path, bound: Option<Ident>, when: Option<Expr>, body: Block, span: Span },
    /// `while in <context> { … }`
    WhileIn { context: Path, body: Block, span: Span },
    /// `while <cond> { … } otherwise { … }`
    While { cond: Expr, body: Block, otherwise: Option<Block>, span: Span },
    /// `ui.<part> <name> { prop = expr … }`
    Ui { part: Ident, name: Ident, props: Vec<(Ident, Expr)>, span: Span },
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    /// `None` for a feature variable, `Some(kind)` for `var agent.stamina`.
    pub kind: Option<Ident>,
    pub name: Ident,
    pub init: Option<Expr>,
    /// `in lo..hi`
    pub range: Option<(Expr, Expr)>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub text: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub parts: Vec<Ident>,
    pub span: Span,
}

pub type Block = Vec<Stmt>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Set,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let { name: Ident, value: Expr, span: Span },
    Assign { place: Path, op: AssignOp, value: Expr, span: Span },
    Call(Call),
    If { cond: Expr, then: Block, otherwise: Option<Block>, span: Span },
    For { var: Ident, iter: Expr, filter: Option<Expr>, body: Block, span: Span },
    Return { value: Option<Expr>, span: Span },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: Path,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateUnit {
    Second,
    Minute,
    Hour,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Or,
    And,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Integer or decimal, kept as thousandths (`0.5` → 500, `12` → 12000): UW numbers are fixed-point.
    Number { milli: i64, span: Span },
    /// `150%` → 150.
    Percent { value: i64, span: Span },
    /// `15 per second`: amount in thousandths per unit.
    Rate { milli: i64, unit: RateUnit, span: Span },
    /// `#3FA34D` → 0x3FA34D.
    Color { rgb: u32, span: Span },
    /// String with `{expr}` parts kept as source text pieces: literal text and expressions alternate.
    Text { parts: Vec<TextPart>, span: Span },
    Bool { value: bool, span: Span },
    Tuple { items: Vec<Expr>, span: Span },
    Path(Path),
    Call(Call),
    Unary { neg: bool, not: bool, expr: Box<Expr>, span: Span },
    Binary { op: BinOp, left: Box<Expr>, right: Box<Expr>, span: Span },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextPart {
    Literal(String),
    Expr(Expr),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Number { span, .. }
            | Expr::Percent { span, .. }
            | Expr::Rate { span, .. }
            | Expr::Color { span, .. }
            | Expr::Text { span, .. }
            | Expr::Bool { span, .. }
            | Expr::Tuple { span, .. }
            | Expr::Unary { span, .. }
            | Expr::Binary { span, .. } => *span,
            Expr::Path(path) => path.span,
            Expr::Call(call) => call.span,
        }
    }
}
