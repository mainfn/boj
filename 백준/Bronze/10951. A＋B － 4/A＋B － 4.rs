use std::{
    io::{stdin, Read},
    str::FromStr,
};

fn main() {
    let nums = input_vec::<i32>();

    for i in (0..nums.len()).step_by(2) {
        let sum = nums[i] + nums[i + 1];
        println!("{sum}");
    }
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
