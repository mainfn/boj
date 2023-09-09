use std::io::{stdin, Read};

fn main() {
    truncate_line();

    for line in input_lines() {
        let mut total_score = 0;
        let mut score = 1;
        for c in line.chars() {
            if c == 'O' {
                total_score += score;
                score += 1;
            } else {
                score = 1;
            }
        }
        println!("{total_score}");
    }
}

fn truncate_line() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}

fn input_lines() -> Vec<String> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    buf.trim()
        .split('\n')
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}
