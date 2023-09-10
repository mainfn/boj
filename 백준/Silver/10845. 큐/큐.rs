use std::{
    collections::VecDeque,
    fmt::Write,
    io::{stdin, Read},
};

fn main() {
    let mut output = String::new();
    truncate_line();

    let mut queue = VecDeque::new();

    let lines = input_lines();

    for line in lines {
        let value = process_cmd(&mut queue, &line);

        if value.is_some() {
            writeln!(&mut output, "{}", value.unwrap()).unwrap();
        }
    }

    print!("{output}");
}

fn process_cmd(queue: &mut VecDeque<i32>, cmd: &str) -> Option<i32> {
    let mut iter = cmd.split_ascii_whitespace();

    match iter.next().unwrap() {
        "push" => {
            let value = iter.next().unwrap().parse::<i32>().unwrap();
            queue.push_back(value);
            None
        }
        "pop" => Some(queue.pop_front().unwrap_or(-1)),
        "size" => Some(queue.len() as i32),
        "empty" => Some(if queue.is_empty() { 1 } else { 0 }),
        "front" => Some(*queue.front().unwrap_or(&-1)),
        "back" => Some(*queue.back().unwrap_or(&-1)),
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
