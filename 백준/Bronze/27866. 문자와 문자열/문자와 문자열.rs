use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let v = buf.trim().split_ascii_whitespace().collect::<Vec<&str>>();
    let index = v[1].parse::<usize>().unwrap() - 1;
    let ch = v[0].chars().nth(index).unwrap();
    println!("{ch}");
}