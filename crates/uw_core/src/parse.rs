//! Parser of the UW core language v0 (docs/dil/CORE-v0.md §3): tokens → [`ast::File`].
//!
//! Statements have no terminator, so the line breaks the lexer keeps decide where a statement or an item
//! ends (`return` takes a value only on its own line). Errors are collected per file: a broken item is
//! reported and skipped, so one file can show several messages.

use crate::ast::{self, AssignOp, BinOp, Block, Call, Expr, Ident, Item, Path, Span, Stmt, TextPart, VarDecl};
use crate::lex::{line_at, lex_from, Tok, Token};
use crate::Error;

/// Parses one core file. `feature` is the file's feature name (`main` for `main.uw`), `file` the path used
/// in messages. `Err` carries every error found, in source order.
pub fn parse(feature: &str, file: &str, text: &str) -> Result<ast::File, Vec<Error>> {
    let (toks, errors) = crate::lex::lex(file, text);
    let mut p = Parser { file, src: text, toks, pos: 0, errors };
    let items = p.file();
    if p.errors.is_empty() { Ok(ast::File { feature: feature.to_string(), items }) } else { Err(p.errors) }
}

fn join(a: Span, b: Span) -> Span {
    Span { start: a.start.min(b.start), end: a.end.max(b.end), line: a.line }
}

fn binary(op: BinOp, left: Expr, right: Expr) -> Expr {
    let span = join(left.span(), right.span());
    Expr::Binary { op, left: Box::new(left), right: Box::new(right), span }
}

fn describe(tok: &Tok) -> String {
    match tok {
        Tok::Word(w) => format!("'{w}'"),
        Tok::Num(milli) if milli % 1000 == 0 => format!("number {}", milli / 1000),
        Tok::Num(milli) => format!("number {}", *milli as f64 / 1000.0),
        Tok::Percent(value) => format!("'{value}%'"),
        Tok::Rate { milli, unit } => {
            let unit = match unit {
                ast::RateUnit::Second => "second",
                ast::RateUnit::Minute => "minute",
                ast::RateUnit::Hour => "hour",
            };
            format!("'{} per {unit}'", *milli as f64 / 1000.0)
        }
        Tok::Color(rgb) => format!("color #{rgb:06X}"),
        Tok::Str { .. } => "a text".into(),
        Tok::LBrace => "'{'".into(),
        Tok::RBrace => "'}'".into(),
        Tok::LParen => "'('".into(),
        Tok::RParen => "')'".into(),
        Tok::Comma => "','".into(),
        Tok::Dot => "'.'".into(),
        Tok::DotDot => "'..'".into(),
        Tok::Assign => "'='".into(),
        Tok::PlusAssign => "'+='".into(),
        Tok::MinusAssign => "'-='".into(),
        Tok::StarAssign => "'*='".into(),
        Tok::SlashAssign => "'/='".into(),
        Tok::EqEq => "'=='".into(),
        Tok::NotEq => "'!='".into(),
        Tok::Lt => "'<'".into(),
        Tok::Le => "'<='".into(),
        Tok::Gt => "'>'".into(),
        Tok::Ge => "'>='".into(),
        Tok::Plus => "'+'".into(),
        Tok::Minus => "'-'".into(),
        Tok::Star => "'*'".into(),
        Tok::Slash => "'/'".into(),
        Tok::Newline => "the end of the line".into(),
        Tok::Eof => "the end of the file".into(),
    }
}

const ITEM_LIST: &str = "an item (const, var, fn, on, while, ui)";

struct Parser<'a> {
    file: &'a str,
    src: &'a str,
    toks: Vec<Token>,
    pos: usize,
    errors: Vec<Error>,
}

impl<'a> Parser<'a> {
    // ------------------------------------------------------------ token stream

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

    fn at(&self, tok: &Tok) -> bool {
        &self.peek().tok == tok
    }

    fn at_word(&self, word: &str) -> bool {
        matches!(&self.peek().tok, Tok::Word(w) if w == word)
    }

    fn skip_newlines(&mut self) {
        while self.peek().tok == Tok::Newline {
            self.bump();
        }
    }

    fn error(&mut self, span: Span, message: impl Into<String>) {
        self.errors.push(Error { file: self.file.to_string(), line: span.line, message: message.into() });
    }

    /// Reports `expected <what>, found <token>` at the current token.
    fn expected(&mut self, what: &str) -> Span {
        let t = self.peek().clone();
        let found = describe(&t.tok);
        self.error(t.span, format!("expected {what}, found {found}"));
        t.span
    }

    fn expect(&mut self, tok: Tok, what: &str) -> Result<Token, ()> {
        if self.at(&tok) {
            Ok(self.bump())
        } else {
            self.expected(what);
            Err(())
        }
    }

    fn ident(&mut self, what: &str) -> Result<Ident, ()> {
        let t = self.peek().clone();
        match t.tok {
            Tok::Word(text) => {
                self.bump();
                Ok(Ident { text, span: t.span })
            }
            _ => {
                self.expected(what);
                Err(())
            }
        }
    }

    /// `a.b.c`
    fn path(&mut self) -> Result<Path, ()> {
        let first = self.ident("a name")?;
        let start = first.span;
        let mut parts = vec![first];
        while self.at(&Tok::Dot) {
            self.bump();
            parts.push(self.ident("a name after '.'")?);
        }
        let end = parts.last().map(|p| p.span).unwrap_or(start);
        Ok(Path { parts, span: join(start, end) })
    }

    /// Skips a broken item or statement: to the end of the line, past any braces it opened.
    fn recover(&mut self) {
        let mut depth = 0usize;
        loop {
            let tok = self.peek().tok.clone();
            match tok {
                Tok::Eof => return,
                Tok::LBrace => {
                    depth += 1;
                    self.bump();
                }
                Tok::RBrace => {
                    if depth == 0 {
                        return;
                    }
                    depth -= 1;
                    self.bump();
                }
                Tok::Newline => {
                    self.bump();
                    if depth == 0 {
                        return;
                    }
                }
                _ => {
                    self.bump();
                }
            }
        }
    }

    // ------------------------------------------------------------ file and items

    fn file(&mut self) -> Vec<Item> {
        let mut items = Vec::new();
        loop {
            self.skip_newlines();
            if self.at(&Tok::Eof) {
                return items;
            }
            match self.item() {
                Ok(item) => items.push(item),
                Err(()) => {
                    let before = self.pos;
                    self.recover();
                    // A `}` with no block open would leave the loop where it was.
                    if self.pos == before {
                        self.bump();
                    }
                }
            }
        }
    }

    fn item(&mut self) -> Result<Item, ()> {
        let t = self.peek().clone();
        if let Tok::Word(word) = &t.tok {
            match word.as_str() {
                "const" => return self.const_item(),
                "var" => return self.var_item().map(Item::Var),
                "fn" => return self.fn_item(),
                "on" => return self.on_item(),
                "while" => return self.while_item(),
                "otherwise" => {
                    self.error(t.span, "`otherwise` must follow the block of a `while`");
                    return Err(());
                }
                "ui" if self.toks.get(self.pos + 1).is_some_and(|n| n.tok == Tok::Dot) => return self.ui_item(),
                _ => {}
            }
        }
        self.expected(ITEM_LIST);
        Err(())
    }

    /// `on <event> (<bound>)? (when <cond>)? { … }`
    fn on_item(&mut self) -> Result<Item, ()> {
        let start = self.bump().span;
        let event = self.path()?;
        let mut bound = None;
        if self.at(&Tok::LParen) {
            self.bump();
            bound = Some(self.ident("the bound object's name")?);
            self.expect(Tok::RParen, "')'")?;
        }
        let mut when = None;
        if self.at_word("when") {
            self.bump();
            when = Some(self.expr()?);
        }
        let body = self.block()?;
        let end = body.last().map(stmt_span).unwrap_or(event.span);
        Ok(Item::On { event, bound, when, body, span: join(start, end) })
    }

    /// `const NAME = expr`
    fn const_item(&mut self) -> Result<Item, ()> {
        let start = self.bump().span;
        let name = self.ident("a constant name")?;
        self.expect(Tok::Assign, "'='")?;
        let value = self.expr()?;
        let span = join(start, value.span());
        Ok(Item::Const { name, value, span })
    }

    /// `var target ("=" expr)? ("in" expr ".." expr)?`, `target := NAME | KIND "." NAME`
    fn var_item(&mut self) -> Result<VarDecl, ()> {
        let start = self.bump().span;
        let first = self.ident("a variable name")?;
        let (kind, name) = if self.at(&Tok::Dot) {
            self.bump();
            let name = self.ident("a field name after '.'")?;
            (Some(first), name)
        } else {
            (None, first)
        };
        let mut init = None;
        if self.at(&Tok::Assign) {
            self.bump();
            init = Some(self.expr()?);
        }
        let mut range = None;
        if self.at_word("in") {
            self.bump();
            let lo = self.expr()?;
            self.expect(Tok::DotDot, "'..'")?;
            let hi = self.expr()?;
            range = Some((lo, hi));
        }
        let end = range
            .as_ref()
            .map(|(_, hi)| hi.span())
            .or_else(|| init.as_ref().map(|e| e.span()))
            .unwrap_or(name.span);
        Ok(VarDecl { kind, name, init, range, span: join(start, end) })
    }

    /// `fn NAME (NAME, …) { … }`
    fn fn_item(&mut self) -> Result<Item, ()> {
        let start = self.bump().span;
        let name = self.ident("a function name")?;
        self.expect(Tok::LParen, "'('")?;
        let mut params = Vec::new();
        self.skip_newlines();
        if !self.at(&Tok::RParen) {
            loop {
                params.push(self.ident("a parameter name")?);
                self.skip_newlines();
                if self.at(&Tok::Comma) {
                    self.bump();
                    self.skip_newlines();
                } else {
                    break;
                }
            }
        }
        let close = self.expect(Tok::RParen, "')'")?;
        let body = self.block()?;
        let end = body.last().map(stmt_span).unwrap_or(close.span);
        Ok(Item::Fn { name, params, body, span: join(start, end) })
    }

    /// `while in <context> { … }` | `while <cond> { … } (otherwise { … })?`
    fn while_item(&mut self) -> Result<Item, ()> {
        let start = self.bump().span;
        if self.at_word("in") {
            self.bump();
            let context = self.path()?;
            let body = self.block()?;
            let end = body.last().map(stmt_span).unwrap_or(context.span);
            return Ok(Item::WhileIn { context, body, span: join(start, end) });
        }
        let cond = self.expr()?;
        let body = self.block()?;
        // `otherwise` may sit on the line after the closing `}` of the `while` block.
        self.skip_newlines();
        let mut otherwise = None;
        if self.at_word("otherwise") {
            self.bump();
            otherwise = Some(self.block()?);
        }
        let end = otherwise
            .as_ref()
            .and_then(|b| b.last())
            .map(stmt_span)
            .or_else(|| body.last().map(stmt_span))
            .unwrap_or(cond.span());
        Ok(Item::While { cond, body, otherwise, span: join(start, end) })
    }

    /// `ui.<part> <name> { prop = expr … }`
    fn ui_item(&mut self) -> Result<Item, ()> {
        let start = self.bump().span;
        self.expect(Tok::Dot, "'.' after 'ui'")?;
        let part = self.ident("a ui part (bar, text, image, button, panel)")?;
        let name = self.ident("a name for this ui part")?;
        let open = self.expect(Tok::LBrace, "'{'")?;
        let mut props = Vec::new();
        loop {
            self.skip_newlines();
            match self.peek().tok {
                Tok::RBrace => {
                    let close = self.bump().span;
                    return Ok(Item::Ui { part, name, props, span: join(start, close) });
                }
                Tok::Eof => {
                    self.error(open.span, "this `{` is never closed: expected '}' for the ui part");
                    return Err(());
                }
                _ => {}
            }
            let key = self.ident("a property name")?;
            self.expect(Tok::Assign, "'='")?;
            let value = self.expr()?;
            props.push((key, value));
        }
    }

    // ------------------------------------------------------------ blocks and statements

    fn block(&mut self) -> Result<Block, ()> {
        let open = self.expect(Tok::LBrace, "'{'")?;
        let mut stmts = Vec::new();
        loop {
            self.skip_newlines();
            match self.peek().tok {
                Tok::RBrace => {
                    self.bump();
                    return Ok(stmts);
                }
                Tok::Eof => {
                    self.error(open.span, "this `{` is never closed: expected '}'");
                    return Err(());
                }
                _ => {}
            }
            match self.stmt() {
                Ok(stmt) => stmts.push(stmt),
                Err(()) => self.recover(),
            }
        }
    }

    fn stmt(&mut self) -> Result<Stmt, ()> {
        let t = self.peek().clone();
        if let Tok::Word(word) = &t.tok {
            match word.as_str() {
                "let" => {
                    let start = self.bump().span;
                    let name = self.ident("a name")?;
                    self.expect(Tok::Assign, "'='")?;
                    let value = self.expr()?;
                    let span = join(start, value.span());
                    return Ok(Stmt::Let { name, value, span });
                }
                "if" => return self.if_stmt(),
                "for" => return self.for_stmt(),
                "return" => {
                    let start = self.bump().span;
                    // `return` takes a value only on its own line: the next line starts a new statement.
                    let value = if matches!(self.peek().tok, Tok::Newline | Tok::RBrace | Tok::Eof) {
                        None
                    } else {
                        Some(self.expr()?)
                    };
                    let span = match &value {
                        Some(e) => join(start, e.span()),
                        None => start,
                    };
                    return Ok(Stmt::Return { value, span });
                }
                _ => {}
            }
        }
        let place = self.path()?;
        let op = match self.peek().tok {
            Tok::Assign => Some(AssignOp::Set),
            Tok::PlusAssign => Some(AssignOp::Add),
            Tok::MinusAssign => Some(AssignOp::Sub),
            Tok::StarAssign => Some(AssignOp::Mul),
            Tok::SlashAssign => Some(AssignOp::Div),
            _ => None,
        };
        if let Some(op) = op {
            self.bump();
            let value = self.expr()?;
            let span = join(place.span, value.span());
            return Ok(Stmt::Assign { place, op, value, span });
        }
        if self.at(&Tok::LParen) {
            return Ok(Stmt::Call(self.call(place)?));
        }
        self.expected("'=', '+=', '-=', '*=', '/=' or '(' after this name");
        Err(())
    }

    fn if_stmt(&mut self) -> Result<Stmt, ()> {
        let start = self.bump().span;
        let cond = self.expr()?;
        let then = self.block()?;
        self.skip_newlines();
        let mut otherwise = None;
        if self.at_word("else") {
            self.bump();
            if self.at_word("if") {
                otherwise = Some(vec![self.if_stmt()?]);
            } else {
                otherwise = Some(self.block()?);
            }
        }
        let end = otherwise
            .as_ref()
            .and_then(|b| b.last())
            .map(stmt_span)
            .or_else(|| then.last().map(stmt_span))
            .unwrap_or(cond.span());
        Ok(Stmt::If { cond, then, otherwise, span: join(start, end) })
    }

    fn for_stmt(&mut self) -> Result<Stmt, ()> {
        let start = self.bump().span;
        let var = self.ident("a name")?;
        if !self.at_word("in") {
            self.expected("'in'");
            return Err(());
        }
        self.bump();
        let iter = self.expr()?;
        let filter = if self.at_word("where") {
            self.bump();
            Some(self.expr()?)
        } else {
            None
        };
        let body = self.block()?;
        let end = body.last().map(stmt_span).unwrap_or(iter.span());
        Ok(Stmt::For { var, iter, filter, body, span: join(start, end) })
    }

    // ------------------------------------------------------------ expressions

    fn expr(&mut self) -> Result<Expr, ()> {
        self.or_expr()
    }

    fn or_expr(&mut self) -> Result<Expr, ()> {
        let mut left = self.and_expr()?;
        while self.at_word("or") {
            self.bump();
            self.skip_newlines();
            let right = self.and_expr()?;
            left = binary(BinOp::Or, left, right);
        }
        Ok(left)
    }

    fn and_expr(&mut self) -> Result<Expr, ()> {
        let mut left = self.not_expr()?;
        while self.at_word("and") {
            self.bump();
            self.skip_newlines();
            let right = self.not_expr()?;
            left = binary(BinOp::And, left, right);
        }
        Ok(left)
    }

    fn not_expr(&mut self) -> Result<Expr, ()> {
        if self.at_word("not") {
            let start = self.bump().span;
            let expr = self.not_expr()?;
            let span = join(start, expr.span());
            return Ok(Expr::Unary { neg: false, not: true, expr: Box::new(expr), span });
        }
        self.cmp_expr()
    }

    fn cmp_expr(&mut self) -> Result<Expr, ()> {
        let mut left = self.add_expr()?;
        loop {
            let op = match self.peek().tok {
                Tok::EqEq => BinOp::Eq,
                Tok::NotEq => BinOp::Ne,
                Tok::Lt => BinOp::Lt,
                Tok::Le => BinOp::Le,
                Tok::Gt => BinOp::Gt,
                Tok::Ge => BinOp::Ge,
                _ => return Ok(left),
            };
            self.bump();
            self.skip_newlines();
            let right = self.add_expr()?;
            left = binary(op, left, right);
        }
    }

    fn add_expr(&mut self) -> Result<Expr, ()> {
        let mut left = self.mul_expr()?;
        loop {
            let op = match self.peek().tok {
                Tok::Plus => BinOp::Add,
                Tok::Minus => BinOp::Sub,
                _ => return Ok(left),
            };
            self.bump();
            self.skip_newlines();
            let right = self.mul_expr()?;
            left = binary(op, left, right);
        }
    }

    fn mul_expr(&mut self) -> Result<Expr, ()> {
        let mut left = self.unary_expr()?;
        loop {
            let op = match self.peek().tok {
                Tok::Star => BinOp::Mul,
                Tok::Slash => BinOp::Div,
                _ => return Ok(left),
            };
            self.bump();
            self.skip_newlines();
            let right = self.unary_expr()?;
            left = binary(op, left, right);
        }
    }

    fn unary_expr(&mut self) -> Result<Expr, ()> {
        if self.at(&Tok::Minus) {
            let start = self.bump().span;
            let expr = self.unary_expr()?;
            let span = join(start, expr.span());
            return Ok(Expr::Unary { neg: true, not: false, expr: Box::new(expr), span });
        }
        let expr = self.primary()?;
        if self.at(&Tok::LParen) {
            let callee = match expr {
                Expr::Path(path) => path,
                other => {
                    let span = other.span();
                    self.error(span, "only a name can be called, this is a value");
                    return Err(());
                }
            };
            return Ok(Expr::Call(self.call(callee)?));
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        let t = self.peek().clone();
        match t.tok {
            Tok::Num(milli) => {
                self.bump();
                Ok(Expr::Number { milli, span: t.span })
            }
            Tok::Percent(value) => {
                self.bump();
                Ok(Expr::Percent { value, span: t.span })
            }
            Tok::Rate { milli, unit } => {
                self.bump();
                Ok(Expr::Rate { milli, unit, span: t.span })
            }
            Tok::Color(rgb) => {
                self.bump();
                Ok(Expr::Color { rgb, span: t.span })
            }
            Tok::Str { start, end } => {
                self.bump();
                self.text(start, end)
            }
            Tok::Word(word) => match word.as_str() {
                "true" => {
                    self.bump();
                    Ok(Expr::Bool { value: true, span: t.span })
                }
                "false" => {
                    self.bump();
                    Ok(Expr::Bool { value: false, span: t.span })
                }
                _ => Ok(Expr::Path(self.path()?)),
            },
            Tok::LParen => self.tuple_or_paren(),
            _ => {
                self.expected("a value");
                Err(())
            }
        }
    }

    /// `(expr)` is just that expression; `(expr, expr, …)` is a tuple.
    fn tuple_or_paren(&mut self) -> Result<Expr, ()> {
        let open = self.bump().span;
        self.skip_newlines();
        if self.at(&Tok::RParen) {
            let close = self.bump().span;
            return Ok(Expr::Tuple { items: Vec::new(), span: join(open, close) });
        }
        let first = self.expr()?;
        self.skip_newlines();
        if !self.at(&Tok::Comma) {
            self.expect(Tok::RParen, "')'")?;
            return Ok(first);
        }
        let mut items = vec![first];
        while self.at(&Tok::Comma) {
            self.bump();
            self.skip_newlines();
            items.push(self.expr()?);
            self.skip_newlines();
        }
        let close = self.expect(Tok::RParen, "')' or ','")?;
        Ok(Expr::Tuple { items, span: join(open, close.span) })
    }

    /// `path "(" (expr ("," expr)*)? ")"`
    fn call(&mut self, callee: Path) -> Result<Call, ()> {
        let open = self.expect(Tok::LParen, "'('")?;
        let mut args = Vec::new();
        self.skip_newlines();
        if !self.at(&Tok::RParen) {
            loop {
                args.push(self.expr()?);
                self.skip_newlines();
                if self.at(&Tok::Comma) {
                    self.bump();
                    self.skip_newlines();
                } else {
                    break;
                }
            }
        }
        let close = self.expect(Tok::RParen, "')' after the arguments")?;
        let span = join(join(callee.span, open.span), close.span);
        Ok(Call { callee, args, span })
    }

    // ------------------------------------------------------------ text with `{expr}`

    /// `"a {expr} b"` → [`TextPart`]s: literal text and expressions alternate (§2).
    fn text(&mut self, start: usize, end: usize) -> Result<Expr, ()> {
        let mut parts: Vec<TextPart> = Vec::new();
        let mut literal = String::new();
        let mut i = start;
        while i < end {
            let c = self.src[i..].chars().next().unwrap_or(' ');
            match c {
                '\\' => {
                    let esc_at = i;
                    let next_at = i + 1;
                    let Some(esc) = self.src[next_at..end].chars().next() else {
                        self.error(Span { start: esc_at, end: next_at, line: line_at(self.src, esc_at) }, "'\\' at the end of a text");
                        i = next_at;
                        continue;
                    };
                    let decoded = match esc {
                        'n' => Some('\n'),
                        't' => Some('\t'),
                        '"' => Some('"'),
                        '\\' => Some('\\'),
                        '{' => Some('{'),
                        '}' => Some('}'),
                        _ => None,
                    };
                    match decoded {
                        Some(ch) => literal.push(ch),
                        None => {
                            let line = line_at(self.src, esc_at);
                            self.error(
                                Span { start: esc_at, end: next_at + esc.len_utf8(), line },
                                format!("unknown escape '\\{esc}': use \\n, \\t, \\\", \\\\ or \\{{"),
                            );
                            literal.push(esc);
                        }
                    }
                    i = next_at + esc.len_utf8();
                }
                '{' => {
                    if !literal.is_empty() {
                        parts.push(TextPart::Literal(std::mem::take(&mut literal)));
                    }
                    let Some(close) = matching_brace(self.src, i) else {
                        self.error(
                            Span { start: i, end: i + 1, line: line_at(self.src, i) },
                            "this '{' in a text is never closed: expected '}'",
                        );
                        i = end;
                        continue;
                    };
                    let inner = i + 1;
                    let line = line_at(self.src, inner);
                    let (toks, errors) = lex_from(self.file, self.src, inner, close, line);
                    let mut sub = Parser { file: self.file, src: self.src, toks, pos: 0, errors };
                    match sub.expr() {
                        Ok(expr) => {
                            if !sub.at(&Tok::Eof) {
                                let t = sub.peek().clone();
                                let found = describe(&t.tok);
                                sub.error(t.span, format!("expected '}}' after this value in the text, found {found}"));
                            }
                            parts.push(TextPart::Expr(expr));
                        }
                        Err(()) => {}
                    }
                    self.errors.append(&mut sub.errors);
                    i = close + 1;
                }
                _ => {
                    literal.push(c);
                    i += c.len_utf8();
                }
            }
        }
        if !literal.is_empty() {
            parts.push(TextPart::Literal(literal));
        }
        // The span covers the quotes too, like the other literals (`start` is just after the opening one).
        let span = Span { start: start.saturating_sub(1), end: end + 1, line: line_at(self.src, start) };
        Ok(Expr::Text { parts, span })
    }
}

/// Byte offset of the `}` matching the `{` at `open`, skipping quoted text.
fn matching_brace(text: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut i = open;
    while i < text.len() {
        let c = text[i..].chars().next()?;
        match c {
            '\\' => {
                i += 1;
                if let Some(next) = text[i..].chars().next() {
                    i += next.len_utf8();
                }
            }
            '"' => {
                i += 1;
                while let Some(c) = text[i..].chars().next() {
                    i += c.len_utf8();
                    match c {
                        '\\' => {
                            if let Some(next) = text[i..].chars().next() {
                                i += next.len_utf8();
                            }
                        }
                        '"' => break,
                        _ => {}
                    }
                }
            }
            '{' => {
                depth += 1;
                i += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
                i += 1;
            }
            _ => i += c.len_utf8(),
        }
    }
    None
}

fn stmt_span(stmt: &Stmt) -> Span {
    match stmt {
        Stmt::Let { span, .. }
        | Stmt::Assign { span, .. }
        | Stmt::If { span, .. }
        | Stmt::For { span, .. }
        | Stmt::Return { span, .. } => *span,
        Stmt::Call(call) => call.span,
    }
}
