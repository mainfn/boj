use std::collections::VecDeque;
use std::io::{Read, stdin};
use std::str::FromStr;

fn main() {
    let mut mp: [[usize; 15]; 15] = [[0; 15]; 15];
    for i in 1..15 {
        mp[0][i] = i;
    }
    for floor in 1..15 {
        for i in 1..15 {
            mp[floor][i] = mp[floor - 1][1..=i].iter().sum::<usize>();
        }
    }

    let mut nums = input_until_eof_as::<usize>();
    nums.pop_front();


    while !nums.is_empty() {
        let k = nums.pop_front().unwrap();
        let n = nums.pop_front().unwrap();

        println!("{}", mp[k][n]);
    }
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