use std::{collections::BTreeMap, rc::Rc};

use crate::{
    asm::{Instruction, Item, Syntax, Value},
    robin::types::UnaryOp,
    utils::{get_hash, Either},
};

use super::types::{AssignOp, BinaryOp, BlockType, Expression, Statement, TopLevelSyntax};

struct Scope<'a> {
    constants: &'a BTreeMap<Rc<str>, Value>,
    globals: &'a BTreeMap<Rc<str>, Value>,
    functions: &'a BTreeMap<Rc<str>, Vec<Rc<str>>>,
    parameters: &'a BTreeMap<Rc<str>, Value>,
    locals: &'a BTreeMap<Rc<str>, Value>,
}

impl<'a> Scope<'a> {
    pub fn get(&self, ident: &str) -> Option<Value> {
        if let Some(local) = self.locals.get(ident) {
            return Some(local.clone());
        };
        if let Some(param) = self.parameters.get(ident) {
            return Some(param.clone());
        };
        if let Some(global) = self.globals.get(ident) {
            return Some(global.clone());
        };
        if self.functions.get(ident).is_some() {
            return Some(Value::Label(ident.into()));
        }
        None
    }

    pub fn get_fn(&self, ident: &str) -> Option<Vec<Rc<str>>> {
        self.functions.get(ident).cloned()
    }

    pub fn get_constant(&self, ident: &str) -> Option<Value> {
        self.constants.get(ident).cloned()
    }
}

#[derive(Debug)]
pub enum Error {
    CompilationFailed(String),
    InvalidIdentifier(Rc<str>),
    InvalidSyntax(TopLevelSyntax),
    InvalidStatement(Statement),
    InvalidExpression(Expression),
}

pub fn compile(src: Vec<TopLevelSyntax>) -> Result<Vec<Syntax>, Error> {
    let mut function_signatures = BTreeMap::new();
    let mut function_bodies = BTreeMap::new();
    let mut statics = BTreeMap::new();
    let mut constants = BTreeMap::new();
    let mut statics_syntax = Vec::new();
    for syn in src {
        match syn {
            TopLevelSyntax::Function(name, args, body) => {
                function_signatures.insert(name.clone(), args.clone());
                function_bodies.insert(name, (args, body));
            }
            TopLevelSyntax::Global(name, Expression::String(str)) => {
                let label: Rc<str> = format!("_global_{name}").into();
                statics.insert(name.clone(), Value::Label(label.clone()));
                statics_syntax.push(Syntax::Label(label));
                statics_syntax.extend(crate::asm::string_literal(&str));
            }
            TopLevelSyntax::Global(name, Expression::Array(arr)) => {
                let label: Rc<str> = format!("_global_{name}").into();
                statics.insert(name.clone(), Value::Label(label.clone()));
                statics_syntax.push(Syntax::Label(label));
                statics_syntax.extend(arr.into_iter().map(Syntax::Literal));
                statics_syntax.push(Syntax::Literal(0));
            }
            TopLevelSyntax::Constant(name, Expression::Int(int)) => {
                constants.insert(name, Value::Given(int));
            }
            _ => return Err(Error::InvalidSyntax(syn)),
        }
    }
    let mut output = Vec::new();
    let Some((main_args, main_body)) = function_bodies.remove("main") else {return Err(Error::CompilationFailed(String::from("Missing `main` function")))};
    if !main_args.is_empty() {
        return Err(Error::CompilationFailed(String::from(
            "Main function can't accept arguments",
        )));
    }
    output.extend(compile_fn(
        &"main".into(),
        &main_args,
        main_body,
        &statics,
        &constants,
        &function_signatures,
    )?);
    for (name, (args, body)) in function_bodies {
        output.extend(compile_fn(
            &name,
            &args,
            body,
            &statics,
            &constants,
            &function_signatures,
        )?);
    }
    println!("{output:?}");
    let mut output = output
        .into_iter()
        .map(Either::left)
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::CompilationFailed(String::from(
            "Not all statements were parsed",
        )))?;
    output.extend(statics_syntax);
    Ok(output)
}

#[allow(clippy::unnecessary_wraps)]
fn compile_fn(
    name: &Rc<str>,
    args: &[Rc<str>],
    body: Vec<Statement>,
    statics: &BTreeMap<Rc<str>, Value>,
    constants: &BTreeMap<Rc<str>, Value>,
    function_signatures: &BTreeMap<Rc<str>, Vec<Rc<str>>>,
) -> Result<Vec<Either<Syntax, Statement>>, Error> {
    let mut out = Vec::new();
    let mut args_map = BTreeMap::new();
    for arg in args {
        let arg_label: Rc<str> = format!("_fn_{name}_arg_{arg}").into();
        out.push(Either::Left(Syntax::Label(arg_label.clone())));
        args_map.insert(arg.clone(), Value::Label(arg_label));
        out.push(Either::Left(Syntax::Reserve(1)));
    }
    let mut locals = BTreeMap::new();
    let mut locals_initial = BTreeMap::new();
    for statement in &body {
        if let Statement::Declaration(var, value) = statement {
            locals.insert(
                var.clone(),
                Value::Label(format!("_fn_{name}_local_{var}").into()),
            );
            locals_initial.insert(var.clone(), value.clone());
        }
    }
    let scope = Scope {
        globals: statics,
        parameters: &args_map,
        locals: &locals,
        functions: function_signatures,
        constants,
    };
    out.push(Either::Left(Syntax::Label(format!("_fn_{name}").into())));
    let mut rolling_hash = get_hash(&body);
    for stmt in body {
        rolling_hash = get_hash((&stmt, rolling_hash));
        out.extend(compile_statement(stmt, &scope, name, rolling_hash)?);
    }
    out.push(Either::Left(Syntax::Literal(0x0E40)));
    out.push(Either::Left(Syntax::Label(
        format!("_fn_{name}_ret_to").into(),
    )));
    out.push(Either::Left(Syntax::Literal(0xFFFF)));
    for (local, initial) in locals_initial {
        out.push(Either::Left(Syntax::Label(
            format!("_fn_{name}_local_{local}").into(),
        )));
        match initial {
            Some(Expression::Int(int)) => out.push(Either::Left(Syntax::Literal(int))),
            _ => out.push(Either::Left(Syntax::Reserve(1))),
        }
    }
    out.push(Either::Left(Syntax::Label(
        format!("_fn_{name}_ret").into(),
    )));
    out.push(Either::Left(Syntax::Reserve(1)));
    Ok(out)
}

#[allow(clippy::unnecessary_wraps, clippy::too_many_lines)]
fn compile_statement(
    stmt: Statement,
    scope: &Scope,
    func: &str,
    hash: u64,
) -> Result<Vec<Either<Syntax, Statement>>, Error> {
    match stmt {
        Statement::FunctionCall(name, args) if &*name == "yield" && args.is_empty() => {
            Ok(vec![Either::Left(Syntax::Instruction(Instruction::Yield))])
        }
        Statement::FunctionCall(func, args) => {
            compile_fn_call(func, args, scope, hash).map(|vec| vec.0)
        }
        Statement::Assignment(lhs, AssignOp::Eq, rhs) => {
            let (mut code, src) = value_from(rhs, scope, 1, hash)?;
            let Some(dst) = scope.get(&lhs) else { return Err(Error::InvalidIdentifier(lhs))};
            code.push(Either::Left(Syntax::Instruction(Instruction::Mov(
                src, dst,
            ))));
            Ok(code.into_iter().collect())
        }
        Statement::StarAssignment(lhs, rhs) => {
            let (mut code, dst) = value_from(lhs, scope, 1, hash)?;
            let (rest, src) = value_from(rhs, scope, 5, hash)?;
            code.extend(rest);
            match (src, dst) {
                (src, Item::Address(dst)) => {
                    code.push(Either::Left(Syntax::Instruction(Instruction::Ptrwrite(
                        src, dst,
                    ))));
                }
                (src, Item::Literal(dst)) => {
                    code.push(Either::Left(Syntax::Instruction(Instruction::Mov(
                        src, dst,
                    ))));
                } // (src, dst) => {
                  //     return Err(Error::CompilationFailed(format!(
                  //         "Couldn't apply format for *{dst} = {src}"
                  //     )))
                  // }
            }
            Ok(code)
        }
        Statement::Assignment(lhs, op, rhs) if crate::asm::MathOp::try_from(op).is_ok() => {
            let math_op = crate::asm::MathOp::try_from(op).unwrap();
            let (mut code, src) = value_from(rhs, scope, 1, hash)?;
            let Some(dst) = scope.get(&lhs) else { return Err(Error::InvalidIdentifier(lhs)) };
            code.push(Either::Left(Syntax::Instruction(Instruction::MathBinary(
                math_op, src, dst,
            ))));
            Ok(code)
        }
        Statement::Block(block_type, condition, body) => {
            let hash: Rc<str> = format!(
                "_{}_{:x}",
                block_type.as_ref(),
                get_hash((&condition, &body))
            )
            .into();
            let tail_hash: Rc<str> = format!("{hash}_tail").into();
            let mut output = Vec::new();
            let jcmp_hash = get_hash((&condition, &hash));
            if block_type == BlockType::While {
                output.push(Either::Left(Syntax::Instruction(Instruction::Jmp(
                    Item::Literal(Value::Label(tail_hash.clone())),
                ))));
            } else if block_type == BlockType::If {
                // !JMP #tail
                output.extend(compile_jcmp(
                    Expression::UnaryOp(UnaryOp::Not, Box::new(condition.clone())),
                    Item::Literal(Value::Label(tail_hash.clone())),
                    scope,
                    jcmp_hash,
                )?);
            }
            output.push(Either::Left(Syntax::Label(hash.clone())));
            for stmt in body {
                let hash = get_hash((&hash, &stmt));
                output.extend(compile_statement(stmt, scope, func, hash)?);
            }
            output.push(Either::Left(Syntax::Label(tail_hash)));
            if block_type == BlockType::While {
                output.extend(compile_jcmp(
                    condition,
                    Item::Literal(Value::Label(hash)),
                    scope,
                    jcmp_hash,
                )?);
            }
            Ok(output)
        }
        Statement::Declaration(var, expr) => {
            if expr.is_none()
                || expr
                    .as_ref()
                    .is_some_and(|expr| try_as_const(expr.clone(), scope).is_some())
            {
                return Ok(Vec::new());
            }
            let (mut code, value) = value_from(expr.unwrap(), scope, 1, hash)?;
            code.push(Either::Left(Syntax::Instruction(Instruction::Mov(
                value,
                scope.get(&var).unwrap(),
            ))));
            Ok(code)
        }
        Statement::Return(Some(value)) => {
            let (mut code, item) = value_from(value, scope, 1, hash)?;
            code.push(Either::Left(Syntax::Instruction(Instruction::Mov(
                item,
                Value::Label(format!("_fn_{func}_ret").into()),
            ))));
            code.push(Either::Left(Syntax::Instruction(Instruction::Jmp(
                Item::Address(Value::Label(format!("_fn_{func}_ret_to").into())),
            ))));
            Ok(code)
        }
        other => Ok(vec![Either::Right(other)]),
    }
}

fn compile_jcmp(
    cond: Expression,
    jmp: Item,
    scope: &Scope,
    hash: u64,
) -> Result<Vec<Either<Syntax, Statement>>, Error> {
    match cond {
        Expression::UnaryOp(UnaryOp::Not, inner) if matches!(&*inner, Expression::BinaryOp(..)) => {
            let Expression::BinaryOp(lhs, op, rhs) = *inner else { unreachable!() };
            match op {
                BinaryOp::And => compile_jcmp(
                    Expression::BinaryOp(
                        Box::new(Expression::UnaryOp(UnaryOp::Not, lhs)),
                        BinaryOp::Or,
                        Box::new(Expression::UnaryOp(UnaryOp::Not, rhs)),
                    ),
                    jmp,
                    scope,
                    hash,
                ),
                op => {
                    let cmp_op = crate::asm::CmpOp::try_from(op).map_err(|_| {
                        Error::CompilationFailed(format!(
                            "Couldn't turn `{op:?}` into a comparison"
                        ))
                    })?;
                    compile_jcmp(
                        Expression::BinaryOp(lhs, cmp_op.inverse().into(), rhs),
                        jmp,
                        scope,
                        hash,
                    )
                }
            }
        }
        Expression::UnaryOp(UnaryOp::Not, inner)
            if matches!(&*inner, Expression::UnaryOp(UnaryOp::Not, _)) =>
        {
            let Expression::UnaryOp(UnaryOp::Not, inner) = *inner else { unreachable!() };
            compile_jcmp(*inner, jmp, scope, hash)
        }
        Expression::BinaryOp(lhs, op, rhs)
            if crate::asm::CmpOp::try_from(op).is_ok()
                && value_from(*lhs.clone(), scope, 1, hash).is_ok()
                && value_from(*rhs.clone(), scope, 3, hash).is_ok() =>
        {
            let lhs = value_from(*lhs, scope, 1, hash)?;
            let rhs = value_from(*rhs, scope, 3, hash)?;
            let cmp_op = crate::asm::CmpOp::try_from(op).unwrap();
            let mut syn = lhs.0;
            syn.extend(rhs.0);
            let Item::Address(lhs) = lhs.1 else { return Err(Error::CompilationFailed(String::from("Can't compare literal on lhs")))};
            let rhs = rhs.1;
            syn.push(Either::Left(Syntax::Instruction(Instruction::JmpCmp(
                cmp_op, lhs, rhs, jmp,
            ))));
            Ok(syn)
        }
        Expression::BinaryOp(lhs, BinaryOp::And, rhs) => {
            let lhs_hash = get_hash((hash, &lhs));
            let rhs_hash = get_hash((hash, &rhs));
            let else_hash: Rc<str> = format!("_else_{hash:x}").into();
            let mut syn = Vec::new();
            syn.extend(compile_jcmp(
                Expression::UnaryOp(UnaryOp::Not, lhs),
                Item::Literal(Value::Label(else_hash.clone())),
                scope,
                lhs_hash,
            )?);
            syn.extend(compile_jcmp(*rhs, jmp, scope, rhs_hash)?);
            syn.push(Either::Left(Syntax::Label(else_hash)));
            Ok(syn)
        }
        Expression::BinaryOp(lhs, BinaryOp::Or, rhs) => {
            let lhs_hash = get_hash((hash, &lhs));
            let rhs_hash = get_hash((hash, &rhs));
            let mut syn = Vec::new();
            syn.extend(compile_jcmp(*lhs, jmp.clone(), scope, lhs_hash)?);
            syn.extend(compile_jcmp(*rhs, jmp, scope, rhs_hash)?);
            Ok(syn)
        }
        cond => Err(Error::InvalidExpression(cond)),
    }
}

fn value_from(
    expr: Expression,
    scope: &Scope,
    register: u16,
    hash: u64,
) -> Result<(Vec<Either<Syntax, Statement>>, Item), Error> {
    match expr {
        Expression::Int(i) => Ok((Vec::new(), Item::Literal(Value::Given(i)))),
        Expression::UnaryOp(UnaryOp::Deref, inner) if matches!(&*inner, Expression::Int(_)) => {
            let Expression::Int(int) = *inner else { unreachable!() };
            Ok((Vec::new(), Item::Address(Value::Given(int))))
        }
        Expression::UnaryOp(op @ (UnaryOp::Deref | UnaryOp::Address), inner)
            if matches!(&*inner, Expression::Ident(_)) =>
        {
            let Expression::Ident(var) = *inner else { unreachable!() };
            let var_value = scope.get(&var);
            let Some(var) = var_value else { return Err(Error::InvalidIdentifier(var))};
            if op == UnaryOp::Deref {
                Ok((
                    vec![Either::Left(Syntax::Instruction(Instruction::Ptrread(
                        var,
                        Value::Given(register),
                    )))],
                    Item::Address(Value::Given(register)),
                ))
            } else {
                Ok((Vec::new(), Item::Literal(var)))
            }
        }
        Expression::Ident(ident) => scope.get_constant(&ident).map_or_else(
            || {
                scope.get(&ident).map_or_else(
                    || Err(Error::InvalidIdentifier(ident)),
                    |val| Ok((Vec::new(), Item::Address(val))),
                )
            },
            |val| Ok((Vec::new(), Item::Literal(val))),
        ),
        Expression::FunctionCall(func, args) => compile_fn_call(func, args, scope, hash),
        expr => Err(Error::CompilationFailed(format!(
            "Couldn't get a value from expression `{expr:?}`"
        ))),
    }
}

fn compile_fn_call(
    func: Rc<str>,
    args: Vec<Expression>,
    scope: &Scope,
    hash: u64,
) -> Result<(Vec<Either<Syntax, Statement>>, Item), Error> {
    let Some(parameters) = scope.get_fn(&func) else { return Err(Error::InvalidIdentifier(func))};
    if parameters.len() != args.len() {
        return Err(Error::CompilationFailed(format!(
            "Expected {} argument(s) for {func}; got {}",
            parameters.len(),
            args.len()
        )));
    }
    let mut function_call = Vec::new();
    for (expr, arg) in args.into_iter().zip(parameters) {
        let (syn, value) = value_from(expr, scope, 1, hash)?;
        function_call.extend(syn);
        function_call.push(Either::Left(Syntax::Instruction(Instruction::Mov(
            value,
            Value::Label(format!("_fn_{func}_arg_{arg}").into()),
        ))));
    }
    let ret_hash: Rc<str> = format!("_call_{func}_ret_{hash:x}").into();
    function_call.push(Either::Left(Syntax::Instruction(Instruction::Mov(
        Item::Literal(Value::Label(ret_hash.clone())),
        Value::Label(format!("_fn_{func}_ret_to").into()),
    ))));
    function_call.push(Either::Left(Syntax::Instruction(Instruction::Jmp(
        Item::Literal(Value::Label(format!("_fn_{func}").into())),
    ))));
    function_call.push(Either::Left(Syntax::Label(ret_hash)));
    Ok((
        function_call,
        Item::Address(Value::Label(format!("_fn_{func}_ret").into())),
    ))
}

fn try_as_const(expr: Expression, scope: &Scope) -> Option<Value> {
    match expr {
        Expression::Int(int) => Some(Value::Given(int)),
        Expression::Ident(ident) => scope.get_constant(&ident),
        Expression::BinaryOp(lhs, op, rhs) => {
            let Some(Value::Given(lhs)) = try_as_const(*lhs, scope) else { return None };
            let Some(Value::Given(rhs)) = try_as_const(*rhs, scope) else { return None };
            Some(Value::Given(match op {
                BinaryOp::Add => lhs.wrapping_add(rhs),
                BinaryOp::Sub => lhs.wrapping_sub(rhs),
                BinaryOp::Mul => lhs.wrapping_mul(rhs),
                BinaryOp::Eq => u16::from(lhs == rhs),
                BinaryOp::Ne => u16::from(lhs != rhs),
                BinaryOp::Lt => u16::from(lhs < rhs),
                BinaryOp::Le => u16::from(lhs <= rhs),
                BinaryOp::Gt => u16::from(lhs > rhs),
                BinaryOp::Ge => u16::from(lhs >= rhs),
                BinaryOp::BitAnd => lhs & rhs,
                BinaryOp::BitOr => lhs | rhs,
                BinaryOp::BitXor => lhs ^ rhs,
                BinaryOp::Shl => lhs.wrapping_shl(rhs.into()),
                BinaryOp::Shr => lhs.wrapping_shr(rhs.into()),
                _ => return None,
            }))
        }
        _ => None,
    }
}
