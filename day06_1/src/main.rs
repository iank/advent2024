use grid::Grid;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

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

fn walk_guard(grid: &mut Grid<char>, guard_pos: (usize, usize)) -> usize {
    let mut direction = (-1, 0);
    let mut position = guard_pos;
    let mut visited = 1; // We start where we start

    loop {
        //println!("{:#?}", grid);
        // Check next space.
        //   If out of bounds, return
        //   If '#', turn right but don't move
        //   If '.' or 'X', mark current as visited and take the step

        let next_row = position.0 as isize + direction.0;
        let next_col = position.1 as isize + direction.1;
        if next_col < 0
            || next_col >= grid.cols() as isize
            || next_row < 0
            || next_row >= grid.rows() as isize
        {
            return visited;
        }

        let next_position = (next_row as usize, next_col as usize);

        if grid[next_position] == '#' {
            direction = match direction {
                (-1, 0) => (0, 1),      // Facing up, turn to face right
                (0, 1) => (1, 0),       // Facing right, turn to face down
                (1, 0) => (0, -1),      // Facing down, turn to face left
                (0, -1) => (-1, 0),     // Facing left, turn to face up
                i => i,                 // This is for the compiler..
            }
        }

        else if grid[next_position] == '.' || grid[next_position] == 'X' {
            if grid[next_position] == '.' {
                visited += 1;
            }

            grid[position] = 'X';
            position = next_position;
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let mut grid = read_grid(file_path)?;

    let guard_pos = find_guard(&grid).unwrap();
    let unique_positions = walk_guard(&mut grid, guard_pos);

    println!("{}", unique_positions);
    Ok(())
}
