use std::{
    io::{stdin, Read},
    str::FromStr,
};

fn main() {
    let m = input_line_splitted_as::<i32>()[1];
    let mut nums = input_all_splitted_as::<i32>();
    nums.sort();

    let mut ans = 0;

    for n1 in nums.iter() {
        for n2 in nums.iter() {
            for n3 in nums.iter() {
                if n1 == n2 || n2 == n3 || n3 == n1 {
                    continue;
                }
                let sum = n1 + n2 + n3;
                if sum <= m && sum > ans {
                    ans = sum;
                }
            }
        }
    }

    println!("{ans}");
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
