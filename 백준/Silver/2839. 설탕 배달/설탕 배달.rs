use std::io::stdin;
use std::str::FromStr;

fn main() {
    let n = input_number::<usize>();

    let mut max_five_count = n / 5;

    for five_count in (0..=max_five_count).rev() {
        match get_count(n, five_count) {
            Ok(ans) => {
                println!("{ans}");
                return;
            }
            _ => {}
        }
    }

    println!("-1");
}

fn get_count(mut n: usize, five_count: usize) -> Result<usize, ()> {
    let mut count = five_count;

    n -= 5 * five_count;
    count += n / 3;
    n %= 3;

    if n > 0 {
        return Err(());
    }
    Ok(count)
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