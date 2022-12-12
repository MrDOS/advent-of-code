use std::{i32::MAX, io::stdin, vec};

use pathfinding::prelude::dijkstra;

fn main() {
    let mut hill: Vec<Vec<_>> = Vec::new();
    let mut marked_start = (0, 0);
    let mut all_starts = Vec::new();
    let mut end = (0, 0);

    for (row, line) in stdin().lines().map(|line| line.unwrap()).enumerate() {
        hill.push(
            line.bytes()
                .enumerate()
                .map(|(col, b)| {
                    if b == b'S' {
                        marked_start = (row as i32, col as i32);
                        all_starts.push(marked_start);
                        0
                    } else if b == b'a' {
                        all_starts.push((row as i32, col as i32));
                        0
                    } else if b == b'E' {
                        end = (row as i32, col as i32);
                        25
                    } else {
                        b - b'a'
                    }
                })
                .collect(),
        );
    }

    let hill_depth = hill.len() as i32;
    let hill_width = hill[0].len() as i32;

    let mut marked_start_step_count = MAX;
    let mut min_step_count = MAX;

    for start in all_starts {
        let path = dijkstra(
            &start,
            |&(row, col)| {
                let current_height = hill[row as usize][col as usize];

                vec![
                    (row - 1, col as i32),
                    (row, col - 1),
                    (row, col + 1),
                    (row + 1, col),
                ]
                .into_iter()
                .filter(|&(next_row, next_col)| {
                    next_row >= 0
                        && next_col >= 0
                        && next_row < hill_depth
                        && next_col < hill_width
                        && hill[next_row as usize][next_col as usize] <= current_height + 1
                })
                .map(|n| (n, 1))
                .collect::<Vec<((i32, i32), i32)>>()
            },
            |&n| n == end,
        );

        match path {
            Some(steps) => {
                let step_count = (steps.0.len() - 1) as i32;

                if start == marked_start {
                    marked_start_step_count = step_count;
                }

                if step_count < min_step_count {
                    min_step_count = step_count;
                }
            },
            None => (),
        };
    }

    println!("{} steps from marked start.", marked_start_step_count);
    println!("{} steps from the closest start.", min_step_count);
}
