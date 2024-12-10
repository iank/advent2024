use grid::Grid;
use queues::{queue, IsQueue, Queue};
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_grid(path: &Path) -> Result<Grid<u8>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        rows.push(line.chars().collect());
    }

    let max_cols = rows[0].len();
    let flat_vec: Vec<u8> = rows
        .into_iter()
        .flatten()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect();

    Ok(Grid::from_vec(flat_vec, max_cols))
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point(usize, usize);

fn find_trailheads(grid: &Grid<u8>) -> Vec<Point> {
    let mut result = vec![];
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid[(row, col)] == 0 {
                result.push(Point(row, col));
            }
        }
    }

    result
}

// find all in-bounds neighbors w/ value one more than point
fn eligible_neighbors(grid: &Grid<u8>, point: Point) -> Vec<Point> {
    let row = point.0 as isize;
    let col = point.1 as isize;

    let neighbors = vec![
        (row + 1, col),
        (row - 1, col),
        (row, col + 1),
        (row, col - 1),
    ];

    let mut result = vec![];

    for n in neighbors {
        // Bounds check
        if n.0 < 0 || n.1 < 0 {
            continue;
        }
        if n.0 >= grid.rows() as isize || n.1 >= grid.cols() as isize {
            continue;
        }

        // Check for gradual increase
        if grid[(n.0 as usize, n.1 as usize)] != grid[(point.0, point.1)] + 1 {
            continue;
        }

        result.push(Point(n.0 as usize, n.1 as usize));
    }

    result
}

fn trailhead_score(grid: &Grid<u8>, trailhead: Point) -> usize {
    let mut q: Queue<Point> = queue![];
    q.add(trailhead).unwrap();

    let mut summits = HashSet::new();

    while q.size() > 0 {
        let item = q.remove().unwrap();

        if grid[(item.0, item.1)] == 9 {
            // Reached summit
            summits.insert(item);
        } else {
            // check neighbors, add to queue
            for neighbor in eligible_neighbors(grid, item) {
                q.add(neighbor).unwrap();
            }
        }
    }

    summits.len()
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let grid = read_grid(file_path)?;
    let trailheads = find_trailheads(&grid);
    let scores: Vec<usize> = trailheads
        .iter()
        .map(|th| trailhead_score(&grid, *th))
        .collect();

    println!("{}", scores.iter().sum::<usize>());

    Ok(())
}
