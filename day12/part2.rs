#![feature(let_chains)]
use std::collections::{HashSet, VecDeque};

const GRID_SIZE: usize = include_bytes!("input.txt").len().isqrt();

fn dfs(grid: &[[char; GRID_SIZE]; GRID_SIZE], start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::from([start]);
    let mut set = HashSet::from([start]);
    let plant_type = grid[start.0][start.1];

    while let Some(next) = queue.pop_front() {
        let to_check = [
            (next.0 + 1, next.1),
            (next.0.wrapping_sub(1), next.1),
            (next.0, next.1 + 1),
            (next.0, next.1.wrapping_sub(1)),
        ];
        for pos in to_check {
            if set.contains(&pos) {
                continue;
            }

            if let Some(plant) = grid.get(pos.0).and_then(|r| r.get(pos.1))
                && *plant == plant_type
            {
                queue.push_back(pos);
                set.insert(pos);
            }
        }
    }

    set
}

// fn dfs_perimeter(
//     perimeter_set: &HashSet<(isize, isize)>,
//     start: (isize, isize),
// ) -> HashSet<(isize, isize)> {
//     let mut queue = VecDeque::from([start]);
//     let mut set = HashSet::from([start]);

//     while let Some(next) = queue.pop_front() {
//         let to_check = [
//             (next.0 + 1, next.1),
//             (next.0 - 1, next.1),
//             (next.0, next.1 + 1),
//             (next.0, next.1 - 1),
//         ];
//         for pos in to_check {
//             if set.contains(&pos) {
//                 continue;
//             }

//             if perimeter_set.contains(&pos) {
//                 queue.push_back(pos);
//                 set.insert(pos);
//             }
//         }
//     }

//     set
// }

fn main() {
    let mut input_iter = include_str!("input.txt").lines().map(|l| {
        let mut iter = l.chars();
        std::array::from_fn::<_, GRID_SIZE, _>(|_| iter.next().unwrap())
    });
    let grid: [_; GRID_SIZE] = std::array::from_fn(|_| input_iter.next().unwrap());

    // Parse all independent regions
    let mut regions = Vec::<(char, HashSet<(usize, usize)>)>::new();
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if regions.iter().all(|region| !region.1.contains(&(row, col))) {
                regions.push((grid[row][col], dfs(&grid, (row, col))));
            }
        }
    }

    // Sanity check -> are there any duplicated nodes between the regions?
    assert_eq!(
        regions.iter().map(|r| r.1.len()).sum::<usize>(),
        regions
            .iter()
            .map(|r| &r.1)
            .flatten()
            .collect::<HashSet<&(usize, usize)>>()
            .len()
    );

    // Expand grid by two because im going insane
    let mut expanded_grid = [[' '; GRID_SIZE * 2 + 1]; GRID_SIZE * 2 + 1];
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            expanded_grid[row * 2 + 1][col * 2 + 1] = grid[row][col];
        }
    }

    for row in 0..(GRID_SIZE*2+1) {
        print!("|");
        for col in 0..(GRID_SIZE*2+1) {
            print!("{}", expanded_grid[row][col]);
        }
        println!("|");
    }


    // Calculate area and sides of each
    let mut sum = 0;
    for (plant_type, region) in regions.into_iter() {
        let area = region.len();
        let sides = 0;

        // Clone the expanded grid
        let expanded_grid_clone = expanded_grid.clone();
        for pos in region.iter() {
            // None in a position is a special case meaning "overfilled in the negative direction"
            // Since we don't actually access the point there, it just gets treated like -1 would
            let to_check = [
                (pos.0.checked_add(1), Some(pos.1)),
                (pos.0.checked_sub(1), Some(pos.1)),
                (Some(pos.0), pos.1.checked_add(1)),
                (Some(pos.0), pos.1.checked_sub(1)),
            ];
            for outer_pos in to_check {
                if (outer_pos.0.is_none() || outer_pos.1.is_none())
                    || !region.contains(&(outer_pos.0.unwrap(), outer_pos.1.unwrap()))
                {
                    expanded_grid_clone[outer_pos.0.map(|p| p * 2 + 1)]
                }
            }
        }

        sum += area * sides;
        println!("{plant_type}: {area}A {sides}S");
    }

    println!("Result: {sum}");
}
