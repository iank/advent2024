use grid::Grid;
use std::collections::HashMap;
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

fn search_maze(
    maze: &Grid<char>,
    position: (usize, usize),
    direction: (isize, isize),
    visited: &mut HashMap<(usize, usize), usize>,
    score: usize,
) -> Vec<usize> {
    if maze[position] == 'E' {
        return vec![score];
    }

    let mut new_directions = vec![];
    new_directions.push((direction, 1));

    let (turn_left, turn_right) = match direction {
        (0, 1) => ((-1, 0), (1, 0)),
        (0, -1) => ((1, 0), (-1, 0)),
        (1, 0) => ((0, -1), (0, 1)),
        (-1, 0) => ((0, 1), (0, -1)),
        _ => unreachable!(),
    };

    new_directions.push((turn_left, 1001));
    new_directions.push((turn_right, 1001));

    let mut scores = vec![];
    for (d, score_inc) in new_directions {
        let new_position = (
            (position.0 as isize + d.0) as usize,
            (position.1 as isize + d.1) as usize,
        );
        if visited.contains_key(&new_position) {
            if *visited.get(&new_position).unwrap() < score {
                // We've been here before but it was better
                continue;
            }
            // Otherwise this is worth exploring still
        }
        if maze[new_position] == '#' {
            continue;
        }

        visited.insert(new_position, score + score_inc);

        scores.extend(search_maze(maze, new_position, d, visited, score + score_inc));
    }

    scores
}

fn find_start_position(maze: &Grid<char>) -> (usize, usize) {
    for row in 0..maze.rows() {
        for col in 0..maze.cols() {
            if maze[(row, col)] == 'S' {
                return (row, col);
            }
        }
    }

    unreachable!();
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let grid = read_grid(file_path)?;
    let start_position = find_start_position(&grid);

    let mut visited = HashMap::new();
    let scores = search_maze(&grid, start_position, (0, 1), &mut visited, 0);
    println!("{}", scores.iter().min().unwrap());

    Ok(())
}
