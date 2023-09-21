use std::collections::VecDeque;
use std::io::{Read, stdin};
use std::str::FromStr;

fn main() {
    let mut nums = input_until_eof_as::<usize>();
    nums.pop_front();

    let mut ans = Vec::<usize>::new();

    for num in nums.into_iter() {
        if num == 0 {
            ans.pop();
        } else {
            ans.push(num);
        }
    }

    let sum = ans.iter().sum::<usize>();
    println!("{sum}");
}

fn input_until_eof_as<T>() -> VecDeque<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    buf.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<VecDeque<_>>()
}