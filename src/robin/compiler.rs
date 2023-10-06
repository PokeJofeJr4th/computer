use std::{collections::BTreeMap, rc::Rc};

use crate::{
    asm::{Instruction, Item, Syntax, Value},
    utils::{get_hash, Either},
};

use super::types::{Expression, Statement, TopLevelSyntax};

struct Scope<'a> {
    globals: &'a BTreeMap<Rc<str>, Value>,
    parameters: &'a BTreeMap<Rc<str>, Value>,
    locals: &'a BTreeMap<Rc<str>, Value>,
}

impl<'a> Scope<'a> {
    pub fn get(&self, ident: &str) -> Option<&Value> {
        if let Some(local) = self.locals.get(ident) {
            return Some(local);
        };
        if let Some(param) = self.parameters.get(ident) {
            return Some(param);
        };
        if let Some(global) = self.globals.get(ident) {
            return Some(global);
        };
        None
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
    for syn in src {
        match syn {
            TopLevelSyntax::Function(name, args, body) => {
                function_signatures.insert(name.clone(), args.clone());
                function_bodies.insert(name, (args, body));
            }
            TopLevelSyntax::Constant(..) => return Err(Error::InvalidSyntax(syn)),
        }
    }
    let mut output = Vec::new();
    for (name, (args, body)) in function_bodies {
        output.extend(compile_fn(&name, &args, body, &statics)?);
    }
    println!("{output:?}");
    output
        .into_iter()
        .map(Either::left)
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::CompilationFailed(String::from(
            "Not all statements were parsed",
        )))
}

#[allow(clippy::unnecessary_wraps)]
fn compile_fn(
    name: &Rc<str>,
    args: &[Rc<str>],
    body: Vec<Statement>,
    statics: &BTreeMap<Rc<str>, Value>,
) -> Result<Vec<Either<Syntax, Statement>>, Error> {
    let mut out = Vec::new();
    let mut args_map = BTreeMap::new();
    for arg in args {
        let arg_label: Rc<str> = format!("_fn_{name}_arg_{arg}").into();
        out.push(Either::Left(Syntax::Label(arg_label.clone())));
        args_map.insert(arg.clone(), Value::Label(arg_label));
        out.push(Either::Left(Syntax::Reserve(1)));
    }
    let locals = BTreeMap::new();
    let scope = Scope {
        globals: statics,
        parameters: &args_map,
        locals: &locals,
    };
    out.push(Either::Left(Syntax::Label(format!("_fn_{name}").into())));
    for stmt in body {
        out.extend(compile_statement(stmt, &scope)?);
    }
    Ok(out)
}

#[allow(clippy::unnecessary_wraps)]
fn compile_statement(
    stmt: Statement,
    scope: &Scope,
) -> Result<Vec<Either<Syntax, Statement>>, Error> {
    match stmt {
        Statement::FunctionCall(name, args) if &*name == "yield" && args.is_empty() => {
            Ok(vec![Either::Left(Syntax::Instruction(Instruction::Yield))])
        }
        Statement::FunctionCall(name, args)
            if &*name == "write"
                && matches!(&args[..], &[Expression::Int(_), Expression::Int(_)]) =>
        {
            let Expression::Int(src) = args[0] else { unreachable!() };
            let Expression::Int(dst) = args[1] else { unreachable!() };
            Ok(vec![Either::Left(Syntax::Instruction(Instruction::Mov(
                Item::Literal(Value::Given(src)),
                Value::Given(dst),
            )))])
        }
        Statement::FunctionCall(name, args)
            if &*name == "write"
                && args.len() == 2
                && matches!(&args[1], Expression::Int(_))
                && 'guard: {
                    let Expression::Deref(deref) = &args[0] else { break 'guard false };
                    matches!(&**deref, Expression::Ident(_))
                } =>
        {
            let Expression::Deref(ref deref) = args[0] else { unreachable!()};
            let Expression::Ident(variable) = &**deref else { unreachable!() };
            let Some(variable) = scope.get(variable) else { return Err(Error::InvalidIdentifier(variable.clone()))};
            let Expression::Int(src) = args[1] else {unreachable!()};
            Ok(vec![Either::Left(Syntax::Instruction(
                Instruction::Ptrwrite(Item::Literal(Value::Given(src)), variable.clone()),
            ))])
        }
        Statement::Assignment(lhs, op, rhs) if crate::asm::MathOp::try_from(op).is_ok() => {
            let math_op = crate::asm::MathOp::try_from(op).unwrap();
            let (mut code, src) = value_from(rhs, scope, 0)?;
            let dst = match scope.get(&lhs) {
                Some(val) => val.clone(),
                None => return Err(Error::InvalidIdentifier(lhs)),
            };
            code.push(Syntax::Instruction(Instruction::MathBinary(
                math_op, src, dst,
            )));
            Ok(code.into_iter().map(Either::Left).collect())
        }
        Statement::While(condition, body) => {
            let hash: Rc<str> = format!("_while_{:x}", get_hash((&condition, &body))).into();
            let tail_hash: Rc<str> = format!("{hash}_tail").into();
            let mut output = Vec::new();
            output.push(Either::Left(Syntax::Instruction(Instruction::Jmp(
                Item::Literal(Value::Label(tail_hash.clone())),
            ))));
            output.push(Either::Left(Syntax::Label(hash.clone())));
            for stmt in body {
                output.extend(compile_statement(stmt, scope)?);
            }
            output.push(Either::Left(Syntax::Label(tail_hash)));
            output.extend(
                compile_jcmp(condition, Item::Literal(Value::Label(hash)), scope)?
                    .into_iter()
                    .map(Either::Left),
            );
            Ok(output)
        }
        other => Ok(vec![Either::Right(other)]),
    }
}

fn compile_jcmp(cond: Expression, jmp: Item, scope: &Scope) -> Result<Vec<Syntax>, Error> {
    match cond {
        Expression::BinaryOp(lhs, op, rhs)
            if crate::asm::CmpOp::try_from(op).is_ok()
                && value_from(*lhs.clone(), scope, 0).is_ok()
                && value_from(*rhs.clone(), scope, 1).is_ok() =>
        {
            let lhs = value_from(*lhs, scope, 0)?;
            let rhs = value_from(*rhs, scope, 1)?;
            let cmp_op = crate::asm::CmpOp::try_from(op).unwrap();
            let mut syn = lhs.0;
            syn.extend(rhs.0);
            let Item::Address(lhs) = lhs.1 else { return Err(Error::CompilationFailed(String::from("Can't compare literal on lhs")))};
            let rhs = rhs.1;
            syn.push(Syntax::Instruction(Instruction::JmpCmp(
                cmp_op, lhs, rhs, jmp,
            )));
            Ok(syn)
        }
        cond => Err(Error::InvalidExpression(cond)),
    }
}

fn value_from(
    expr: Expression,
    scope: &Scope,
    register: u16,
) -> Result<(Vec<Syntax>, Item), Error> {
    match expr {
        Expression::Int(i) => Ok((Vec::new(), Item::Literal(Value::Given(i)))),
        Expression::Deref(inner) if matches!(&*inner, Expression::Ident(_)) => {
            let Expression::Ident(var) = *inner else { unreachable!() };
            let var_value = scope.get(&var);
            let Some(var) = var_value else { return Err(Error::InvalidIdentifier(var))};
            Ok((
                vec![Syntax::Instruction(Instruction::Ptrread(
                    var.clone(),
                    Value::Given(register),
                ))],
                Item::Address(Value::Given(register)),
            ))
        }
        Expression::Ident(ident) => scope.get(&ident).map_or_else(
            || Err(Error::InvalidIdentifier(ident)),
            |val| Ok((Vec::new(), Item::Address(val.clone()))),
        ),
        expr => Err(Error::CompilationFailed(format!(
            "Couldn't get a value from expression `{expr:?}`"
        ))),
    }
}

// while (condition) {
//     do thingk
// }

// jmp #tail
// :while
// do thinkgk
// :tail
// jcmp :while
// :after
