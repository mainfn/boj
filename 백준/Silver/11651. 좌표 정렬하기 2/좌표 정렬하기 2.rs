use std::fmt::Write;
use std::io::stdin;
use std::str::FromStr;

fn main() {
    let mut n = input_number::<isize>();

    let mut coords = Vec::<(i32, i32)>::new();

    for _ in 0..n {
        let nums = input_line_as::<i32>();
        coords.push((nums[0], nums[1]));
    }

    coords.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let mut output = String::new();
    for coord in coords {
        writeln!(&mut output, "{} {}", coord.0, coord.1).unwrap();
    }

    print!("{output}");
}

fn input_number<T>() -> T
    where
        T: Eq + Ord + Copy + FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<T>().unwrap()
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
        .collect::<Vec<T>>()
}