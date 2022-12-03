use std::collections::HashSet;

use crate::{SolutionKey, SOLUTIONS};

use color_eyre::eyre::Result;
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day3"));

fn solve() -> Result<()> {
    let mut res = 0u64;
    for sack in INPUT.trim().lines() {
        let (l, r) = sack.split_at(sack.len() / 2);
        let [l, r] = [l, r].map(|s| s.chars().collect::<HashSet<_>>());

        let intersection = l.intersection(&r).next().unwrap();
        if intersection.is_uppercase() {
            res += (*intersection as u8 - b'A') as u64 + 27;
        } else {
            res += (*intersection as u8 - b'a') as u64 + 1;
        }
    }

    println!("Answer: {res}");
    Ok(())
}

fn solve2() -> Result<()> {
    let mut res = 0;
    for [a, b, c] in INPUT.trim().lines().array_chunks() {
        let intersection = a.chars().find(|&x| b.contains(x) && c.contains(x)).unwrap();
        res += if intersection.is_uppercase() {
            (intersection as u8 - b'A') as u64 + 27
        } else {
            (intersection as u8 - b'a') as u64 + 1
        };
    }

    println!("Answer: {res}");
    Ok(())
}

#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_03_1: SolutionKey = (3, 1, solve);
#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_03_2: SolutionKey = (3, 2, solve2);
