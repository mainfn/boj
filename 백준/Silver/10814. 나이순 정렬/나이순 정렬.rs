use std::{fmt::Write, io::stdin, str::FromStr};

struct User {
    id: usize,
    age: u8,
    name: String,
}

fn main() {
    let times = input_number::<i32>();

    let mut users = Vec::<User>::new();

    for id in 0..times as usize {
        users.push(new_user(id));
    }

    users.sort_by(|a, b| {
        if a.age == b.age {
            a.id.cmp(&b.id)
        } else {
            a.age.cmp(&b.age)
        }
    });

    let mut buf = String::new();

    for user in users {
        writeln!(&mut buf, "{} {}", user.age, user.name).unwrap();
    }
    print!("{buf}");
}

fn new_user(id: usize) -> User {
    let input = input_line()
        .split_ascii_whitespace()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    let age = input[0].parse::<u8>().unwrap();
    let name = input[1].clone();
    let user = User { age, name, id };

    user
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

fn input_line() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}
