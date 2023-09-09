use std::{
    io::{stdin, Read},
    str::FromStr,
};

fn main() {
    truncate_line();
    let mut nums = input_vec::<i32>();
    nums.sort();

    let min = nums.first().unwrap();
    let max = nums.last().unwrap();

    println!("{min} {max}");
}

fn truncate_line() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
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
