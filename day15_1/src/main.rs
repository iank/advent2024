use grid::Grid;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_input(path: &Path) -> Result<(Grid<char>, Vec<char>), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut reading_warehouse = true;
    let mut warehouse_rows: Vec<Vec<char>> = vec![];
    let mut moves: Vec<char> = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            reading_warehouse = false;
            continue;
        }

        if reading_warehouse {
            warehouse_rows.push(line.chars().collect());
        } else {
            moves.extend(line.chars());
        }
    }

    // Construct warehouse grid
    let max_cols = warehouse_rows[0].len();
    let flat_vec: Vec<char> = warehouse_rows.into_iter().flatten().collect();
    let warehouse = Grid::from_vec(flat_vec, max_cols);

    Ok((warehouse, moves))
}

fn find_robot(warehouse: &Grid<char>) -> (usize, usize) {
    for row in 0..warehouse.rows() {
        for col in 0..warehouse.cols() {
            if warehouse[(row, col)] == '@' {
                return (row, col);
            }
        }
    }

    unreachable!()
}

fn process_move(warehouse: &Grid<char>, m: char, pos: (usize, usize)) -> Grid<char> {
    let mut result = warehouse.clone();

    let (ry, rx) = pos;
    let (dest_y, dest_x) = match m {
        '^' => (ry - 1, rx),
        '>' => (ry, rx + 1),
        '<' => (ry, rx - 1),
        'v' => (ry + 1, rx),
        _ => unreachable!(),
    };

    let me = result[(ry, rx)];

    // Check for wall
    if result[(dest_y, dest_x)] == '#' {
        return result;
    }

    // Check for boulder
    if result[(dest_y, dest_x)] == 'O' {
        result = process_move(warehouse, m, (dest_y, dest_x)); // Try to move the boulder
    }

    // Move if there's a free space
    if result[(dest_y, dest_x)] == '.' {
        result[(dest_y, dest_x)] = me;
        result[(ry, rx)] = '.';
    }

    result
}

fn calculate_gps(warehouse: &Grid<char>) -> usize {
    let mut result = 0;

    for row in 0..warehouse.rows() {
        for col in 0..warehouse.cols() {
            if warehouse[(row, col)] == 'O' {
                result += row * 100 + col;
            }
        }
    }

    result
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let (warehouse, moves) = read_input(file_path)?;

    let mut nw = warehouse.clone();
    for m in moves {
        let robot_pos = find_robot(&nw);
        nw = process_move(&nw, m, robot_pos);
    }

    let gps = calculate_gps(&nw);
    println!("{}", gps);

    Ok(())
}
