use std::{
    io::{stdin, Read},
    str::FromStr,
};

fn main() {
    truncate_line();
    let mut nums = input_all_splitted_as::<i32>();
    nums.sort();

    let output = nums
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{output}");
}

fn truncate_line() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}

fn input_all_splitted_as<T>() -> Vec<T>
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
