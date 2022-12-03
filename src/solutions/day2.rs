use crate::{SolutionKey, SOLUTIONS};

use color_eyre::eyre::Result;
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day2"));

#[derive(Clone, Copy, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn worst(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn best(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Fourth Hand?! Panic!"),
        }
    }
}

impl From<char> for Hand {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Fourth Hand?! Panic!"),
        }
    }
}

fn shake(opponent: Hand, me: Hand) -> u64 {
    use Hand::*;
    let outcome = match (me, opponent) {
        // win
        (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => 6,
        // draw
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        // lose
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 0,
    };
    outcome + me.score()
}

pub fn solve() -> Result<()> {
    let mut total = 0;
    for play in INPUT.trim().split('\n') {
        if let Some((opponent, me)) = play.split_once(' ') {
            let opponent = Hand::from(opponent);
            let me = Hand::from(me);
            total += shake(opponent, me);
        }
    }

    println!("Answer: {total}");
    Ok(())
}

enum Round {
    Win,
    Draw,
    Lose,
}

impl From<&str> for Round {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Fourth Fate?! Panic!"),
        }
    }
}

impl From<char> for Round {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Fourth Fate?! Panic!"),
        }
    }
}

pub fn solve2() -> Result<()> {
    let mut total = 0;
    for play in INPUT.trim().split('\n') {
        if let Some((opponent, round)) = play.split_once(' ') {
            let opponent = Hand::from(opponent);
            let round = Round::from(round);
            total += match round {
                Round::Win => shake(opponent, opponent.worst()),
                Round::Draw => shake(opponent, opponent),
                Round::Lose => shake(opponent, opponent.best()),
            };
        }
    }

    println!("Answer: {total}");
    Ok(())
}

#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_02_1: SolutionKey = (2, 1, solve);
#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_02_2: SolutionKey = (2, 2, solve2);
