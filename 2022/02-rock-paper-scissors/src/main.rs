use std::{io::stdin, str::FromStr};

#[derive(Clone, Copy)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    fn play(self, other: TheirPlay) -> i32 {
        let mut score = self as i32;

        score += match self {
            Self::Loss => match other {
                TheirPlay::Rock => MyPlay::Scissors,
                TheirPlay::Paper => MyPlay::Rock,
                TheirPlay::Scissors => MyPlay::Paper,
            },
            Self::Draw => match other {
                TheirPlay::Rock => MyPlay::Rock,
                TheirPlay::Paper => MyPlay::Paper,
                TheirPlay::Scissors => MyPlay::Scissors,
            },
            Self::Win => match other {
                TheirPlay::Rock => MyPlay::Paper,
                TheirPlay::Paper => MyPlay::Scissors,
                TheirPlay::Scissors => MyPlay::Rock,
            },
        } as i32;

        return score;
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
enum MyPlay {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl MyPlay {
    fn play(self, other: TheirPlay) -> i32 {
        let mut score = self as i32;

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

#[derive(Clone, Copy)]
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
    let mut play_score = 0;
    let mut outcome_score = 0;

    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut plays = line.split_whitespace();
        let theirs = TheirPlay::from_str(plays.next().unwrap()).unwrap();

        let second = plays.next().unwrap();
        let mine = MyPlay::from_str(second).unwrap();
        let outcome = Outcome::from_str(second).unwrap();
        play_score += mine.play(theirs);
        outcome_score += outcome.play(theirs);
    }

    println!("Total score if the second column gives plays: {}", play_score);
    println!("Total score if the second column gives outcomes: {}", outcome_score);
}
