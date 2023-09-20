use std::io::{Read, stdin};
use std::str::FromStr;

fn main() {
    let input = input_until_eof_as::<isize>();

    let x = input[0];
    let y = input[1];

    let n = if x > 0 && y > 0 {
        1
    } else if x < 0 && y > 0 {
        2
    } else if x < 0 && y < 0 {
        3
    } else {
        4
    };

    println!("{n}");
}

fn input_until_eof_as<T>() -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    buf.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}