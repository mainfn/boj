use std::io::{stdin, Read};
use std::str::FromStr;

fn main() {
    let nums = input_vec::<usize>();
    let ans = nums.iter().fold(1, |r, n| r * n);
    let mut counting = [0usize; 10];

    {
        let mut ans = ans;

        while ans > 0 {
            let digit = ans % 10;
            counting[digit] += 1;
            ans /= 10;
        }
    }

    for n in counting {
        println!("{n}");
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
