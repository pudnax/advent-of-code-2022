use crate::{SolutionKey, SOLUTIONS};

use color_eyre::eyre::Result;
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day5"));

#[derive(Clone, Copy, Debug)]
struct Move {
    from: usize,
    to: usize,
    val: usize,
}

impl TryFrom<&str> for Move {
    type Error = color_eyre::Report;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut words = value.split_whitespace();
        words.next();
        let val = words.next().unwrap().parse()?;
        words.next();
        let from = words.next().unwrap().parse::<usize>()? - 1;
        words.next();
        let to = words.next().unwrap().parse::<usize>()? - 1;

        Ok(Self { from, to, val })
    }
}

fn parse_input() -> Result<(Vec<Vec<char>>, Vec<Move>)> {
    let input = INPUT.lines();
    let mut crates = vec![];
    let mut moves = vec![];
    let mut first_part = true;
    for line in input {
        if line.is_empty() {
            first_part = false;
            continue;
        }
        if first_part {
            crates.push(line.chars().collect::<Vec<_>>());
        } else {
            moves.push(Move::try_from(line)?);
        }
    }

    let [_, n] = [crates[0].len(), crates.len()];
    let mut stacks = vec![];
    for i in 0..9 {
        let i = 1 + i * 4;
        let mut stack = vec![];
        for j in 1..n {
            let j = n - (j + 1);
            if crates[j][i].is_whitespace() {
                break;
            }
            stack.push(crates[j][i]);
        }
        stacks.push(stack);
    }

    Ok((stacks, moves))
}

fn solve() -> Result<()> {
    let (mut stacks, moves) = parse_input()?;
    for m in moves {
        for _ in 0..m.val {
            let c = stacks[m.from].pop().unwrap();
            stacks[m.to].push(c);
        }
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    Ok(())
}

fn solve2() -> Result<()> {
    let (mut stacks, moves) = parse_input()?;
    for m in moves {
        let mut temp = vec![];
        for _ in 0..m.val {
            let c = stacks[m.from].pop().unwrap();
            temp.push(c);
        }
        for c in temp.into_iter().rev() {
            stacks[m.to].push(c);
        }
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    Ok(())
}

#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_05_1: SolutionKey = (5, 1, solve);
#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_05_2: SolutionKey = (5, 2, solve2);
