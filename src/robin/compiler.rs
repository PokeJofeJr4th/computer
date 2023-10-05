use crate::asm::Syntax;

use super::types::TopLevelSyntax;

#[derive(Debug)]
pub enum CompilerError {
    InvalidSyntax(TopLevelSyntax)
}

pub fn compile(src: Vec<TopLevelSyntax>) -> Result<Vec<Syntax>, CompilerError> {
    todo!()
}
