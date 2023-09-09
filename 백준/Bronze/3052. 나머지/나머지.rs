use std::{
    collections::HashSet,
    io::{stdin, Read},
    str::FromStr,
};

fn main() {
    let nums = input_vec::<i32>();

    let unique_count = nums
        .iter()
        .map(|n| n % 42)
        .collect::<HashSet<_>>()
        .iter()
        .count();

    println!("{unique_count}");
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
