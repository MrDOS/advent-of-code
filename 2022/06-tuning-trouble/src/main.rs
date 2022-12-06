use std::{collections::HashSet, hash::Hash, io::stdin};

const PACKET_MARKER_LENGTH: usize = 4;
const MESSAGE_MARKER_LENGTH: usize = 14;

fn find_end_of_unique_run<T>(elements: &Vec<T>, run_length: usize) -> Option<usize>
where
    T: Eq,
    T: Hash,
{
    let mut observed_elements: HashSet<&T> = HashSet::new();

    for i in run_length..=elements.len() {
        for j in i - run_length..i {
            observed_elements.insert(&elements[j]);
        }

        if observed_elements.len() == run_length {
            return Option::Some(i);
        }

        observed_elements.clear();
    }

    return Option::None;
}

fn main() {
    for signal in stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
    {
        match find_end_of_unique_run(&signal, PACKET_MARKER_LENGTH) {
            Some(end) => println!("First start-of-packet marker after character {}", end),
            _ => (),
        };

        match find_end_of_unique_run(&signal, MESSAGE_MARKER_LENGTH) {
            Some(end) => println!("First start-of-message marker after character {}", end),
            _ => (),
        };
    }
}
