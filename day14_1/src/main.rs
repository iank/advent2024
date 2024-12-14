use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
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

fn simulate_robot(robot: &Robot, width: i32, height: i32, steps: i32) -> (i32, i32) {
    let result_x = (robot.px + steps * robot.vx + steps * width) % width;
    let result_y = (robot.py + steps * robot.vy + steps * height) % height;

    return (result_x, result_y);
}

fn safety_factor(positions: Vec<(i32, i32)>, width: i32, height: i32) -> usize {
    // Group by quadrant, discarding middle robots
    // Count robots in each quadrant and multiply quadrant counts

    let mid_x = width / 2;
    let mid_y = height / 2;

    let quadrant_1: Vec<_> = positions
        .iter()
        .filter(|p| p.0 < mid_x && p.1 < mid_y)
        .collect();
    let quadrant_2: Vec<_> = positions
        .iter()
        .filter(|p| p.0 < mid_x && p.1 > mid_y)
        .collect();
    let quadrant_3: Vec<_> = positions
        .iter()
        .filter(|p| p.0 > mid_x && p.1 < mid_y)
        .collect();
    let quadrant_4: Vec<_> = positions
        .iter()
        .filter(|p| p.0 > mid_x && p.1 > mid_y)
        .collect();

    return quadrant_1.len() * quadrant_2.len() * quadrant_3.len() * quadrant_4.len();
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

    let robots = read_robots(&file_path)?;

    let results: Vec<_> = robots
        .iter()
        .map(|r| simulate_robot(r, width, height, 100))
        .collect();
    println!("{}", safety_factor(results, width, height));

    Ok(())
}
