#![allow(dead_code)]
#![feature(iter_array_chunks)]

use color_eyre::eyre::Result;
use linkme::distributed_slice;

mod utils;
mod solutions {
    automod::dir!(pub "src/solutions");
}

type SolutionKey = (usize, usize, fn() -> Result<()>);
#[distributed_slice]
static SOLUTIONS: [SolutionKey] = [..];

fn main() -> Result<()> {
    color_eyre::install()?;

    let day = match std::env::args().nth(1) {
        Some(day) => day.parse::<usize>()?,
        None => {
            eprintln!("please pass the day");
            std::process::exit(1);
        }
    };

    let problem = std::env::args()
        .nth(2)
        .map(|x| x.parse())
        .transpose()?
        .unwrap_or(1);

    SOLUTIONS
        .iter()
        .find(|(i, j, _)| *i == day && *j == problem)
        .unwrap_or_else(|| todo!("day {day} not implemented!"))
        .2()
}
