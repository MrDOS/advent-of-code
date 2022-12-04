use std::{collections::HashSet, io::stdin};

trait Item {
    fn priority(&self) -> i32;
}

impl Item for u8 {
    fn priority(&self) -> i32 {
        (match self {
            b'a'..=b'z' => self - b'a' + 1,
            b'A'..=b'Z' => self - b'A' + 27,
            _ => 0,
        }) as i32
    }
}

fn main() {
    let mut duplicates_sum: i32 = 0;
    let mut badges_sum: i32 = 0;

    let mut group: [HashSet<u8>; 3] = [HashSet::new(), HashSet::new(), HashSet::new()];

    for (i, line) in stdin().lines().map(|line| line.unwrap()).enumerate() {
        group[i % group.len()].extend(line.bytes());

        let compartments: [HashSet<_>; 2] = [
            HashSet::from_iter(line.bytes().take(line.len() / 2)),
            HashSet::from_iter(line.bytes().skip(line.len() / 2)),
        ];

        for duplicate in &compartments[0] & &compartments[1] {
            duplicates_sum += duplicate.priority();
        }

        if i % group.len() == group.len() - 1 {
            for badge in &(&group[0] & &group[1]) & &group[2] {
                badges_sum += badge.priority();
            }

            for g in group.iter_mut() {
                g.clear();
            }
        }
    }

    println!("Sum of priorities of duplicate items: {}", duplicates_sum);
    println!("Sum of priorities of group badges: {}", badges_sum);
}
