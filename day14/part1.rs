use std::ops::Range;

const INPUT: &str = include_str!("input.txt");
const GRID_WIDTH: i64 = 101;
const GRID_HEIGHT: i64 = 103;
const SECONDS: i64 = 100;

const QUADRANT_1: (Range<i64>, Range<i64>) = (0..(GRID_WIDTH/2), 0..(GRID_HEIGHT/2));
const QUADRANT_2: (Range<i64>, Range<i64>) = ((GRID_WIDTH/2 + 1)..GRID_WIDTH, 0..(GRID_HEIGHT/2));
const QUADRANT_3: (Range<i64>, Range<i64>) = ((GRID_WIDTH/2 + 1)..GRID_WIDTH, (GRID_HEIGHT/2 + 1)..GRID_HEIGHT);
const QUADRANT_4: (Range<i64>, Range<i64>) = (0..(GRID_WIDTH/2), (GRID_HEIGHT/2 + 1)..GRID_HEIGHT);

#[derive(Debug)]
pub struct Robot {
    pub position: (i64, i64),
    pub velocity: (i64, i64),
}

fn main() {
    let robots = INPUT
        .lines()
        .map(|l| l.split_once(" v=").unwrap())
        .map(|(p, v)| {
            (
                p.trim_start_matches("p=").split_once(",").unwrap(),
                v.split_once(",").unwrap(),
            )
        })
        .map(|(p, v)| Robot {
            position: (p.0.parse().unwrap(), p.1.parse().unwrap()),
            velocity: (v.0.parse().unwrap(), v.1.parse().unwrap()),
        });
    
    let mut quadrants = (0u64, 0u64, 0u64, 0u64);

    for mut robot in robots {
        robot.position = (
            ((robot.position.0 + robot.velocity.0 * SECONDS)).rem_euclid(GRID_WIDTH),
            ((robot.position.1 + robot.velocity.1 * SECONDS)).rem_euclid(GRID_HEIGHT),
        );
        
        if QUADRANT_1.0.contains(&robot.position.0) && QUADRANT_1.1.contains(&robot.position.1) {
            quadrants.0 += 1;
        } else if QUADRANT_2.0.contains(&robot.position.0) && QUADRANT_2.1.contains(&robot.position.1) {
            quadrants.1 += 1;
        } else if QUADRANT_3.0.contains(&robot.position.0) && QUADRANT_3.1.contains(&robot.position.1) {
            quadrants.2 += 1;
        } else if QUADRANT_4.0.contains(&robot.position.0) && QUADRANT_4.1.contains(&robot.position.1) {
            quadrants.3 += 1;
        }
    }

    println!("Result: {}", quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3);
}
