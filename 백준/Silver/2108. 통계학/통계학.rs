use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, stdin};
use std::str::FromStr;

fn main() {
    truncate_line();
    let mut nums = input_vec::<i32>();
    nums.sort();

    let avg = (nums.iter().sum::<i32>() as f64 / nums.len() as f64).round() as i32;
    let mid = nums[nums.len() / 2];
    let mode = get_mode(&nums);
    let rng = nums.last().unwrap() - nums.first().unwrap();

    println!("{avg}");
    println!("{mid}");
    println!("{mode}");
    println!("{rng}");
}

fn get_mode(nums: &Vec<i32>) -> i32 {
    let mut max_count = i32::MIN;
    let mut mp = HashMap::new();

    for num in nums {
        let count = mp.entry(num).or_insert(0);
        *count += 1;
        max_count = max_count.max(*count);
    }

    let mut modes = mp.into_iter()
        .filter(|(_, count)| *count == max_count)
        .map(|(num, _)| *num)
        .collect::<Vec<_>>();

    modes.sort();

    if modes.len() == 1 {
        modes[0]
    } else {
        modes[1]
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