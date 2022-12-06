use std::{collections::HashSet, io::stdin};

const MARKER_LENGTH: usize = 4;

fn main() {
    let mut marker_chars: HashSet<char> = HashSet::new();

    for signal in stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
    {
        for i in MARKER_LENGTH..=signal.len() {
            for j in i - MARKER_LENGTH..i {
                marker_chars.insert(signal[j]);
            }

            if marker_chars.len() == MARKER_LENGTH {
                println!("{}", i);
                break;
            }

            marker_chars.clear();
        }

        if marker_chars.len() > 0 {
            marker_chars.clear();
        }
    }
}
