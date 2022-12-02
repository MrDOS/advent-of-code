use std::{io::stdin, str::FromStr};

enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Eq, Hash, PartialEq)]
enum MyPlay {
    Rock,
    Paper,
    Scissors,
}

impl MyPlay {
    fn play(self, other: TheirPlay) -> i32 {
        let mut score = match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        };

        score += match self {
            Self::Rock => match other {
                TheirPlay::Rock => Outcome::Draw,
                TheirPlay::Paper => Outcome::Loss,
                TheirPlay::Scissors => Outcome::Win,
            },
            Self::Paper => match other {
                TheirPlay::Rock => Outcome::Win,
                TheirPlay::Paper => Outcome::Draw,
                TheirPlay::Scissors => Outcome::Loss,
            },
            Self::Scissors => match other {
                TheirPlay::Rock => Outcome::Loss,
                TheirPlay::Paper => Outcome::Win,
                TheirPlay::Scissors => Outcome::Draw,
            },
        } as i32;

        return score;
    }
}

impl FromStr for MyPlay {
    type Err = ();

    fn from_str(input: &str) -> Result<MyPlay, Self::Err> {
        match input {
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

enum TheirPlay {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for TheirPlay {
    type Err = ();

    fn from_str(input: &str) -> Result<TheirPlay, Self::Err> {
        match input {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

fn main() {
    let mut score = 0;

    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut plays = line.split_whitespace();
        let theirs = TheirPlay::from_str(plays.next().unwrap()).unwrap();
        let mine = MyPlay::from_str(plays.next().unwrap()).unwrap();
        score += mine.play(theirs);
    }

    println!("{}", score);
}
