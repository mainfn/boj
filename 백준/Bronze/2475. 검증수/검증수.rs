use std::io::{stdin, Read};
use std::str::FromStr;

fn main() {
    let nums = input_vec::<usize>();
    let ans = nums.iter().map(|n| n * n).sum::<usize>() % 10;

    println!("{ans}");
}

fn input_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    buf.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<T>>()
}
