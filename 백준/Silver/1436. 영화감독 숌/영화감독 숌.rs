use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let mut nth = buf.trim().parse::<i32>().unwrap();
    let mut number = 665;

    while nth > 0 {
        number = next_666(number);
        nth -= 1;
    }

    println!("{number}");
}

fn include_666(mut n: i32) -> bool {
    while n >= 666 {
        if (n % 1000) == 666 {
            return true;
        }
        n /= 10;
    }

    false
}

fn next_666(mut n: i32) -> i32 {
    n += 1;

    while !include_666(n) {
        n += 1;
    }

    n
}