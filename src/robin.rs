use self::types::TopLevelSyntax;

mod lexer;
mod parser;
mod types;
mod compiler;

#[derive(Debug)]
pub enum Error {
    ParseError(parser::ParseError),
    LexError(lexer::LexError),
}

impl From<parser::ParseError> for Error {
    fn from(value: parser::ParseError) -> Self {
        Self::ParseError(value)
    }
}

impl From<lexer::LexError> for Error {
    fn from(value: lexer::LexError) -> Self {
        Self::LexError(value)
    }
}

pub fn pipe(src: &str) -> Result<Vec<TopLevelSyntax>, Error> {
    parser::parse(lexer::lex(src)?).map_err(Into::into)
}
