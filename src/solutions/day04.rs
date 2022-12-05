use crate::{SolutionKey, SOLUTIONS};

use color_eyre::eyre::Result;
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day4"));

fn solve() -> Result<()> {
    let mut res = 0;
    for line in INPUT.trim().lines() {
        let (first, second) = line.split_once(',').unwrap();
        let first = first.split_once('-').unwrap();
        let second = second.split_once('-').unwrap();
        let first: [u64; 2] = [first.0.parse()?, first.1.parse()?];
        let second: [u64; 2] = [second.0.parse()?, second.1.parse()?];
        if (first[0] <= second[0] && first[1] >= second[1])
            || (second[0] <= first[0] && second[1] >= first[1])
        {
            res += 1;
        }
    }

    println!("Answer: {res}");

    Ok(())
}

fn solve2() -> Result<()> {
    let mut res = 0;
    for line in INPUT.trim().lines() {
        let (first, second) = line.split_once(',').unwrap();
        let first = first.split_once('-').unwrap();
        let second = second.split_once('-').unwrap();
        let first: [u64; 2] = [first.0.parse()?, first.1.parse()?];
        let second: [u64; 2] = [second.0.parse()?, second.1.parse()?];
        if (first[0] <= second[0] && first[1] >= second[1])
            || (second[0] <= first[0] && second[1] >= first[1])
            || (first[0] <= second[0] && first[1] >= second[0])
            || (second[0] <= first[0] && second[1] >= first[0])
        {
            res += 1;
        }
    }

    println!("Answer: {res}");

    Ok(())
}

#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_04_1: SolutionKey = (4, 1, solve);
#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_04_2: SolutionKey = (4, 2, solve2);
