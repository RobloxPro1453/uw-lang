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
    let mut lower = Lower { out: Vec::new(), next_temp: 0 };
    lower.stmts(body);
    lower.out
}

/// One lowering pass: the operations written so far and the next temporary number.
struct Lower {
    out: Vec<Op>,
    next_temp: usize,
}

impl Lower {
    fn op(&mut self, name: &str, args: Vec<Value>, origin: &str) {
        self.out.push(Op { name: name.into(), args, origin: origin.into() });
    }

    fn temp(&mut self) -> Value {
        let value = Value::Ref(format!("{TEMP_PREFIX}{}", self.next_temp));
        self.next_temp += 1;
        value
    }

    fn stmts(&mut self, body: &[Stmt]) {
        for stmt in body {
            match stmt {
                Stmt::Op(op) => self.out.push(op.clone()),
                Stmt::Assign { target, op, value, origin } => match op {
                    AssignOp::Set => self.expr_into(target.clone(), value, origin),
                    AssignOp::Add => self.val("val_add", target, value, origin),
                    AssignOp::Sub => self.val("val_sub", target, value, origin),
                    AssignOp::Mul => self.val("val_mul", target, value, origin),
                    AssignOp::Div => self.val("val_div", target, value, origin),
                    AssignOp::Mod => self.val("val_mod", target, value, origin),
                },
                Stmt::If { arms, otherwise, origin } => self.if_stmt(arms, otherwise, origin),
            }
        }
    }

    /// `x op= e`: the kit's `val_*` takes one value, so an expression becomes a temporary first.
    fn val(&mut self, name: &str, target: &Value, value: &Expr, origin: &str) {
        let value = self.value(value, origin);
        self.op(name, vec![target.clone(), value], origin);
    }

    /// `e` as one value: a leaf as it is, anything else evaluated into a fresh temporary (§7.6).
    fn value(&mut self, e: &Expr, origin: &str) -> Value {
        if let Some(leaf) = leaf(e) {
            return leaf;
        }
        let temp = self.temp();
        self.expr_into(temp.clone(), e, origin);
        temp
    }

    /// Evaluates `e` into `dest` (§1): a leaf is `assign`, a binary operation a `store_*`, and `-`
    /// of a non-literal a `store_sub`.
    fn expr_into(&mut self, dest: Value, e: &Expr, origin: &str) {
        match e {
            Expr::Value(value) => self.op("assign", vec![dest, value.clone()], origin),
            Expr::Neg(inner) => match int_literal(inner) {
                Some(n) => self.op("assign", vec![dest, Value::Int(-n)], origin),
                None => {
                    let value = self.value(inner, origin);
                    self.op("store_sub", vec![dest, Value::Int(0), value], origin);
                }
            },
            Expr::Bin { op, left, right } => {
                let left = self.value(left, origin);
                let right = self.value(right, origin);
                let name = format!("store_{}", bin_name(*op));
                self.op(&name, vec![dest, left, right], origin);
            }
        }
    }

    fn if_stmt(&mut self, arms: &[(Cond, Vec<Stmt>)], otherwise: &Option<Vec<Stmt>>, origin: &str) {
        self.op("try_begin", Vec::new(), origin);
        for (i, (cond, body)) in arms.iter().enumerate() {
            if i > 0 {
                self.op("else_try", Vec::new(), origin);
            }
            self.cond(cond, origin);
            self.stmts(body);
        }
        if let Some(body) = otherwise {
            self.op("else_try", Vec::new(), origin);
            self.stmts(body);
        }
        self.op("try_end", Vec::new(), origin);
    }

    /// Emits the operations of one condition: they fall through when it holds (§2).
    fn cond(&mut self, c: &Cond, origin: &str) {
        match c {
            // One Native operation, or the same under `not`.
            Cond::Cmp { .. } | Cond::Op(_) => {
                self.single_op(c);
            }
            Cond::Not(inner) => {
                // `not a < b` flips the operator, `not party_is_active(x)` adds `neg|` (§7.7).
                if !self.single_op(c) {
                    let flag = self.flag_of(inner, origin);
                    self.op("eq", vec![flag, Value::Int(0)], origin);
                }
            }
            // `and` is a plain sequence of conditions.
            Cond::And(parts) => {
                for part in parts {
                    match part {
                        // A pure `or` group of single operations inside an `and` chain (§7.3).
                        Cond::Or(branches) if is_pure_or(branches) => self.or_group(branches),
                        _ => self.cond(part, origin),
                    }
                }
            }
            // A pure `or` on its own is an `and` chain of one element: `this_or_next|` (§7.3), the
            // way Native writes it.
            Cond::Or(branches) if is_pure_or(branches) => self.or_group(branches),
            // Everything else cannot be reduced to a plain sequence: a flag temporary (§2, §7.3).
            Cond::Or(_) => {
                let flag = self.flag_of(c, origin);
                self.op("eq", vec![flag, Value::Int(1)], origin);
            }
        }
    }

    /// A pure `or` group: every branch but the last carries `this_or_next|`.
    fn or_group(&mut self, branches: &[Cond]) {
        let last = branches.len() - 1;
        for (i, branch) in branches.iter().enumerate() {
            if self.single_op(branch) && i < last {
                let at = self.out.len() - 1;
                let name = format!("this_or_next|{}", self.out[at].name);
                self.out[at].name = name;
            }
        }
    }

    /// Emits the one operation a single condition needs; false when it is not a single condition.
    fn single_op(&mut self, c: &Cond) -> bool {
        match c {
            Cond::Cmp { op, left, right, origin } => {
                self.cmp(*op, left, right, origin);
                true
            }
            Cond::Op(op) => {
                self.out.push(op.clone());
                true
            }
            Cond::Not(inner) => match &**inner {
                Cond::Cmp { op, left, right, origin } => {
                    self.cmp(flip(*op), left, right, origin);
                    true
                }
                Cond::Op(op) => {
                    let name = format!("neg|{}", op.name);
                    self.out.push(Op { name, args: op.args.clone(), origin: op.origin.clone() });
                    true
                }
                Cond::Not(_) | Cond::And(_) | Cond::Or(_) => false,
            },
            Cond::And(_) | Cond::Or(_) => false,
        }
    }

    fn cmp(&mut self, op: CmpOp, left: &Expr, right: &Expr, origin: &str) {
        let left = self.value(left, origin);
        let right = self.value(right, origin);
        self.op(cmp_name(op), vec![left, right], origin);
    }

    /// A temporary holding 1 when `c` holds: each `or` branch runs under `try_begin` (§2, §7.6).
    fn flag_of(&mut self, c: &Cond, origin: &str) -> Value {
        let flag = self.temp();
        self.op("assign", vec![flag.clone(), Value::Int(0)], origin);
        for branch in or_branches(c) {
            self.op("try_begin", Vec::new(), origin);
            self.cond(branch, origin);
            self.op("assign", vec![flag.clone(), Value::Int(1)], origin);
            self.op("try_end", Vec::new(), origin);
        }
        flag
    }
}

/// A leaf needs no temporary: a plain value, or a negated integer literal.
fn leaf(e: &Expr) -> Option<Value> {
    match e {
        Expr::Value(value) => Some(value.clone()),
        Expr::Neg(inner) => int_literal(inner).map(|n| Value::Int(-n)),
        Expr::Bin { .. } => None,
    }
}

fn int_literal(e: &Expr) -> Option<i64> {
    match e {
        Expr::Value(Value::Int(n)) => Some(*n),
        Expr::Neg(inner) => int_literal(inner).map(|n| -n),
        Expr::Value(_) | Expr::Bin { .. } => None,
    }
}

/// True when the condition is exactly one Native operation.
fn is_single(c: &Cond) -> bool {
    match c {
        // A comparison counts only when both sides are leaves: temporaries computed between two
        // `this_or_next|` conditions would be ORed instead of the next condition (store_* always
        // succeeds, so the whole group would always hold).
        Cond::Cmp { left, right, .. } => leaf(left).is_some() && leaf(right).is_some(),
        Cond::Op(_) => true,
        Cond::Not(inner) => !matches!(&**inner, Cond::Not(_)) && is_single(inner),
        Cond::And(_) | Cond::Or(_) => false,
    }
}

/// An `or` whose branches are each one Native operation (so `this_or_next|` can join them).
fn is_pure_or(branches: &[Cond]) -> bool {
    !branches.is_empty() && branches.iter().all(is_single)
}

/// The alternatives of an `or`; any other condition is a single branch.
fn or_branches(c: &Cond) -> &[Cond] {
    match c {
        Cond::Or(branches) => branches,
        Cond::Cmp { .. } | Cond::Op(_) | Cond::Not(_) | Cond::And(_) => std::slice::from_ref(c),
    }
}

fn bin_name(op: BinOp) -> &'static str {
    match op {
        BinOp::Add => "add",
        BinOp::Sub => "sub",
        BinOp::Mul => "mul",
        BinOp::Div => "div",
        BinOp::Mod => "mod",
    }
}

fn cmp_name(op: CmpOp) -> &'static str {
    match op {
        CmpOp::Eq => "eq",
        CmpOp::Ne => "neq",
        CmpOp::Gt => "gt",
        CmpOp::Ge => "ge",
        CmpOp::Lt => "lt",
        CmpOp::Le => "le",
    }
}

/// The operator of the negated comparison (§7.7: never `neg|lt`, `neg|neq`, `neg|le`).
fn flip(op: CmpOp) -> CmpOp {
    match op {
        CmpOp::Eq => CmpOp::Ne,
        CmpOp::Ne => CmpOp::Eq,
        CmpOp::Gt => CmpOp::Le,
        CmpOp::Le => CmpOp::Gt,
        CmpOp::Ge => CmpOp::Lt,
        CmpOp::Lt => CmpOp::Ge,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORIGIN: &str = "t.uw:1";

    fn native(name: &str, args: Vec<Value>) -> Op {
        Op { name: name.into(), args, origin: ORIGIN.into() }
    }

    fn value(name: &str) -> Expr {
        Expr::Value(Value::Ref(name.into()))
    }

    fn global(name: &str) -> Value {
        Value::Ref(name.into())
    }

    fn int(n: i64) -> Expr {
        Expr::Value(Value::Int(n))
    }

    fn binary(op: BinOp, left: Expr, right: Expr) -> Expr {
        Expr::Bin { op, left: Box::new(left), right: Box::new(right) }
    }

    fn compare(op: CmpOp, left: Expr, right: Expr) -> Cond {
        Cond::Cmp { op, left, right, origin: ORIGIN.into() }
    }

    fn assign(target: Value, op: AssignOp, value: Expr) -> Stmt {
        Stmt::Assign { target, op, value, origin: ORIGIN.into() }
    }

    /// The ops an `if` with one empty arm gives this condition, without the try block itself.
    fn cond_ops(cond: Cond) -> Vec<Op> {
        let stmt = Stmt::If { arms: vec![(cond, Vec::new())], otherwise: None, origin: ORIGIN.into() };
        let mut ops = lower(&[stmt]);
        assert_eq!(ops.first(), Some(&native("try_begin", Vec::new())));
        assert_eq!(ops.pop(), Some(native("try_end", Vec::new())));
        ops.remove(0);
        ops
    }

    #[test]
    fn plain_operations_pass_through() {
        let op = Op { name: "change_screen_map".into(), args: vec![], origin: "a.uw:2".into() };
        assert_eq!(lower(&[Stmt::Op(op.clone())]), vec![op]);
    }

    #[test]
    fn assignment_of_a_value_is_assign() {
        assert_eq!(
            lower(&[assign(global("$gold"), AssignOp::Set, int(100))]),
            vec![native("assign", vec![global("$gold"), Value::Int(100)])]
        );
    }

    #[test]
    fn compound_assignment_uses_the_matching_val_operation() {
        for (op, name) in [
            (AssignOp::Add, "val_add"),
            (AssignOp::Sub, "val_sub"),
            (AssignOp::Mul, "val_mul"),
            (AssignOp::Div, "val_div"),
            (AssignOp::Mod, "val_mod"),
        ] {
            assert_eq!(
                lower(&[assign(global("$gold"), op, int(50))]),
                vec![native(name, vec![global("$gold"), Value::Int(50)])]
            );
        }
    }

    #[test]
    fn compound_assignment_of_an_expression_goes_through_a_temporary() {
        let sum = binary(BinOp::Add, value("$a"), int(1));
        assert_eq!(
            lower(&[assign(global(":x"), AssignOp::Add, sum)]),
            vec![
                native("store_add", vec![global(":_uw0"), global("$a"), Value::Int(1)]),
                native("val_add", vec![global(":x"), global(":_uw0")]),
            ]
        );
    }

    #[test]
    fn nested_expression_is_lowered_post_order_into_temporaries() {
        // §1: `:x = $gold + 2 * :y` -> store_mul, then store_add.
        let expr = binary(BinOp::Add, value("$gold"), binary(BinOp::Mul, int(2), value(":y")));
        assert_eq!(
            lower(&[assign(global(":x"), AssignOp::Set, expr)]),
            vec![
                native("store_mul", vec![global(":_uw0"), Value::Int(2), global(":y")]),
                native("store_add", vec![global(":x"), global("$gold"), global(":_uw0")]),
            ]
        );
    }

    #[test]
    fn unary_minus_folds_a_literal_and_otherwise_uses_store_sub() {
        assert_eq!(
            lower(&[assign(global(":x"), AssignOp::Set, Expr::Neg(Box::new(int(5))))]),
            vec![native("assign", vec![global(":x"), Value::Int(-5)])]
        );
        assert_eq!(
            lower(&[assign(global(":x"), AssignOp::Set, Expr::Neg(Box::new(value(":y"))))]),
            vec![native("store_sub", vec![global(":x"), Value::Int(0), global(":y")])]
        );
    }

    #[test]
    fn comparison_sides_that_are_expressions_go_through_temporaries() {
        let cond = compare(
            CmpOp::Gt,
            binary(BinOp::Add, value("$a"), int(1)),
            binary(BinOp::Mul, value("$b"), int(2)),
        );
        assert_eq!(
            cond_ops(cond),
            vec![
                native("store_add", vec![global(":_uw0"), global("$a"), Value::Int(1)]),
                native("store_mul", vec![global(":_uw1"), global("$b"), Value::Int(2)]),
                native("gt", vec![global(":_uw0"), global(":_uw1")]),
            ]
        );
    }

    #[test]
    fn if_else_if_else_becomes_try_blocks() {
        let stmt = Stmt::If {
            arms: vec![
                (
                    compare(CmpOp::Ge, value("$gold"), int(100)),
                    vec![assign(global("$gold"), AssignOp::Sub, int(100))],
                ),
                (compare(CmpOp::Gt, value("$gold"), int(0)), Vec::new()),
            ],
            otherwise: Some(vec![assign(global("$gold"), AssignOp::Set, int(1))]),
            origin: ORIGIN.into(),
        };
        assert_eq!(
            lower(&[stmt]),
            vec![
                native("try_begin", vec![]),
                native("ge", vec![global("$gold"), Value::Int(100)]),
                native("val_sub", vec![global("$gold"), Value::Int(100)]),
                native("else_try", vec![]),
                native("gt", vec![global("$gold"), Value::Int(0)]),
                native("else_try", vec![]),
                native("assign", vec![global("$gold"), Value::Int(1)]),
                native("try_end", vec![]),
            ]
        );
    }

    #[test]
    fn an_or_group_inside_an_and_chain_uses_this_or_next() {
        let op_cond = |name: &str, args: Vec<Value>| Cond::Op(native(name, args));
        let cond = Cond::And(vec![
            compare(CmpOp::Eq, value("$x"), int(1)),
            Cond::Or(vec![
                compare(CmpOp::Gt, value("$y"), int(2)),
                compare(CmpOp::Lt, value("$y"), int(5)),
            ]),
            op_cond("party_is_active", vec![global("p_town_6")]),
        ]);
        assert_eq!(
            cond_ops(cond),
            vec![
                native("eq", vec![global("$x"), Value::Int(1)]),
                native("this_or_next|gt", vec![global("$y"), Value::Int(2)]),
                native("lt", vec![global("$y"), Value::Int(5)]),
                native("party_is_active", vec![global("p_town_6")]),
            ]
        );
    }

    #[test]
    fn an_or_group_with_a_computed_side_uses_a_flag_not_this_or_next() {
        // `$x + 1 > 2 or $y == 3`: the temporary for `$x + 1` must not sit between two
        // this_or_next conditions (it would be ORed and always succeed).
        let cond = Cond::And(vec![Cond::Or(vec![
            compare(CmpOp::Gt, binary(BinOp::Add, value("$x"), int(1)), int(2)),
            compare(CmpOp::Eq, value("$y"), int(3)),
        ])]);
        let ops = cond_ops(cond);
        assert!(ops.iter().all(|op| !op.name.starts_with("this_or_next|")), "{ops:?}");
        assert_eq!(ops.last().unwrap(), &native("eq", vec![global(":_uw0"), Value::Int(1)]));
    }

    #[test]
    fn a_mixed_and_or_condition_uses_a_flag_temporary() {
        let cond = Cond::Or(vec![
            Cond::And(vec![
                compare(CmpOp::Eq, value("$x"), int(1)),
                compare(CmpOp::Ne, value("$y"), int(2)),
            ]),
            compare(CmpOp::Eq, value(":z"), int(3)),
        ]);
        assert_eq!(
            cond_ops(cond),
            vec![
                native("assign", vec![global(":_uw0"), Value::Int(0)]),
                native("try_begin", vec![]),
                native("eq", vec![global("$x"), Value::Int(1)]),
                native("neq", vec![global("$y"), Value::Int(2)]),
                native("assign", vec![global(":_uw0"), Value::Int(1)]),
                native("try_end", vec![]),
                native("try_begin", vec![]),
                native("eq", vec![global(":z"), Value::Int(3)]),
                native("assign", vec![global(":_uw0"), Value::Int(1)]),
                native("try_end", vec![]),
                native("eq", vec![global(":_uw0"), Value::Int(1)]),
            ]
        );
    }

    #[test]
    fn a_bare_pure_or_at_the_top_uses_this_or_next() {
        let cond = Cond::Or(vec![
            compare(CmpOp::Eq, value("$x"), int(1)),
            compare(CmpOp::Eq, value("$y"), int(2)),
            compare(CmpOp::Lt, value("$z"), int(3)),
        ]);
        assert_eq!(
            cond_ops(cond),
            vec![
                native("this_or_next|eq", vec![global("$x"), Value::Int(1)]),
                native("this_or_next|eq", vec![global("$y"), Value::Int(2)]),
                native("lt", vec![global("$z"), Value::Int(3)]),
            ]
        );
    }

    #[test]
    fn a_bare_or_with_a_computed_side_still_uses_a_flag_temporary() {
        let cond = Cond::Or(vec![
            compare(CmpOp::Eq, value("$x"), int(1)),
            compare(CmpOp::Eq, binary(BinOp::Add, value("$y"), int(1)), int(2)),
        ]);
        let ops = cond_ops(cond);
        assert!(ops.iter().all(|op| !op.name.starts_with("this_or_next|")), "{ops:?}");
        assert_eq!(ops.first().unwrap(), &native("assign", vec![global(":_uw0"), Value::Int(0)]));
    }

    #[test]
    fn not_on_a_comparison_flips_the_operator() {
        for (op, flipped) in [
            (CmpOp::Eq, "neq"),
            (CmpOp::Ne, "eq"),
            (CmpOp::Gt, "le"),
            (CmpOp::Ge, "lt"),
            (CmpOp::Lt, "ge"),
            (CmpOp::Le, "gt"),
        ] {
            let cond = Cond::Not(Box::new(compare(op, value("$a"), int(1))));
            assert_eq!(
                cond_ops(cond),
                vec![native(flipped, vec![global("$a"), Value::Int(1)])],
                "not {op:?}"
            );
        }
    }

    #[test]
    fn not_on_a_condition_operation_sets_the_neg_flag() {
        let cond = Cond::Not(Box::new(Cond::Op(native("party_is_active", vec![global("p_town_6")]))));
        assert_eq!(
            cond_ops(cond),
            vec![native("neg|party_is_active", vec![global("p_town_6")])]
        );
    }

    #[test]
    fn not_on_a_composite_uses_a_flag_and_tests_it_for_zero() {
        let cond = Cond::Not(Box::new(Cond::And(vec![
            compare(CmpOp::Eq, value("$x"), int(1)),
            compare(CmpOp::Eq, value("$y"), int(2)),
        ])));
        assert_eq!(
            cond_ops(cond),
            vec![
                native("assign", vec![global(":_uw0"), Value::Int(0)]),
                native("try_begin", vec![]),
                native("eq", vec![global("$x"), Value::Int(1)]),
                native("eq", vec![global("$y"), Value::Int(2)]),
                native("assign", vec![global(":_uw0"), Value::Int(1)]),
                native("try_end", vec![]),
                native("eq", vec![global(":_uw0"), Value::Int(0)]),
            ]
        );
    }

    #[test]
    fn a_second_block_numbers_its_temporaries_from_zero_again() {
        let block = [assign(
            global("$x"),
            AssignOp::Set,
            binary(BinOp::Add, value("$a"), binary(BinOp::Mul, int(2), value("$b"))),
        )];
        let expected = vec![
            native("store_mul", vec![global(":_uw0"), Value::Int(2), global("$b")]),
            native("store_add", vec![global("$x"), global("$a"), global(":_uw0")]),
        ];
        for _ in 0..2 {
            assert_eq!(lower(&block), expected);
        }
    }
}
