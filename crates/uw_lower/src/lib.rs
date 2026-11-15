//! Lowering: the checked statement tree (assignments, expressions, `if`, `and/or/not`) becomes the
//! flat Native operation list the kit writes. Names are already resolved by `uw_check`; nothing
//! here looks at the atlas. Rules: docs/meclis/gramer-2-oneri.md §1, §2 and §7 (bağlayıcı).
//!
//! Native operations used (header_operations.py line): try_end 16, try_begin 17, else_try 19,
//! ge 40, eq 41, gt 42, neg 440, this_or_next 441, lt/neq/le 444-446 (`neg|ge`, `neg|eq`, `neg|gt`),
//! val_add..val_mod 1139-1147, store_mod/add/sub/mul/div 1166-1174, assign 1191.

/// A value inside an operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// A Native reference, emitted as a Python string the kit resolves (`"p_town_6"`), also
    /// `$global` and `:local`.
    Ref(String),
    /// A kit constant emitted as a bare Python name, with the module that defines it.
    Const { name: String, module: String },
    Int(i64),
    /// Text, emitted as a Python string literal.
    Text(String),
}

/// One Native operation. `name` may carry flag prefixes the kit understands: `neg|eq`,
/// `this_or_next|gt` (emitted as written: `(neg|eq, a, b)`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub name: String,
    pub args: Vec<Value>,
    /// `file.uw:line`, written next to the generated operation.
    pub origin: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Set,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

/// A checked integer expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Value(Value),
    Neg(Box<Expr>),
    Bin { op: BinOp, left: Box<Expr>, right: Box<Expr> },
}

/// A checked condition. `Op` is a Native condition operation (`party_is_active`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cond {
    Cmp { op: CmpOp, left: Expr, right: Expr, origin: String },
    Op(Op),
    Not(Box<Cond>),
    And(Vec<Cond>),
    Or(Vec<Cond>),
}

/// A checked statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Op(Op),
    /// `target` is a `Value::Ref` (`$g`, `:l`) or a register `Value::Const`.
    Assign { target: Value, op: AssignOp, value: Expr, origin: String },
    If { arms: Vec<(Cond, Vec<Stmt>)>, otherwise: Option<Vec<Stmt>>, origin: String },
}

/// Prefix of the temporaries lowering creates (`:_uw0`, `:_uw1`, …). The checker rejects user
/// locals that start with `:_` so they never collide (§1, §7.6).
pub const TEMP_PREFIX: &str = ":_uw";

/// Lowers one operation block (a menu's enter code, an option's condition or consequence, a hook
/// body). Temporaries are numbered from 0 per block.
pub fn lower(body: &[Stmt]) -> Vec<Op> {
    let mut out = Vec::new();
    for stmt in body {
        match stmt {
            Stmt::Op(op) => out.push(op.clone()),
            // Lowering of assignments, expressions and `if` is task T-016.
            Stmt::Assign { .. } | Stmt::If { .. } => unimplemented!("T-016: lowering of assignments and if"),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_operations_pass_through() {
        let op = Op { name: "change_screen_map".into(), args: vec![], origin: "a.uw:2".into() };
        assert_eq!(lower(&[Stmt::Op(op.clone())]), vec![op]);
    }
}
