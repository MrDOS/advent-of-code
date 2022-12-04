use std::io::stdin;

struct ContainableRange {
    start: i32,
    end: i32,
}

impl From<&str> for ContainableRange {
    fn from(range: &str) -> Self {
        match range.split_once("-") {
            Some((start, end)) => ContainableRange {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            },
            None => ContainableRange { start: 0, end: 0 },
        }
    }
}

impl ContainableRange {
    fn contains(&self, other: &ContainableRange) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }
}

fn main() {
    let mut contained = 0;

    for line in stdin().lines().map(|line| line.unwrap()) {
        let pieces: Vec<_> = line
            .split(",")
            .take(2)
            .map(|range| Into::<ContainableRange>::into(range))
            .collect();
        if pieces[0].contains(&pieces[1]) || pieces[1].contains(&pieces[0]) {
            contained += 1;
        }
    }

    println!("{}", contained);
}
