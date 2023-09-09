use std::{
    io::{stdin, Read},
    str::FromStr,
};

fn main() {
    let nums = input_vec::<i32>();
    let mut ascending = nums.clone();
    let mut descending = nums.clone();

    ascending.sort();
    descending.sort_by(|a, b| b.cmp(a));

    println!(
        "{}",
        if nums == ascending {
            "ascending"
        } else if nums == descending {
            "descending"
        } else {
            "mixed"
        }
    );
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
