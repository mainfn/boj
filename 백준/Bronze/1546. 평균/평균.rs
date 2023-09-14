use std::io::stdin;
use std::str::FromStr;

fn main() {
    truncate_line();

    let mut nums = input_line_as::<i32>();
    nums.sort();
    let m = *nums.last().unwrap();

    let avg = nums.iter()
        .map(|&num| num as f64 / m as f64 * 100.0)
        .sum::<f64>() / nums.len() as f64;

    println!("{avg}");
}

fn truncate_line() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
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
        .collect::<Vec<_>>()
}