use std::io::{Read, stdin};

fn main() {
    truncate_line();
    let board = input_strings();

    let min_count = find_min_change_count(&board);
    println!("{min_count}");
}

fn find_min_change_count(board: &[String]) -> usize {
    let color_patterns = vec![vec!['B', 'W'], vec!['W', 'B']];
    let mut min_change_count = usize::MAX;

    for color_pattern in color_patterns {
        for y_start in 0..=board.len() - 8 {
            for x_start in 0..=board[0].len() - 8 {
                min_change_count = min_change_count.min(
                    count_to_repaint(
                        board,
                        &color_pattern,
                        y_start,
                        x_start,
                    )
                );
            }
        }
    }

    min_change_count
}

fn count_to_repaint(
    board: &[String],
    color_pattern: &Vec<char>,
    y_start: usize,
    x_start: usize,
) -> usize {
    let mut change_count = 0;
    let first_color = color_pattern[0];
    let second_color = color_pattern[1];

    for y in y_start..y_start + 8 {
        for x in x_start..x_start + 8 {
            let color = board[y].chars().nth(x).unwrap();

            let to_repaint_in_odd = (x + y) & 1 == 0 && color == first_color;
            let to_repaint_in_even = (x + y) & 1 != 0 && color == second_color;

            if to_repaint_in_odd || to_repaint_in_even {
                change_count += 1;
            }
        }
    }

    change_count
}

fn truncate_line() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}

fn input_strings() -> Vec<String> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    buf.trim().split('\n')
        .map(|s| s.to_owned())
        .collect()
}
