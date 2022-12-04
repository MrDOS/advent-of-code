use std::{collections::HashSet, io::stdin};

fn main() {
    let mut sum: i32 = 0;

    for line in stdin().lines().map(|line| line.unwrap()) {
        let compartments: [HashSet<_>; 2] = [
            HashSet::from_iter(line.bytes().take(line.len() / 2)),
            HashSet::from_iter(line.bytes().skip(line.len() / 2)),
        ];

        for duplicate in compartments[0].intersection(&compartments[1]) {
            sum += match duplicate {
                b'a'..=b'z' => duplicate - b'a' + 1,
                b'A'..=b'Z' => duplicate - b'A' + 27,
                _ => 0,
            } as i32;
        }
    }

    println!("{}", sum);
}
