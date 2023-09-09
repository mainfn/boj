use std::io::{stdin, Read};
use std::str::FromStr;

fn main() {
    truncate_line();
    let lines = input_lines();

    for line in lines {
        let line = line.split_ascii_whitespace().collect::<Vec<_>>();
        let n = line[0].trim().parse::<usize>().unwrap();
        let str = line[1]
            .chars()
            .into_iter()
            .map(|c| c.to_string().repeat(n))
            .collect::<String>();

        println!("{str}");
    }
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

fn input_lines() -> Vec<String> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    buf.trim()
        .split('\n')
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}
