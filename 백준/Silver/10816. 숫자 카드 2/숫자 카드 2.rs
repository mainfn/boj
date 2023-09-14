use fmt::Write;
use std::collections::HashMap;
use std::fmt;
use std::io::stdin;
use std::str::FromStr;

fn main() {
    truncate_line();
    let mut my_cards = input_line_as::<i32>();
    truncate_line();
    let mut cards = input_line_as::<i32>();
    let mut card_counts = HashMap::<i32, i32>::new();

    let mut card_counts = my_cards.iter()
        .fold(&mut card_counts, |acc, card| {
            acc.entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            acc
        });

    let mut output = String::new();
    for card in cards {
        write!(&mut output, "{} ", &card_counts.get(&card).unwrap_or(&0)).unwrap();
    }

    print!("{output}");
}

fn truncate_line() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}

fn input_line_as<T>() -> Vec<T>
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