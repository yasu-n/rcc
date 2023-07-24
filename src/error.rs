use std::error::Error as StdError;

use crate::Loc;
use crate::Annot;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexKind {
    InvalidChar(char),
    Eof,
}

pub fn print_error(input: &str, loc: Loc) {
    eprintln!("{}", input);
    eprintln!("{}{}", " ".repeat(loc.0), "^".repeat(loc.1 - loc.0));
}

pub type LexError = Annot<LexKind>;

impl StdError for LexError {}

impl LexError {
    pub fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexKind::InvalidChar(c), loc)
    }
    pub fn eof(loc: Loc) -> Self {
        LexError::new(LexKind::Eof, loc)
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::LexKind::*;
        let loc = &self.loc;
        match self.value {
            InvalidChar(c) => write!(f, "{}: invalid char: {}", loc, c),
            Eof => write!(f, "End of file"),
        }
    }
}

impl From<LexError> for Error {
    fn from(e: LexError) -> Self {
        Error::Lexer(e)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use self::Error::*; 
        match self {
            Lexer(lex) => Some(lex)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    Lexer(LexError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error")
    }
}

impl Error {
    pub fn show_diagnostic(&self, input: &str) {
        use self::Error::*;
        let (e, loc): (&dyn StdError, Loc) = match self {
            Lexer(e) => (e, e.loc.clone()),
        };

        eprintln!("{:?}", e);
        print_error(input, loc);
    }
}
