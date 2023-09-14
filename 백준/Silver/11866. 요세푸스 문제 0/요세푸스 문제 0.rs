use std::collections::VecDeque;
use std::io::stdin;
use std::str::FromStr;

fn main() {
    let input = input_line_as::<usize>();
    let n = input[0];
    let k = input[1];

    let ans = josephus_sequence(n, k)
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    println!("<{ans}>");
}

fn josephus_sequence(n: usize, k: usize) -> Vec<usize> {
    let mut circle: VecDeque<usize> = (1..=n).collect();
    let mut ans = Vec::new();

    let mut at = 0;

    while !circle.is_empty() {
        at += k - 1;
        at %= circle.len();
        let removed = circle.remove(at).unwrap();
        ans.push(removed);
    }
    ans
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