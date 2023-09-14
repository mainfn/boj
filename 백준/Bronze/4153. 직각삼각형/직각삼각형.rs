use std::io::stdin;
use std::process::exit;
use std::str::FromStr;

fn main() {
    loop {
        let mut nums = get_nums();
        nums.sort();
        let n0 = nums[0];
        let n1 = nums[1];
        let n2 = nums[2];
        println!("{}", if n2 * n2 == n1 * n1 + n0 * n0 {
            "right"
        } else {
            "wrong"
        });
    }
}

fn get_nums() -> Vec<i32> {
    let nums = input_line_as::<i32>();
    if nums.iter().sum::<i32>() == 0 {
        exit(0);
    }
    nums
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