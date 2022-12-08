use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::{SolutionKey, SOLUTIONS};

use color_eyre::eyre::Result;
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day7"));

fn compute_dir_size(
    fs: &HashMap<PathBuf, HashSet<(i64, &str)>>,
    sizes: &mut HashMap<PathBuf, i64>,
    dir: &PathBuf,
) {
    if sizes.contains_key(dir) {
        return;
    }
    let size = fs[dir]
        .iter()
        .map(|&(s, d)| match s {
            -1 => {
                let dir = dir.join(d);
                compute_dir_size(fs, sizes, &dir);
                sizes[&dir]
            }
            s => s,
        })
        .sum();
    sizes.insert(dir.clone(), size);
}

fn solve() -> Result<()> {
    let mut fs = HashMap::new();
    let mut pwd = PathBuf::new();
    for l in INPUT.split('$').skip(1) {
        match l.trim().lines().next().unwrap() {
            "ls" => {
                let entries = l.lines().skip(1).map(|output| {
                    let (size, f) = output.split_once(' ').unwrap();
                    (size.parse::<i64>().unwrap_or(-1), f)
                });
                fs.entry(pwd.clone())
                    .or_insert(HashSet::new())
                    .extend(entries);
            }
            "cd .." => {
                pwd.pop();
            }
            cd_dir => {
                pwd.push(cd_dir.split_once(' ').unwrap().1);
            }
        }
    }

    let mut sizes = HashMap::new();
    for k in fs.keys() {
        compute_dir_size(&fs, &mut sizes, k);
    }
    let total_size = sizes[&PathBuf::from("/")];
    let p1 = sizes.values().filter(|&&s| s <= 100000).sum::<i64>();
    let p2 = sizes
        .values()
        .filter(|&&s| 40000000 + s >= total_size)
        .min()
        .copied()
        .unwrap();
    println!("{:?}", (p1, p2));

    Ok(())
}

#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_07_1: SolutionKey = (7, 1, solve);
