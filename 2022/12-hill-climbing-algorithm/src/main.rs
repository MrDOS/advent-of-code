use std::{io::stdin, vec};

use pathfinding::prelude::dijkstra;

fn main() {
    let mut hill: Vec<Vec<_>> = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row, line) in stdin().lines().map(|line| line.unwrap()).enumerate() {
        hill.push(
            line.bytes()
                .enumerate()
                .map(|(col, b)| {
                    if b == b'S' {
                        start = (row as i32, col as i32);
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
        Some(steps) => println!("{}", steps.0.len() - 1),
        None => println!("Could not find a path from the start to the end!"),
    }
}
