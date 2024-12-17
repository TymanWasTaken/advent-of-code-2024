#![feature(const_str_split_at)]

#[derive(Debug, Clone, Copy, PartialEq)]
enum BlockType {
    Wall,
    Box,
    Empty,
    Robot
}

#[derive(Debug, Clone, Copy)]
enum InstructionType {
    Up,
    Down,
    Left,
    Right
}

const GRID_SIZE: usize = 50;
const INSTRUCTIONS_LENGTH: usize = 1000;
const INSTRUCTIONS_COUNT: usize = 20;

const INPUT: (([[BlockType; GRID_SIZE]; GRID_SIZE], (usize, usize)), [InstructionType; INSTRUCTIONS_COUNT * INSTRUCTIONS_LENGTH]) = {
    let split = include_str!("input.txt").split_at(GRID_SIZE * (GRID_SIZE + 1));
    let grid_str = split.0.trim_ascii().as_bytes();

    let mut grid = [[BlockType::Empty; GRID_SIZE]; GRID_SIZE];
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
        grid[row][col] = match grid_str[i] {
            b'#' => BlockType::Wall,
            b'O' => BlockType::Box,
            b'.' => BlockType::Empty,
            b'@' => {
                robot_pos = (row, col);
                BlockType::Robot
            },
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

fn main() {
    let ((mut grid, mut robot_pos), instructions) = INPUT;

    for instruction in instructions {
        let mut next = match instruction {
            InstructionType::Up => (robot_pos.0 - 1, robot_pos.1),
            InstructionType::Down => (robot_pos.0 + 1, robot_pos.1),
            InstructionType::Left => (robot_pos.0, robot_pos.1 - 1),
            InstructionType::Right => (robot_pos.0, robot_pos.1 + 1)
        };
        
        match grid[next.0][next.1] {
            // Can't do anything if next is a wall
            BlockType::Wall => continue,
            // If empty, shrimply move there
            BlockType::Empty => {
                grid[robot_pos.0][robot_pos.1] = BlockType::Empty;
                grid[next.0][next.1] = BlockType::Robot;
                robot_pos = next;
            },
            // Wtf
            BlockType::Robot => unreachable!(),
            // Do box pusher logic
            BlockType::Box => {
                let original_box = next;
                // Loop until we find a place to put the box
                let mut failed = false;
                loop {
                    next = match instruction {
                        InstructionType::Up => (next.0 - 1, next.1),
                        InstructionType::Down => (next.0 + 1, next.1),
                        InstructionType::Left => (next.0, next.1 - 1),
                        InstructionType::Right => (next.0, next.1 + 1)
                    };
                    // Overflow impossible, breaks on wall
                    match grid[next.0][next.1] {
                        BlockType::Wall => {
                            failed = true;
                            break;
                        },
                        BlockType::Box => continue,
                        BlockType::Empty => break,
                        BlockType::Robot => unreachable!(),
                    }
                }
                if !failed {
                    // Move robot to box location
                    grid[original_box.0][original_box.1] = BlockType::Robot;
                    grid[robot_pos.0][robot_pos.1] = BlockType::Empty;
                    robot_pos = original_box;
                    // Put box in empty slot
                    grid[next.0][next.1] = BlockType::Box;
                }
            },
        }
    }

    let mut sum = 0_usize;
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if grid[row][col] != BlockType::Box { continue }
            sum += 100 * row + col;
        }
    }

    println!("Result: {sum}")
}