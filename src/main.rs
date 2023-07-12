use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid arguments length");
        std::process::exit(1);
    }

    // first digit
    let (digit, mut remaining) = split_digit(args[1].as_str());

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", digit);
    while !remaining.is_empty() {
        let ch1 = &remaining[..1];
        if "+" == ch1 {
            remaining = &remaining[1..];
            let (digit, rem) = split_digit(remaining);
            remaining = rem;
            println!("  add rax, {}", digit);
            continue;
        }
        if "-" == ch1 {
            remaining = &remaining[1..];
            let (digit, rem) = split_digit(remaining);
            remaining = rem;
            println!("  sub rax, {}", digit);
            continue;
        }
    }
    println!("  ret");
}

fn split_digit(str: &str) -> (&str, &str) {
    let index = str.find(|c| !char::is_numeric(c)).unwrap_or(str.len());
    str.split_at(index)
}


#[test]
fn split_digit_test() {
    assert_eq!(split_digit("123"), ("123", ""));
    assert_eq!(split_digit("123abc"), ("123", "abc"));

    assert_eq!(split_digit(""), ("", ""));
    assert_eq!(split_digit("abc"), ("", "abc"));
    assert_eq!(split_digit("1+2"), ("1", "+2"));
}
