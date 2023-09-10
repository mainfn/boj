use std::{collections::VecDeque, io::stdin, str::FromStr};

fn main() {
    let n = input_line_splitted_as::<i32>()[0];

    let mut nums = (1..=n).rev().collect::<VecDeque<_>>();

    while nums.len() > 1 {
        nums.pop_back();
        let popped = nums.pop_back().unwrap();
        nums.push_front(popped);
    }

    println!("{}", nums[0]);
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
