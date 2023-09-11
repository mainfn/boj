use std::{collections::HashMap, fmt::Write, io::stdin, str::FromStr};

fn main() {
    truncate_line();
    let numbers = input_line_splitted_as::<i32>();
    let mut mp = HashMap::<i32, bool>::new();
    for number in numbers.into_iter() {
        mp.insert(number, true);
    }

    truncate_line();
    let guesses = input_line_splitted_as::<i32>();

    let mut output = String::new();
    for guess in guesses {
        let result = if mp.contains_key(&guess) { 1 } else { 0 };
        writeln!(&mut output, "{}", result).unwrap();
    }

    print!("{output}");
}

fn truncate_line() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
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
