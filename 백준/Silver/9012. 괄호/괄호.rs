use std::collections::VecDeque;
use std::io::stdin;
use std::str::FromStr;

fn main() {
    let n = input_number::<i32>();

    for _ in 0..n {
        vps();
    }
}

fn vps() {
    let parens = input_line_as_chars();

    let mut stack = VecDeque::<char>::new();

    for paren in parens {
        let back = stack.back().unwrap_or(&'_');
        if paren == ')' && *back == '(' {
            stack.pop_back();
        } else {
            stack.push_back(paren);
        }
    }

    println!("{}",
             if stack.is_empty() {
                 "YES"
             } else {
                 "NO"
             });
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


fn input_line_as_chars() -> Vec<char>
{
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.trim()
        .chars()
        .collect::<Vec<char>>()
}