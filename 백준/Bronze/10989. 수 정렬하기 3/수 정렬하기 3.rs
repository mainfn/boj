use std::fmt::Write;
use std::io::stdin;
use std::str::FromStr;

fn main() {
    let n = input_number::<usize>();

    let mut counting_sort = [0_u32; 10001];

    for _ in 0..n {
        let number = input_number::<usize>();
        counting_sort[number] += 1;
    }

    for (num, count) in counting_sort.into_iter().enumerate() {
        let mut output = String::new();

        for _ in 0..count {
            if output.len() > 10000 {
                print!("{output}");
                output = String::new();
            }
            writeln!(&mut output, "{num}").unwrap();
        }

        print!("{output}");
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