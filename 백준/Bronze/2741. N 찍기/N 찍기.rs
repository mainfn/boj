use std::fmt::Write;
use std::io::stdin;
use std::str::FromStr;

fn main() {
    let n = input_number::<i32>();

    let mut buf = String::new();

    for i in 1..=n as usize {
        writeln!(buf, "{i}").unwrap();
    }

    print!("{buf}");
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
