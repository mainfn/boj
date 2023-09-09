use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let word = input_line();
    let word = word
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<String>();

    let mut count_map: HashMap<char, usize> = HashMap::new();
    let mut max_count = usize::MIN;

    for c in word.chars() {
        let count = count_map.entry(c).or_insert(0);
        *count += 1;
        max_count = max_count.max(*count);
    }

    let vec = count_map
        .iter()
        .filter(|(_, count)| **count == max_count)
        .map(|(ch, _)| *ch)
        .collect::<Vec<_>>();

    let ans = if vec.len() == 1 { vec[0] } else { '?' };

    println!("{ans}");
}

fn input_line() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}
