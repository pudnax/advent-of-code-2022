use std::collections::HashSet;

use crate::{SolutionKey, SOLUTIONS};

use color_eyre::eyre::Result;
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day6"));

fn position_of_unique(s: &str, n: usize) -> Option<usize> {
    s.as_bytes()
        .windows(n)
        .position(|arr| HashSet::<u8>::from_iter(arr.iter().copied()).len() == n)
        .map(|n| n + n)
}

fn solve() -> Result<()> {
    let res = position_of_unique(INPUT, 4).unwrap_or(0);
    println!("Answer: {res}");
    Ok(())
}

fn solve2() -> Result<()> {
    let res = position_of_unique(INPUT, 14).unwrap_or(0);
    println!("Answer: {res}");
    Ok(())
}

#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_06_1: SolutionKey = (6, 1, solve);
#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_06_2: SolutionKey = (6, 2, solve2);
