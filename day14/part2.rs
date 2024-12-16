const INPUT: &str = include_str!("input.txt");
const GRID_WIDTH: i64 = 101;
const GRID_HEIGHT: i64 = 103;

const SQUARE_THRESHOLD: i64 = 4; // 4x4 square

#[derive(Debug)]
pub struct Robot {
    pub position: (i64, i64),
    pub velocity: (i64, i64),
}

fn main() {
    let mut robots = INPUT
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
        })
        .collect::<Vec<_>>();
    
    let mut i = 0u64;
    loop {
        i += 1;
        for robot in robots.iter_mut() {
            robot.position = (
                ((robot.position.0 + robot.velocity.0)).rem_euclid(GRID_WIDTH),
                ((robot.position.1 + robot.velocity.1)).rem_euclid(GRID_HEIGHT),
            );
        }

        // Check for square
        let square = robots.iter().map(|r| r.position).any(
            |(start_x, start_y)|
                (start_x..(start_x + SQUARE_THRESHOLD)).all(
                    |x|
                        (start_y..(start_y + SQUARE_THRESHOLD)).all(
                            |y|
                                robots.iter().any(
                                    |r|
                                        r.position == (x, y)
                                )
                        )
                )
        );
        if square { break; }
    }

    println!("Result: {}", i);
}
