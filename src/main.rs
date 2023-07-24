use std::env;

use rcc::error::{Error, LexError, LexKind};
use rcc::lexer::TokenKind;
use rcc::{lexer, Annot};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid arguments length");
        std::process::exit(1);
    }

    let line = args.get(1).unwrap();
    let tokens = match lexer::lex(line) {
        Ok(t) => t,
        Err(e) => {
            let error = Error::from(e);
            error.show_diagnostic(&line);
            std::process::exit(1);
        }
    };

    if tokens.len() == 0 {
        eprintln!("no input value");
        std::process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    match expect_number(tokens.first().unwrap()) {
        Ok(n) => {
            println!("  mov rax, {}", n);
        }
        Err(e) => {
            let error = Error::from(e);
            error.show_diagnostic(&line);
            std::process::exit(1);
        }
    }

    let mut iter = tokens.into_iter().skip(1).peekable();
    while let Some(token) = iter.next() {
        match iter.peek() {
            Some(peek) => {
                if consume(TokenKind::Plus, &token) {
                    match expect_number(peek) {
                        Ok(n) => {
                            println!("  add rax, {}", n);
                            //iter.next();
                            continue;
                        }
                        Err(e) => {
                            let error = Error::from(e);
                            error.show_diagnostic(&line);
                            std::process::exit(1);
                        }
                    }
                }
                if consume(TokenKind::Minus, &token) {
                    match expect_number(peek) {
                        Ok(n) => {
                            println!("  sub rax, {}", n);
                            //iter.next();
                            continue;
                        }
                        Err(e) => {
                            let error = Error::from(e);
                            error.show_diagnostic(&line);
                            std::process::exit(1);
                        }
                    }
                }
            }
            None => {
                match expect_number(&token) {
                    Err(e) => {
                        let error = Error::from(e);
                        error.show_diagnostic(&line);
                        std::process::exit(1);
                    }
                    _ => break
                }
            }
        }
    }

    println!("  ret");
}

fn expect_number(token: &Annot<TokenKind>) -> Result<u64, LexError> {
    match token.value {
        TokenKind::Number(n) => {
            return Ok(n);
        }
        TokenKind::Plus => {
            return Err(LexError::new(LexKind::InvalidChar('+'), token.loc));
        }
        TokenKind::Minus => {
            return Err(LexError::new(LexKind::InvalidChar('-'), token.loc));
        }
    }
}

fn consume(kind: TokenKind, token: &Annot<TokenKind>) -> bool {
    kind == token.value
}
