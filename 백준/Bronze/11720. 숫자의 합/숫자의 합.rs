use std::{
    io::{stdin, Read},
    str::FromStr,
};

fn main() {
    truncate_line();
    let line = input_line();

    let sum = line
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .sum::<i32>();

    println!("{sum}");
}

fn truncate_line() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}

fn input_line() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}
