use grid::Grid;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn expand_line(line: Vec<char>) -> Vec<char> {
    let mut result = vec![];
    for c in line {
        result.extend(match c {
            '#' => vec!['#', '#'],
            'O' => vec!['[', ']'],
            '.' => vec!['.', '.'],
            '@' => vec!['@', '.'],
            _ => unreachable!(),
        });
    }

    result
}

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
            let expanded = expand_line(line.chars().collect());
            warehouse_rows.push(expanded);
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
    let (ry, rx) = pos;
    if warehouse[(ry, rx)] == '@' {
        return move_robot(warehouse, m, pos);
    } else if (warehouse[(ry, rx)] == '[' || warehouse[(ry, rx)] == ']') && (m == '>' || m == '<') {
        return move_boulder_horiz(warehouse, m, pos);
    } else if (warehouse[(ry, rx)] == '[' || warehouse[(ry, rx)] == ']') && (m == 'v' || m == '^') {
        return move_boulder_vert(warehouse, m, pos);
    } else {
        unreachable!();
    }
}

// if moving robot
// if destination is wall, return
// if destination is boulder, try to move boulder
// if destination is clear, move
fn move_robot(warehouse: &Grid<char>, m: char, pos: (usize, usize)) -> Grid<char> {
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
    if result[(dest_y, dest_x)] == '[' || result[(dest_y, dest_x)] == ']' {
        result = process_move(warehouse, m, (dest_y, dest_x)); // Try to move the boulder
    }

    // Move if there's a free space
    if result[(dest_y, dest_x)] == '.' {
        result[(dest_y, dest_x)] = me;
        result[(ry, rx)] = '.';
    }

    result
}

// if moving boulder horizontally look at two spots out
// if destination is wall return
// if destination is boulder, try to move boulder
// if destination is clear, move
fn move_boulder_horiz(warehouse: &Grid<char>, m: char, pos: (usize, usize)) -> Grid<char> {
    let mut result = warehouse.clone();

    assert!((result[pos] == '[' && m == '>') || (result[pos] == ']' && m == '<'));

    let (ry, rx) = pos;
    let (dest_y, dest_x) = match m {
        '>' => (ry, rx + 2),
        '<' => (ry, rx - 2),
        _ => unreachable!(),
    };

    // Check for wall
    if result[(dest_y, dest_x)] == '#' {
        return result;
    }

    // Check for boulder
    if result[(dest_y, dest_x)] == '[' || result[(dest_y, dest_x)] == ']' {
        result = process_move(warehouse, m, (dest_y, dest_x)); // Try to move the boulder
    }

    // Move if there's a free space
    if result[(dest_y, dest_x)] == '.' {
        if m == '>' {
            result[(dest_y, dest_x)] = ']';
            result[(dest_y, dest_x - 1)] = '[';
            result[(dest_y, dest_x - 2)] = '.';
        }
        if m == '<' {
            result[(dest_y, dest_x)] = '[';
            result[(dest_y, dest_x + 1)] = ']';
            result[(dest_y, dest_x + 2)] = '.';
        }
    }

    result
}

// if moving boulder vertically look at both destinations
// if either is wall, return
// if either is boulder, try to move boulder
// if both are clear, move
fn move_boulder_vert(warehouse: &Grid<char>, m: char, pos: (usize, usize)) -> Grid<char> {
    let mut result = warehouse.clone();

    assert!((result[pos] == '[' || result[pos] == ']') && (m == '^' || m == 'v'));

    let source1 = pos;
    let source2 = match result[pos] {
        '[' => (pos.0, pos.1 + 1),
        ']' => (pos.0, pos.1 - 1),
        _ => unreachable!(),
    };

    let (dest1, dest2) = match m {
        '^' => ((source1.0 - 1, source1.1), (source2.0 - 1, source2.1)),
        'v' => ((source1.0 + 1, source1.1), (source2.0 + 1, source2.1)),
        _ => unreachable!(),
    };

    // Check for wall
    if result[dest1] == '#' || result[dest2] == '#' {
        return result;
    }

    if result[dest1] == '[' || result[dest1] == ']' {
        result = process_move(&result, m, dest1);
    }
    if result[dest2] == '[' || result[dest2] == ']' {
        result = process_move(&result, m, dest2);
    }

    if result[dest1] == '.' && result[dest2] == '.' {
        result[dest1] = result[source1];
        result[dest2] = result[source2];
        result[source1] = '.';
        result[source2] = '.';
    } else {
        return warehouse.clone();
    }

    result
}

fn calculate_gps(warehouse: &Grid<char>) -> usize {
    let mut result = 0;

    for row in 0..warehouse.rows() {
        for col in 0..warehouse.cols() {
            if warehouse[(row, col)] == '[' {
                result += row * 100 + col;
            }
        }
    }

    result
}

#[allow(dead_code)]
fn print_warehouse(warehouse: &Grid<char>) {
    for row in 0..warehouse.rows() {
        for col in 0..warehouse.cols() {
            print!("{}", warehouse[(row, col)]);
        }
        println!("");
    }
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
    //    print_warehouse(&nw);
    //    println!("");

    for m in moves {
        //        println!("{}", m);
        let robot_pos = find_robot(&nw);
        nw = process_move(&nw, m, robot_pos);

        //        print_warehouse(&nw);
        //        println!("");
    }

    let gps = calculate_gps(&nw);
    println!("{}", gps);

    Ok(())
}
