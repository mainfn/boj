use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.expect("Failed to read line");
        if input.trim_end() == "." {
            break;
        }
        let cleaned_input = input.chars().filter(|&c| "()[]".contains(c)).collect::<String>();
        let result = is_balanced(&cleaned_input);
        println!("{}", if result { "yes" } else { "no" });
    }
}

fn is_balanced(input: &str) -> bool {
    let mut paren_stack = VecDeque::new();

    for c in input.chars() {
        if paren_stack.iter().len() > 0 {
            let ch = paren_stack.iter().last().unwrap();
            if c == ')' && *ch == '(' || c == ']' && *ch == '[' {
                paren_stack.pop_back();
                continue;
            }
        }
        paren_stack.push_back(c);
    }


    paren_stack.is_empty()
}
