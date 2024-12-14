use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

fn read_robots(path: &Path) -> Result<Vec<Robot>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut result = vec![];

    for line in reader.lines() {
        let line = line?;

        let caps = re.captures(line.as_str()).unwrap();

        let robot = Robot {
            px: caps.get(1).unwrap().as_str().parse().unwrap(),
            py: caps.get(2).unwrap().as_str().parse().unwrap(),
            vx: caps.get(3).unwrap().as_str().parse().unwrap(),
            vy: caps.get(4).unwrap().as_str().parse().unwrap(),
        };

        result.push(robot);
    }

    Ok(result)
}

fn step_robot(robot: Robot, width: i32, height: i32) -> Robot {
    let mut r = robot.clone();
    r.px = (robot.px + robot.vx + width) % width;
    r.py = (robot.py + robot.vy + height) % height;

    return r;
}

fn print_grid(robots: &Vec<Robot>, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            let n_robots = robots.iter().filter(|r| r.px == x && r.py == y).collect::<Vec<_>>().len();
            if n_robots == 0 {
                print!(".");
            } else {
                print!("{}", n_robots);
            }
        }
        println!("");
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <file_path> <width> <height>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let width: i32 = args[2].parse().unwrap();
    let height: i32 = args[3].parse().unwrap();

    let mut robots = read_robots(&file_path)?;
    let og_robots = robots.clone();

    // I just printed them all out (up to the cycle length) and searched for a long string of '1's,
    // which was a guess about what the tree might look like.
    for step in 1..6772 {
        robots = robots.into_iter().map(|r| step_robot(r, width, height)).collect();
        if step == 6771 {
            print_grid(&robots, width, height);
        }
    }

    Ok(())
}
