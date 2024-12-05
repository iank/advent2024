use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use grid::Grid;

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

fn walk_and_match(grid: &Grid<char>, start_char: char, target: &str) -> usize {
    let directions = [
        (-1, -1), // Up-Left
        (-1, 0),  // Up
        (-1, 1),  // Up-Right
        (0, -1),  // Left
        (0, 1),   // Right
        (1, -1),  // Down-Left
        (1, 0),   // Down
        (1, 1),   // Down-Right
    ];
    let target_chars: Vec<char> = target.chars().collect();

    let mut matches = 0;

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid[(row,col)] == start_char {
                // For each starting character, check all directions
                for &(dx, dy) in &directions {
                    if match_in_direction(grid, row as isize, col as isize, dx, dy, &target_chars) {
                        matches += 1;
                    }
                }
            }
        }
    }

    matches
}

fn match_in_direction(
    grid: &Grid<char>,
    start_row: isize,
    start_col: isize,
    dx: isize,
    dy: isize,
    target: &[char],
) -> bool {
    for (i, &ch) in target.iter().enumerate() {
        let new_row = start_row + i as isize * dx;
        let new_col = start_col + i as isize * dy;

        // Check boundaries
        if new_row < 0
            || new_row >= grid.rows() as isize
            || new_col < 0
            || new_col >= grid.cols() as isize
        {
            return false;
        }

        if grid[(new_row as usize, new_col as usize)] != ch {
            return false;
        }
    }
    true
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let grid = read_grid(file_path)?;

    println!("{}", walk_and_match(&grid, 'X', "XMAS"));

    Ok(())
}
