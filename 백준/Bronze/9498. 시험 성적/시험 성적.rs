use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();

    stdin().read_to_string(&mut buf).unwrap();

    let score = buf.trim().parse::<i32>().unwrap();

    let s = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F'
    };
    println!("{s}");
}
