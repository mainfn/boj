use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let count = buf
        .trim()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .count();

    println!("{count}");
}