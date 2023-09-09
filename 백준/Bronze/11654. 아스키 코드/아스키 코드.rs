use std::io::stdin;

fn main() {
    let c = input_line().trim().chars().nth(0).unwrap() as u8;
    println!("{c}");
}

fn input_line() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}
