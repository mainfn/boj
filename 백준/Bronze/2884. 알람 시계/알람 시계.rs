use std::{
    io::{stdin, Read},
    str::FromStr,
};

fn main() {
    let input = input_vec::<i32>();

    let hh = input[0];
    let mm = input[1];

    let total_mm = hh * 60 + mm - 45;
    let day_mm = 60 * 24;
    let total_mm = (total_mm + day_mm) % day_mm;

    let hh = total_mm / 60;
    let mm = total_mm % 60;
    println!("{hh} {mm}");
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
