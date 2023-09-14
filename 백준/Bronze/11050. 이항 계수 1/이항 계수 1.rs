use std::io::stdin;
use std::str::FromStr;

fn main() {
    let nums = input_line_as::<i64>();
    let n = nums[0];
    let k = nums[1];

    println!("{}", binomial_coefficient(n, k));
}

fn binomial_coefficient(n: i64, k: i64) -> i64 {
    facto(n) / (facto(k) * facto(n - k))
}

fn facto(n: i64) -> i64 {
    if n <= 1 {
        1
    } else {
        n * facto(n - 1)
    }
}

fn input_line_as<T>() -> Vec<T>
    where
        T: Eq + Ord + Copy + FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<T>>()
}