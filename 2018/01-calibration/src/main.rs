use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();
    let mut deltas = Vec::new();
    let mut final_frequency = 0;
    let mut frequencies = HashSet::new();
    let first_repeated_frequency;

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        if input.len() == 0 {
            break;
        }

        let delta = input.trim().parse::<i32>().unwrap();
        deltas.push(delta);
        final_frequency += delta;
    }

    let mut current_frequency = 0;
    'repeat_check: loop {
        for delta in &deltas {
            current_frequency += delta;

            if frequencies.contains(&current_frequency) {
                first_repeated_frequency = current_frequency;
                break 'repeat_check;
            }

            frequencies.insert(current_frequency);
        }
    }

    println!("Final frequency: {}", final_frequency);
    println!("First repeated frequency: {}", first_repeated_frequency);
}
