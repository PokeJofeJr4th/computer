use std::{collections::BTreeMap, rc::Rc};

use crate::{
    asm::{Instruction, Item, Syntax, Value},
    robin::types::UnaryOp,
    utils::{get_hash, Either},
};

use super::types::{Expression, Statement, TopLevelSyntax};

struct Scope<'a> {
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
    let mut statics_syntax = Vec::new();
    for syn in src {
        match syn {
            TopLevelSyntax::Function(name, args, body) => {
                function_signatures.insert(name.clone(), args.clone());
                function_bodies.insert(name, (args, body));
            }
            TopLevelSyntax::Constant(name, Expression::String(str)) => {
                statics.insert(name.clone(), Value::Label(name.clone()));
                statics_syntax.push(Syntax::Label(name));
                statics_syntax.extend(crate::asm::string_literal(&str));
            }
            TopLevelSyntax::Constant(..) => return Err(Error::InvalidSyntax(syn)),
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
        &function_signatures,
    )?);
    for (name, (args, body)) in function_bodies {
        output.extend(compile_fn(
            &name,
            &args,
            body,
            &statics,
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
    let locals = BTreeMap::new();
    let scope = Scope {
        globals: statics,
        parameters: &args_map,
        locals: &locals,
        functions: function_signatures,
    };
    out.push(Either::Left(Syntax::Label(
        format!("_fn_{name}_ret").into(),
    )));
    out.push(Either::Left(Syntax::Reserve(1)));
    out.push(Either::Left(Syntax::Label(format!("_fn_{name}").into())));
    for stmt in body {
        let hash = get_hash(&stmt);
        out.extend(compile_statement(stmt, &scope, hash)?);
    }
    out.push(Either::Left(Syntax::Literal(0x0E40)));
    out.push(Either::Left(Syntax::Label(
        format!("_fn_{name}_ret_to").into(),
    )));
    out.push(Either::Left(Syntax::Literal(0xFFFF)));
    Ok(out)
}

#[allow(clippy::unnecessary_wraps)]
fn compile_statement(
    stmt: Statement,
    scope: &Scope,
    hash: u64,
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
                    let Expression::UnaryOp(UnaryOp::Deref, deref) = &args[0] else { break 'guard false };
                    matches!(&**deref, Expression::Ident(_))
                } =>
        {
            let Expression::UnaryOp(UnaryOp::Deref, ref deref) = args[0] else { unreachable!()};
            let Expression::Ident(variable) = &**deref else { unreachable!() };
            let Some(variable) = scope.get(variable) else { return Err(Error::InvalidIdentifier(variable.clone()))};
            let Expression::Int(src) = args[1] else {unreachable!()};
            Ok(vec![Either::Left(Syntax::Instruction(
                Instruction::Ptrread(variable, Value::Given(src)),
            ))])
        }
        Statement::FunctionCall(func, args) => {
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
                let (syn, value) = value_from(expr, scope, 1)?;
                function_call.extend(syn);
                function_call.push(Syntax::Instruction(Instruction::Mov(
                    value,
                    Value::Label(format!("_fn_{func}_arg_{arg}").into()),
                )));
            }
            function_call.push(Syntax::Instruction(Instruction::Mov(
                Item::Literal(Value::Label(format!("_call_{func}_ret_{hash:x}").into())),
                Value::Label(format!("_fn_{func}_ret_to").into()),
            )));
            function_call.push(Syntax::Instruction(Instruction::Jmp(Item::Literal(
                Value::Label(format!("_fn_{func}").into()),
            ))));
            function_call.push(Syntax::Label(format!("_call_{func}_ret_{hash:x}").into()));
            Ok(function_call.into_iter().map(Either::Left).collect())
        }
        Statement::Assignment(lhs, op, rhs) if crate::asm::MathOp::try_from(op).is_ok() => {
            let math_op = crate::asm::MathOp::try_from(op).unwrap();
            let (mut code, src) = value_from(rhs, scope, 0)?;
            let Some(dst) = scope.get(&lhs) else { return Err(Error::InvalidIdentifier(lhs)) };
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
                let hash = get_hash(&stmt);
                output.extend(compile_statement(stmt, scope, hash)?);
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
        Expression::UnaryOp(op @ (UnaryOp::Deref | UnaryOp::Address), inner)
            if matches!(&*inner, Expression::Ident(_)) =>
        {
            let Expression::Ident(var) = *inner else { unreachable!() };
            let var_value = scope.get(&var);
            let Some(var) = var_value else { return Err(Error::InvalidIdentifier(var))};
            if op == UnaryOp::Deref {
                Ok((
                    vec![Syntax::Instruction(Instruction::Ptrread(
                        var,
                        Value::Given(register),
                    ))],
                    Item::Address(Value::Given(register)),
                ))
            } else {
                Ok((Vec::new(), Item::Literal(var)))
            }
        }
        Expression::Ident(ident) => scope.get(&ident).map_or_else(
            || Err(Error::InvalidIdentifier(ident)),
            |val| Ok((Vec::new(), Item::Address(val))),
        ),
        expr => Err(Error::CompilationFailed(format!(
            "Couldn't get a value from expression `{expr:?}`"
        ))),
    }
}
