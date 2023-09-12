use std::{collections::VecDeque, io::stdin, str::FromStr};

fn main() {
    let t = input_number::<i32>();

    for _ in 0..t {
        let count = func();
        println!("{count}");
    }
}

fn func() -> i32 {
    let mut printed_count = 0;
    let target_index = input_line_splitted_as::<usize>()[1];
    let mut nums = input_line_splitted_as::<i32>();
    let mut printer_queue = nums
        .clone()
        .into_iter()
        .enumerate()
        .collect::<VecDeque<_>>();
    nums.sort_by(|a, b| b.cmp(a));
    let mut priorities = nums.iter().collect::<VecDeque<_>>();

    loop {
        let max_priority = **priorities.front().unwrap();
        let (index, priority) = printer_queue.pop_front().unwrap();

        if priority == max_priority {
            printed_count += 1;
            priorities.pop_front();
            if index == target_index {
                return printed_count;
            }
        } else {
            printer_queue.push_back((index, priority));
        }
    }
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

fn input_line_splitted_as<T>() -> Vec<T>
where
    T: Eq + Ord + Copy + FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}
