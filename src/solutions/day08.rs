use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

use crate::{SolutionKey, SOLUTIONS};

use color_eyre::eyre::Result;
use linkme::distributed_slice;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day08"));

#[derive(Clone, Copy, Debug, Default)]
struct Height {
    max_left: u8,
    max_right: u8,
    max_up: u8,
    max_down: u8,
}

fn solve() -> Result<()> {
    let t = std::time::Instant::now();
    let forest: Vec<u8> = INPUT
        .lines()
        .flat_map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
        .collect();
    let width = INPUT.lines().next().unwrap().len();

    let height = forest.len() / width;
    let corners = (width + height - 2) * 2;
    let mut visible = HashSet::with_capacity(forest.len() - corners);

    for y in 1..height - 1 {
        // left to right
        let mut max = 0;
        for x in 1..width - 1 {
            max = forest[x + y * width - 1].max(max);
            if max >= 9 {
                break;
            }
            if max < forest[x + y * width] {
                visible.insert((x, y));
            }
        }

        // right to left
        let mut max = 0;
        for x in (1..width - 1).rev() {
            max = forest[x + y * width + 1].max(max);
            if max >= 9 {
                break;
            }
            if max < forest[x + y * width] {
                visible.insert((x, y));
            }
        }
    }

    for x in 1..width - 1 {
        // top to bottom
        let mut max = 0;
        for y in 1..height - 1 {
            max = forest[x + (y - 1) * width].max(max);
            if max >= 9 {
                break;
            }
            if max < forest[x + y * width] {
                visible.insert((x, y));
            }
        }

        // bottom to top
        let mut max = 0;
        for y in (1..height - 1).rev() {
            max = forest[x + (y + 1) * width].max(max);
            if max >= 9 {
                break;
            }
            if max < forest[x + y * width] {
                visible.insert((x, y));
            }
        }
    }

    println!("Answer: {}", visible.len() + corners);
    println!("{:?}", t.elapsed());
    Ok(())
}

struct Grid {
    width: usize,
    height: usize,
    values: Vec<u8>,
}

impl Grid {
    fn new(width: usize, height: usize, values: Vec<u8>) -> Self {
        Self {
            width,
            height,
            values,
        }
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = u8;
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.values[idx.0 + idx.1 * self.width]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.values[idx.0 + idx.1 * self.width]
    }
}

fn solve2() -> Result<()> {
    let t = std::time::Instant::now();
    let grid: Vec<u8> = INPUT
        .lines()
        .flat_map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
        .collect();
    let width = INPUT.lines().next().unwrap().len();
    let height = grid.len() / width;
    let grid = Grid::new(width, height, grid);

    let mut res = 0;
    for row in 1..height - 1 {
        for col in 1..width - 1 {
            let mut acc = [0; 4];
            'view: for (k, [dx, dy]) in [[0, -1], [-1, 0], [0, 1], [1, 0]].into_iter().enumerate() {
                let [mut x, mut y] = [dx, dy];
                while let [Some(i), Some(j)] = [
                    row.checked_add_signed(y).filter(|&r| r < height),
                    col.checked_add_signed(x).filter(|&c| c < width),
                ] {
                    acc[k] += 1;
                    if grid[(i, j)] < grid[(row, col)] {
                        [x, y] = [x + dx, y + dy];
                    } else {
                        continue 'view;
                    }
                }
            }

            res = res.max(acc.iter().map(|x| x.max(&1)).product());
        }
    }
    println!("Answer: {}", res);
    println!("{:?}", t.elapsed());
    Ok(())
}

#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_08_1: SolutionKey = (8, 1, solve);
#[distributed_slice(SOLUTIONS)]
static SOLUTION_DAY_08_2: SolutionKey = (8, 2, solve2);
