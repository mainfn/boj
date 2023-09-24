use std::io::{Read, stdin};

fn main() {
    truncate_line();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut ans = 0;
    for (i, ch) in input.trim().char_indices() {
        let n = ch as u32 - 'a' as u32 + 1;

        ans += 31_u32.pow(i as u32) * n;
    }

    println!("{ans}");
}

fn truncate_line() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}
