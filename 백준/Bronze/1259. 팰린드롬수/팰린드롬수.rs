use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    buf.split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .filter(|n| *n > 0)
        .map(|n| if is_palindrome(n) { "yes" } else { "no" })
        .for_each(|s| println!("{s}"));
}

fn is_palindrome(number: i32) -> bool {
    let str = number.to_string();
    let rev_str: String = str.chars().rev().collect();

    return str == rev_str;
}