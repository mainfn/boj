use std::{
    collections::VecDeque,
    fmt::Write,
    io::{stdin, Read},
};

fn main() {
    let mut output = String::new();
    truncate_line();

    let mut deque = VecDeque::new();

    let lines = input_lines();

    for line in lines {
        let value = process_cmd(&mut deque, &line);

        if value.is_some() {
            writeln!(&mut output, "{}", value.unwrap()).unwrap();
        }
    }

    print!("{output}");
}

fn process_cmd(deque: &mut VecDeque<i32>, cmd: &str) -> Option<i32> {
    let mut iter = cmd.split_ascii_whitespace();

    match iter.next().unwrap() {
        "push_front" => {
            let value = iter.next().unwrap().parse::<i32>().unwrap();
            deque.push_front(value);
            None
        }
        "push_back" => {
            let value = iter.next().unwrap().parse::<i32>().unwrap();
            deque.push_back(value);
            None
        }
        "pop_front" => Some(deque.pop_front().unwrap_or(-1)),
        "pop_back" => Some(deque.pop_back().unwrap_or(-1)),
        "size" => Some(deque.len() as i32),
        "empty" => Some(if deque.is_empty() { 1 } else { 0 }),
        "front" => Some(*deque.front().unwrap_or(&-1)),
        "back" => Some(*deque.back().unwrap_or(&-1)),
        _ => None,
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
