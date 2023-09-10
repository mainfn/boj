use std::{io::stdin, str::FromStr};

fn main() {
    let n = input_line_splitted_as::<i32>()[0];

    let mut constructor = 0;

    for i in 1..=1_000_000 {
        if n == destruct(i) {
            constructor = i;
            break;
        }
    }

    println!("{constructor}");
}

fn destruct(mut i: i32) -> i32 {
    let mut result = i;

    while i > 0 {
        result += i % 10;

        i /= 10;
    }
    result
}

fn input_line_splitted_as<T>() -> Vec<T>
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
