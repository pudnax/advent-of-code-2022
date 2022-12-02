use crate::SOLUTIONS;
use std::cmp::Reverse;

use color_eyre::eyre::Result;
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day1"));

pub fn solve() -> Result<()> {
    let mut max_pudgy = 0;
    let elves = INPUT.trim().split("\n\n");
    for gluttony in elves {
        let total = gluttony
            .split('\n')
            .map(|x| x.parse::<u64>().expect("Invalid number?! Blasphemy!"))
            .sum();
        max_pudgy = max_pudgy.max(total);
    }

    println!("Answer: {max_pudgy}");
    Ok(())
}

pub fn solve2() -> Result<()> {
    let mut all = vec![];
    let elves = INPUT.trim().split("\n\n");
    for gluttony in elves {
        let total: u64 = gluttony
            .split('\n')
            .map(|x| x.parse::<u64>().expect("Invalid number?! Blasphemy!"))
            .sum();
        all.push(total);
    }
    all.sort_by_key(|&x| Reverse(x));
    let max_pudgy: u64 = all[0..3].iter().sum();

    println!("Answer: {max_pudgy}");
    Ok(())
}

pub fn solve3() -> Result<()> {
    let elves = INPUT.trim().split("\n\n");
    let mut top3 = [0u64; 3];
    for gluttony in elves {
        let total: u64 = gluttony
            .split('\n')
            .map(|x| x.parse::<u64>().expect("Invalid number?! Blasphemy!"))
            .sum();
        let [first, second, ..] = top3;
        use std::cmp::Ordering::*;
        top3 = match top3.map(|x| total.cmp(&x)) {
            [Greater | Equal, ..] => [total, first, second],
            [Less, Greater | Equal, ..] => [first, total, second],
            [Less, Less, Greater | Equal] => [first, second, total],
            _ => top3,
        };
    }
    let max_pudgy: u64 = top3.iter().sum();
    println!("Answer: {max_pudgy}");
    Ok(())
}

#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_01_1: (usize, usize, fn() -> Result<()>) = (1, 1, solve);
#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_01_2: (usize, usize, fn() -> Result<()>) = (1, 2, solve3);
