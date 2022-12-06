use std::collections::HashSet;

use crate::{SolutionKey, SOLUTIONS};

use color_eyre::eyre::Result;
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day6"));

fn solve() -> Result<()> {
    let input: Vec<char> = INPUT.chars().collect();
    let mut res = 4;
    for &arr in input.array_windows::<4>() {
        let set = HashSet::from(arr);
        if set.len() == 4 {
            break;
        }
        res += 1;
    }
    println!("Answer: {res}");
    Ok(())
}

fn solve2() -> Result<()> {
    let input: Vec<char> = INPUT.chars().collect();
    let mut res = 14;
    for &arr in input.array_windows::<14>() {
        let set = HashSet::from(arr);
        if set.len() == 14 {
            break;
        }
        res += 1;
    }
    println!("Answer: {res}");
    Ok(())
}
#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_06_1: SolutionKey = (6, 1, solve);
#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_06_2: SolutionKey = (6, 2, solve2);
