mod lexer;

use std::env;

use crate::lexer::TokenKind;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid arguments length");
        std::process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let tokens = match lexer::lex(args.get(1).unwrap()) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lex Error: {}", e);
            std::process::exit(1);
        }
    };

    print!("  mov rax, ");
    for token in tokens.into_iter() {
        match token.value {
            TokenKind::Plus => {
                print!("  add rax, ");
            }
            TokenKind::Minus => {
                print!("  sub rax, ")
            }
            TokenKind::Number(n) => {
                println!("{}", n);
            }
        }
    }
    println!("  ret");
}

