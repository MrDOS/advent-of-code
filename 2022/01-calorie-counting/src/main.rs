use std::io::{stdin, BufRead};

fn main() {
    let mut last_max = 0;
    let mut current = 0;

    for line in stdin().lock().lines().map(|line| line.unwrap()) {
        if line == "" {
            if current > last_max {
                last_max = current;
            }
            current = 0;

            continue;
        }

        current += line.parse::<i32>().unwrap();
    }

    println!("{}", last_max);
}
