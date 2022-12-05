use std::{collections::VecDeque, io::stdin};

use itertools::Itertools;

fn main() {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    for line in stdin().lines().map(|line| line.unwrap()) {
        if line == "" {
            break;
        }

        if stacks.len() == 0 {
            for _ in 0..=line.len() / 4 {
                stacks.push(VecDeque::new());
            }
        }

        let chars: Vec<_> = line.chars().collect();
        for i in 0..=line.len() / 4 {
            let crate_sym = chars[i * 4 + 1];
            if ('A'..='Z').contains(&crate_sym) {
                stacks[i].push_back(crate_sym);
            }
        }
    }

    for line in stdin().lines().map(|line| line.unwrap()) {
        let words = line.split_whitespace().collect_tuple();
        match words {
            Some((_, quantity, _, source, _, target)) => {
                let quantity = quantity.parse::<usize>().unwrap();
                let source = source.parse::<usize>().unwrap();
                let target = target.parse::<usize>().unwrap();
                for _ in 0..quantity {
                    let crate_sym = stacks[source - 1].pop_front().unwrap();
                    stacks[target - 1].push_front(crate_sym);
                }
            }
            _ => panic!(),
        }
    }

    for stack in stacks {
        print!("{}", stack.front().unwrap());
    }
    println!();
}
