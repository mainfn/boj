use std::io::{Read, stdin};
use std::str::FromStr;

fn main() {
    let input = input_until_eof_as::<i32>();
    let hh = input[0];
    let mm = input[1];
    let after = input[2];

    let minutes = (hh * 60 + mm + after) % (60 * 24);

    let hh = minutes / 60;
    let mm = minutes % 60;
    println!("{hh} {mm}");
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