use grid::Grid;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashSet;

fn read_grid(path: &Path) -> Result<Grid<char>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        rows.push(line.chars().collect());
    }

    let max_cols = rows[0].len();
    let flat_vec: Vec<char> = rows.into_iter().flatten().collect();

    Ok(Grid::from_vec(flat_vec, max_cols))
}

fn find_guard(grid: &Grid<char>) -> Option<(usize, usize)> {
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid[(row, col)] == '^' {
                return Some((row, col));
            }
        }
    }

    None
}

fn convert_grid(input: &Grid<char>) -> Grid<(char, HashSet<(isize, isize)>)> {
    let mut output = Grid::new(input.rows(), input.cols());

    for row in 0..input.rows() {
        for col in 0..input.cols() {
            output[(row,col)] = (input[(row,col)], HashSet::new());
        }
    }

    output
}

#[allow(dead_code)]
fn print_grid(input: &Grid<(char, HashSet<(isize, isize)>)>, guard_pos: (usize, usize)) {
    for row in 0..input.rows() {
        for col in 0..input.cols() {
            if (row,col) == guard_pos {
                print!("@");
            }
            else {
                let visited = &input[(row,col)].1;
                if visited.contains(&(-1,0)) || visited.contains(&(1,0)) {
                    print!("|");
                }
                else if visited.contains(&(0,-1)) || visited.contains(&(0, 1)) {
                    print!("-");
                }
                else {
                    print!("{}", input[(row,col)].0);
                }
            }
        }
        println!("");
    }

    println!("");
}

fn is_cycle(igrid: &Grid<char>, guard_pos: (usize, usize), new_obstruction: (usize, usize)) -> bool {
    let mut direction = (-1, 0);
    let mut position = guard_pos;

    let mut grid = convert_grid(igrid);
    grid[new_obstruction].0 = '#';

    loop {
        //print_grid(&grid, position);
        // Mark current position w/ travel direction
        // Check next space.
        //   If out of bounds, return false (no cycle)
        //   If '#', turn right but don't move
        //   Would taking the step complete a cycle? return true
        //   Otherwise take the step.

        // Mark current position w/ travel direction
        grid[position].1.insert(direction);

        // Get next position, check for out of bounds
        let next_row = position.0 as isize + direction.0;
        let next_col = position.1 as isize + direction.1;
        if next_col < 0
            || next_col >= grid.cols() as isize
            || next_row < 0
            || next_row >= grid.rows() as isize
        {
            return false;
        }

        let next_position = (next_row as usize, next_col as usize);

        // If next position is an obstruction, turn
        if grid[next_position].0 == '#' {
            direction = match direction {
                (-1, 0) => (0, 1),      // Facing up, turn to face right
                (0, 1) => (1, 0),       // Facing right, turn to face down
                (1, 0) => (0, -1),      // Facing down, turn to face left
                (0, -1) => (-1, 0),     // Facing left, turn to face up
                i => i,                 // This is for the compiler..
            }
        }

        // Otherwise consider taking a step
        else {
            // Would this be a cycle?
            if grid[next_position].1.contains(&direction) {
                return true;
            }

            position = next_position;
        }
    }
}

fn count_cycles(grid: &Grid<char>, guard_pos: (usize, usize)) -> usize {
    let mut cycles = 0;
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            // skip guard position
            if guard_pos == (row, col) {
                continue;
            }

            if is_cycle(&grid, guard_pos, (row, col)) {
                cycles += 1;
            }
        }
    }

    cycles
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let grid = read_grid(file_path)?;

    let guard_pos = find_guard(&grid).unwrap();
    let cycles = count_cycles(&grid, guard_pos);

    println!("{}", cycles);
    Ok(())
}
