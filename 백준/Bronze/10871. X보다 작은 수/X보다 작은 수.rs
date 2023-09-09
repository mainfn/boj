use std::{
    io::{stdin, Read},
    str::FromStr,
};

fn main() {
    let input = input_vec::<i32>();
    let x = input[1];

    let ans = input[2..]
        .iter()
        .filter(|&&n| n < x)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ");

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
