use std::{
    io::{stdin, Read},
    str::FromStr,
};

fn main() {
    truncate_line();

    let mut difficulties = input_all_splitted_as::<i32>();
    difficulties.sort();

    let except_count = (difficulties.len() as f64 * 0.15).round() as usize;

    let difficulties = &difficulties[except_count..difficulties.len() - except_count];
    let difficulty = difficulties.iter().sum::<i32>() as f64 / difficulties.len() as f64;
    let difficulty = difficulty.round() as i32;
    println!("{difficulty}");
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
