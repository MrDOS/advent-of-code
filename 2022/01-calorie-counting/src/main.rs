use std::io::{stdin, BufRead};

fn main() {
    let mut elves = Vec::new();
    let mut current = 0;

    for line in stdin().lock().lines().map(|line| line.unwrap()) {
        if line == "" {
            elves.push(current);
            current = 0;

            continue;
        }

        current += line.parse::<i32>().unwrap();
    }

    elves.sort();
    elves.reverse();

    println!("The snackiest elf is holding {} Calories.", elves[0]);

    println!(
        "The top three snackiest elves are collectively carrying {} Calories.",
        elves.iter().take(3).sum::<i32>(),
    );
}
