#![feature(let_chains)]
use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Block {
    Start { position: u64 },
    End { position: u64 },
    Track { position: u64 },
    Wall,
}

const GRID_SIZE: usize = 141;
// const GRID_SIZE: usize = 15;
const INPUT: &str = include_str!("input.txt");

fn main() {
    let (mut start, mut end) = ((0_usize, 0_usize), (0_usize, 0_usize));
    let mut grid: [[Block; GRID_SIZE]; GRID_SIZE] = INPUT
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    'S' => {
                        start = (row, col);
                        Block::Start { position: 0 }
                    }
                    'E' => {
                        end = (row, col);
                        Block::End { position: u64::MAX }
                    }
                    '.' => Block::Track { position: u64::MAX },
                    '#' => Block::Wall,
                    _ => unreachable!(),
                })
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap();

    // Run the track, setting the index of the specific block each time
    let mut cur = start;
    let mut i = 0_u64;
    while grid[cur.0][cur.1] != (Block::End { position: u64::MAX }) {
        // Set current index
        if let Block::Track { ref mut position } = grid[cur.0][cur.1] {
            *position = i;
        }

        // Find the next one and move to it
        if let Block::Track { position } | Block::End { position } = grid[cur.0 + 1][cur.1]
            && position == u64::MAX
        {
            cur = (cur.0 + 1, cur.1);
        } else if let Block::Track { position } | Block::End { position } = grid[cur.0 - 1][cur.1]
            && position == u64::MAX
        {
            cur = (cur.0 - 1, cur.1);
        } else if let Block::Track { position } | Block::End { position } = grid[cur.0][cur.1 - 1]
            && position == u64::MAX
        {
            cur = (cur.0, cur.1 - 1);
        } else if let Block::Track { position } | Block::End { position } = grid[cur.0][cur.1 + 1]
            && position == u64::MAX
        {
            cur = (cur.0, cur.1 + 1);
        } else {
            unreachable!();
        }

        // Increment and continue
        i += 1;
    }
    // Set end position too
    if let Block::End { ref mut position } = grid[cur.0][cur.1] {
        *position = i;
    } else {
        unreachable!();
    }

    // Iterate over it again, this time checking if a cheat is skipping more than 100
    // Run the track, setting the index of the specific block each time
    let mut sum = 0;
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let (Block::Start {
                position: cur_position,
            }
            | Block::End {
                position: cur_position,
            }
            | Block::Track {
                position: cur_position,
            }) = grid[row][col]
            else {
                continue;
            };

            // Check all directions for a skip
            let to_check = [
                ((row.checked_add(1), Some(col)), (row.checked_add(2), Some(col))), // Down
                ((row.checked_sub(1), Some(col)), (row.checked_sub(2), Some(col))), // Up
                ((Some(row), col.checked_add(1)), (Some(row), col.checked_add(2))), // Right
                ((Some(row), col.checked_sub(1)), (Some(row), col.checked_sub(2))), // Left
            ];

            for (single, double) in to_check {
                if let (Some(single_row), Some(single_col)) = single
                    && let (Some(double_row), Some(double_col)) = double
                    && itertools::max([single_row, single_col, double_row, double_col]).unwrap() < GRID_SIZE
                    && let Block::Wall = grid[single_row][single_col]
                    && let Block::Track { position } | Block::End { position } = grid[double_row][double_col]
                    && position > cur_position
                    && (position - cur_position - 2) >= 100
                {
                    sum += 1;
                }
            }
        }
    }

    println!("Result: {sum}");
}
