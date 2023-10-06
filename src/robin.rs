use crate::asm::Syntax;

mod compiler;
mod lexer;
mod parser;
mod types;

#[derive(Debug)]
pub enum Error {
    Parser(parser::ParseError),
    Lexer(lexer::LexError),
    Compiler(compiler::Error),
}

impl From<parser::ParseError> for Error {
    fn from(value: parser::ParseError) -> Self {
        Self::Parser(value)
    }
}

impl From<lexer::LexError> for Error {
    fn from(value: lexer::LexError) -> Self {
        Self::Lexer(value)
    }
}

impl From<compiler::Error> for Error {
    fn from(value: compiler::Error) -> Self {
        Self::Compiler(value)
    }
}

pub fn pipe(src: &str) -> Result<Vec<Syntax>, Error> {
    let syntax = parser::parse(lexer::lex(src)?).map_err(Error::from)?;
    compiler::compile(syntax).map_err(Error::from)
}
