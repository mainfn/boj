use std::io::stdin;
use std::str::FromStr;

fn main() {
    let n = input_number::<i32>();

    for i in 1..=9 {
        println!("{} * {} = {}", n, i, n * i);
    }
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
