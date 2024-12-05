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

fn x_mas_count(grid: &Grid<char>) -> usize {
    let mut matches = 0;

    for row in 1..grid.rows()-1 {
        for col in 1..grid.cols()-1 {
            if grid[(row,col)] == 'A' {
                // Check for one of four possible X-MAS's
                if (grid[(row-1,col-1)] == 'M' &&
                    grid[(row+1,col-1)] == 'S' &&
                    grid[(row-1,col+1)] == 'M' &&
                    grid[(row+1,col+1)] == 'S') ||
                   (grid[(row-1,col-1)] == 'M' &&
                    grid[(row+1,col-1)] == 'M' &&
                    grid[(row-1,col+1)] == 'S' &&
                    grid[(row+1,col+1)] == 'S') ||
                   (grid[(row-1,col-1)] == 'S' &&
                    grid[(row+1,col-1)] == 'S' &&
                    grid[(row-1,col+1)] == 'M' &&
                    grid[(row+1,col+1)] == 'M') ||
                   (grid[(row-1,col-1)] == 'S' &&
                    grid[(row+1,col-1)] == 'M' &&
                    grid[(row-1,col+1)] == 'S' &&
                    grid[(row+1,col+1)] == 'M') {
                       matches += 1;
                }
            }
        }
    }

    matches
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let grid = read_grid(file_path)?;

    println!("{}", x_mas_count(&grid));

    Ok(())
}
