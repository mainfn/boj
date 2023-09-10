use std::{
    io::{stdin, Read},
    str::FromStr,
};

fn main() {
    let m = input_line_splitted_as::<i32>()[1];
    let nums = input_all_splitted_as::<i32>();

    let mut closest_sum = 0;
    let n = nums.len();

    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let sum = nums[i] + nums[j] + nums[k];
                if sum <= m && sum > closest_sum {
                    closest_sum = sum;
                }
            }
        }
    }

    println!("{closest_sum}");
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

fn input_all_splitted_as<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    buf.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}
