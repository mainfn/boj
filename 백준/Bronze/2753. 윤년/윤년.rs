use std::io::stdin;
use std::str::FromStr;

fn main() {
    let year = input_number::<i32>();

    let is_leap_year = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;

    println!("{}", if is_leap_year { 1 } else { 0 });
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
