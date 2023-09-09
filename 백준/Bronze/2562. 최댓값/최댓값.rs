use std::io::{stdin, Read};
use std::str::FromStr;

fn main() {
    let nums = input_vec::<usize>();

    let mut max_number = usize::MIN;
    let mut at = usize::MIN;

    for (i, &n) in nums.iter().enumerate() {
        if n > max_number {
            max_number = n;
            at = i + 1;
        }
    }

    println!("{max_number}\n{at}");
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
