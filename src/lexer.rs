use crate::Loc;
use crate::Annot;
use crate::error::LexError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Plus,
    Minus,
    Number(u64),
}

pub type Token = Annot<TokenKind>;

impl Token {
    pub fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }
    pub fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }
    pub fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let input = input.as_bytes();
    let mut pos = 0;

    macro_rules! lex_token {
        ($lexer:expr) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok);
            pos = p;
        }};
    }

    while pos < input.len() {
        match input[pos] {
            b'0'..=b'9' => lex_token!(lex_number(input, pos)),
            b'+' => lex_token!(lex_plus(input, pos)),
            b'-' => lex_token!(lex_minus(input, pos)),
            b' ' | b'\n' | b'\t' => {
                let ((), p) = skip_space(input, pos)?;
                pos = p;
            }
            b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1)))
        }
    }

    Ok(tokens)
}

fn consume_byte(input: &[u8], pos: usize, b: u8) -> Result<(u8, usize), LexError> {
    if input.len() < pos {
        return Err(LexError::eof(Loc(pos, pos + 1)));
    }
    if input[pos] != b {
        return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1)));
    }
    Ok((b, pos + 1))
}

fn recognize_many(input: &[u8], mut pos: usize, mut f: impl FnMut(u8) -> bool) -> usize {
    while pos < input.len() && f(input[pos]) {
        pos += 1;
    }
    pos
}

fn lex_number(input: &[u8], pos: usize) -> Result<(Token, usize), LexError> {
    use std::str::from_utf8;
    let start = pos;
    let end = recognize_many(input, pos, |b| b"1234567890".contains(&b));
    let n = from_utf8(&input[start..end]).unwrap().parse().unwrap();
    Ok((Token::number(n, Loc(start, end)), end))
}

fn lex_plus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'+').map(|(_, end)| (Token::plus(Loc(start, end)), end))
}

fn lex_minus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'-').map(|(_, end)| (Token::minus(Loc(start, end)), end))
}

fn skip_space(input: &[u8], pos: usize) -> Result<((), usize), LexError> {
    let pos = recognize_many(input, pos, |b| b" \n\t".contains(&b));
    Ok(((), pos))
}

#[test]
fn test_lex() {
    let input = "42";
    let tokens = lex(input);
    let expected = vec![Token::number(42, Loc(0, 2))];
    assert_eq!(Ok(expected), tokens);


    let input = " 12 + 34 - 5 ";
    let tokens = lex(input);

    let expected = vec![
        Token::number(12, Loc(1, 3)),
        Token::plus(Loc(4, 5)),
        Token::number(34, Loc(6, 8)),
        Token::minus(Loc(9, 10)),
        Token::number(5, Loc(11, 12)),
    ];
    assert_eq!(Ok(expected), tokens);

    let input = " 3 - 4 - ";
    let tokens = lex(input);

    let expected = vec![
        Token::number(3, Loc(1, 2)),
        Token::minus(Loc(3, 4)),
        Token::number(4, Loc(5, 6)),
        Token::minus(Loc(7, 8)),
    ];
    assert_eq!(Ok(expected), tokens);
}
