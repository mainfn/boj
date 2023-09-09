use std::collections::HashSet;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let deduplicated_words: HashSet<&str> = buf.split_ascii_whitespace().collect();
    let mut words: Vec<&str> = deduplicated_words.into_iter().collect();

    words.sort_by(|a, b| {
        if a.len() == b.len() {
            a.cmp(b)
        } else {
            a.len().cmp(&b.len())
        }
    });

    println!("{}", words.join("\n"));
}