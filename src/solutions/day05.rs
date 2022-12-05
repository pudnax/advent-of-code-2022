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
        let mut words = value
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok());
        let val = words.next().unwrap();
        let from = words.next().unwrap() - 1;
        let to = words.next().unwrap() - 1;

        Ok(Self { from, to, val })
    }
}

fn parse_input() -> Result<(Vec<Vec<char>>, Vec<Move>)> {
    let (starter, instructions) = INPUT.split_once("\n\n").unwrap();
    let crates: Vec<Vec<_>> = starter.lines().map(|l| l.chars().collect()).collect();
    let moves = instructions
        .lines()
        .map(Move::try_from)
        .collect::<Result<Vec<_>>>()?;

    let [_, n] = [crates[0].len(), crates.len()];
    let mut stacks = vec![];
    for i in (0..9).map(|i| 1 + i * 4) {
        let mut stack = vec![];
        for j in (1..n).map(|j| n - (j + 1)) {
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
            if let Some(c) = stacks[m.from].pop() {
                stacks[m.to].push(c);
            }
        }
    }
    let res = stacks
        .into_iter()
        .filter_map(|stack| stack.last().cloned())
        .collect::<String>();
    println!("Answer: {res}");
    Ok(())
}

fn solve2() -> Result<()> {
    let (mut stacks, moves) = parse_input()?;
    let mut buf = vec![];
    for m in moves {
        let pos = stacks[m.from].len() - m.val;
        buf.extend(stacks[m.from].drain(pos..));
        stacks[m.to].extend(buf.iter().cloned());
        buf.clear();
    }

    let res = stacks
        .into_iter()
        .filter_map(|stack| stack.last().cloned())
        .collect::<String>();
    println!("Answer: {res}");
    Ok(())
}

#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_05_1: SolutionKey = (5, 1, solve);
#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_05_2: SolutionKey = (5, 2, solve2);
