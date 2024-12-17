#![feature(const_str_split_at)]

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
enum BlockType {
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
    Robot,
}

#[derive(Debug, Clone, Copy)]
enum InstructionType {
    Up,
    Down,
    Left,
    Right,
}

const GRID_SIZE: usize = 50;
const EXPANDED_GRID_WIDTH: usize = GRID_SIZE * 2;
const INSTRUCTIONS_LENGTH: usize = 1000;
const INSTRUCTIONS_COUNT: usize = 20;

const INPUT: (
    (
        [[BlockType; EXPANDED_GRID_WIDTH]; GRID_SIZE],
        (usize, usize),
    ),
    [InstructionType; INSTRUCTIONS_COUNT * INSTRUCTIONS_LENGTH],
) = {
    let split = include_str!("input.txt").split_at(GRID_SIZE * (GRID_SIZE + 1));
    let grid_str = split.0.trim_ascii().as_bytes();

    let mut grid = [[BlockType::Empty; EXPANDED_GRID_WIDTH]; GRID_SIZE];
    let mut i = 0;
    let mut newlines = 0;
    let mut robot_pos = (0usize, 0usize);
    while i < grid_str.len() {
        if grid_str[i] == b'\n' {
            newlines += 1;
            i += 1;
            continue;
        };

        let row = (i - newlines) / GRID_SIZE;
        let col = i - newlines - (row * GRID_SIZE);
        (grid[row][col * 2], grid[row][(col * 2) + 1]) = match grid_str[i] {
            b'#' => (BlockType::Wall, BlockType::Wall),
            b'O' => (BlockType::BoxLeft, BlockType::BoxRight),
            b'.' => (BlockType::Empty, BlockType::Empty),
            b'@' => {
                robot_pos = (row, col * 2);
                (BlockType::Robot, BlockType::Empty)
            }
            _ => {
                let mut tmp = [0u8; 4];
                let your_string = (grid_str[i] as char).encode_utf8(&mut tmp);
                panic!("{}", your_string);
            }
        };

        i += 1;
    }

    let instructions_str = split.1.trim_ascii().as_bytes();
    let mut i = 0;
    let mut newlines = 0;
    let mut instructions = [InstructionType::Up; INSTRUCTIONS_COUNT * INSTRUCTIONS_LENGTH];
    while i < instructions_str.len() {
        if instructions_str[i] == b'\n' {
            newlines += 1;
            i += 1;
            continue;
        }

        instructions[i - newlines] = match instructions_str[i] {
            b'^' => InstructionType::Up,
            b'v' => InstructionType::Down,
            b'<' => InstructionType::Left,
            b'>' => InstructionType::Right,
            _ => {
                let mut tmp = [0u8; 4];
                let your_string = (grid_str[i] as char).encode_utf8(&mut tmp);
                panic!("{}", your_string);
            }
        };

        i += 1;
    }

    ((grid, robot_pos), instructions)
};

#[cfg(debug_assertions)]
fn print_grid(grid: &[[BlockType; EXPANDED_GRID_WIDTH]; GRID_SIZE]) {
    for row in 0..GRID_SIZE {
        for col in 0..EXPANDED_GRID_WIDTH {
            print!("{}", match grid[row][col] {
                BlockType::Wall => '#',
                BlockType::BoxLeft => '[',
                BlockType::BoxRight => ']',
                BlockType::Empty => '.',
                BlockType::Robot => '@',
            });
        }
        println!()
    }
}

fn main() {
    let ((mut grid, mut robot_pos), instructions) = INPUT;

    for instruction in instructions {
        debug_assert_eq!(grid[robot_pos.0][robot_pos.1], BlockType::Robot);

        let mut next = match instruction {
            InstructionType::Up => (robot_pos.0 - 1, robot_pos.1),
            InstructionType::Down => (robot_pos.0 + 1, robot_pos.1),
            InstructionType::Left => (robot_pos.0, robot_pos.1 - 1),
            InstructionType::Right => (robot_pos.0, robot_pos.1 + 1),
        };

        match grid[next.0][next.1] {
            // Can't do anything if next is a wall
            BlockType::Wall => continue,
            // If empty, shrimply move there
            BlockType::Empty => {
                grid[robot_pos.0][robot_pos.1] = BlockType::Empty;
                grid[next.0][next.1] = BlockType::Robot;
                robot_pos = next;
            }
            // Wtf
            BlockType::Robot => unreachable!(),
            // Do box pusher logic
            BlockType::BoxLeft | BlockType::BoxRight => {
                #[cfg(debug_assertions)] {
                    println!("Executing instruction {:?} on grid:", instruction);
                    print_grid(&grid);
                }

                let original_box = next;
                let is_horizontal =
                    matches!(instruction, InstructionType::Left | InstructionType::Right);
                let mut failed = true;

                if is_horizontal {
                    loop {
                        next = match instruction {
                            InstructionType::Left => (next.0, next.1 - 2),
                            InstructionType::Right => (next.0, next.1 + 2),
                            _ => unreachable!(),
                        };

                        match grid[next.0][next.1] {
                            BlockType::Wall => break,
                            BlockType::BoxLeft | BlockType::BoxRight => continue,
                            BlockType::Empty => {
                                failed = false;
                                break;
                            }
                            BlockType::Robot => unreachable!(),
                        }
                    }

                    if !failed {
                        // Set end of box chain to opposite of original
                        grid[next.0][next.1] = match grid[original_box.0][original_box.1] {
                            BlockType::BoxLeft => BlockType::BoxRight,
                            BlockType::BoxRight => BlockType::BoxLeft,
                            _ => unreachable!(),
                        };
                        // Move robot to start of box chain
                        grid[robot_pos.0][robot_pos.1] = BlockType::Empty;
                        grid[original_box.0][original_box.1] = BlockType::Robot;
                        robot_pos = original_box;
                        // Set everything in-between to the opposite
                        let range = if next.1 > (original_box.1) {
                            (original_box.1 + 1)..=(next.1 - 1)
                        } else {
                            (next.1 + 1)..=(original_box.1 - 1)
                        };
                        for i in range {
                            grid[next.0][i] = match grid[next.0][i] {
                                BlockType::BoxLeft => BlockType::BoxRight,
                                BlockType::BoxRight => BlockType::BoxLeft,
                                _ => unreachable!(),
                            };
                        }
                    }
                } else {
                    // List of boxes to move up/down depending on the instruction
                    // Front = Move last
                    // Back  = Move first
                    let mut move_stack = VecDeque::<((usize, usize), (usize, usize))>::new();

                    let mut queue = VecDeque::<((usize, usize), (usize, usize))>::new();
                    // Initialize queue with first box
                    queue.push_back(match grid[next.0][next.1] {
                        BlockType::BoxLeft => (next, (next.0, next.1 + 1)),
                        BlockType::BoxRight => ((next.0, next.1 - 1), next),
                        _ => unreachable!(),
                    });
                    // Iterate adding more onto queue until we run into a problem
                    loop {
                        let Some((box_left_half, box_right_half)) = queue.pop_front() else {
                            failed = false;
                            break;
                        };

                        // Check left side
                        let next_left = match instruction {
                            InstructionType::Up => (box_left_half.0 - 1, box_left_half.1),
                            InstructionType::Down => (box_left_half.0 + 1, box_left_half.1),
                            _ => unreachable!(),
                        };
                        let also_check_right = match grid[next_left.0][next_left.1] {
                            // If we hit a wall the whole thing is failed and we can give up
                            BlockType::Wall => break,
                            BlockType::BoxLeft => {
                                // Add next box that needs checking
                                queue.push_back((next_left, (next_left.0, next_left.1 + 1)));
                                // If left is touching a left box then right is touching the same box
                                false
                            },
                            // If left is touching a right box then it might fork
                            BlockType::BoxRight => {
                                // Add next box that needs checking
                                queue.push_back(((next_left.0, next_left.1 - 1), next_left));
                                // If left is touching a right box then right is separate
                                true
                            },
                            BlockType::Empty => true, // Let flow pass to right check
                            BlockType::Robot => unreachable!(),
                        };

                        if also_check_right {
                            let next_right = match instruction {
                                InstructionType::Up => (box_right_half.0 - 1, box_right_half.1),
                                InstructionType::Down => (box_right_half.0 + 1, box_right_half.1),
                                _ => unreachable!(),
                            };
                            match grid[next_right.0][next_right.1] {
                                // If we hit a wall the whole thing is failed and we can give up
                                BlockType::Wall => break,
                                BlockType::BoxLeft => queue.push_back((next_right, (next_right.0, next_right.1 + 1))),
                                BlockType::BoxRight => unreachable!(),
                                BlockType::Empty => (), // Let flow pass to right check
                                BlockType::Robot => unreachable!(),
                            }
                        }

                        // If we got here, the paths look fine and we haven't given up, so note
                        // this box as a box to move (if we don't fail before the end)
                        move_stack.push_back((box_left_half, box_right_half));
                    }

                    if !failed {
                        // Now we have finished and all in move_stack need moved
                        while let Some((to_move_left, to_move_right)) = move_stack.pop_back() {
                            // Set box to empty
                            grid[to_move_left.0][to_move_left.1] = BlockType::Empty;
                            grid[to_move_right.0][to_move_right.1] = BlockType::Empty;
                            // Set above/below to box
                            let (new_left, new_right) = match instruction {
                                InstructionType::Up => ((to_move_left.0 - 1, to_move_left.1), (to_move_right.0 - 1, to_move_right.1)),
                                InstructionType::Down => ((to_move_left.0 + 1, to_move_left.1), (to_move_right.0 + 1, to_move_right.1)),
                                _ => unreachable!(),
                            };
                            grid[new_left.0][new_left.1] = BlockType::BoxLeft;
                            grid[new_right.0][new_right.1] = BlockType::BoxRight;
                        }
                        
                        // Move the robot to next :3
                        // The mission, the nightmares, they're finally over
                        grid[next.0][next.1] = BlockType::Robot;
                        grid[robot_pos.0][robot_pos.1] = BlockType::Empty;
                        robot_pos = next;
                    }
                }
                
                #[cfg(debug_assertions)] {
                    println!("AFTER:");
                    print_grid(&grid);
                    println!("\n");
                }
            }
        }
        debug_assert_eq!(grid[robot_pos.0][robot_pos.1], BlockType::Robot);
    }

    let mut sum = 0_usize;
    for row in 0..GRID_SIZE {
        for col in 0..EXPANDED_GRID_WIDTH {
            if grid[row][col] != BlockType::BoxLeft {
                continue;
            }
            sum += 100 * row + col;
        }
    }

    println!("Result: {sum}")
}
