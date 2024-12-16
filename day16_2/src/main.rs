use grid::Grid;
use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use queues::{queue, Queue, IsQueue};

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

#[derive(Clone)]
struct SearchPoint {
    position: (usize, usize),
    direction: (isize, isize),
    score: usize,
    path: HashSet<(usize, usize)>,
}

#[allow(dead_code)]
fn print_grid(maze: &Grid<char>, c: &SearchPoint) {
    print!("\x1B[2J\x1B[1;1H");

    for row in 0..maze.rows() {
        for col in 0..maze.cols() {
            if c.path.contains(&(row, col)) {
                print!("X");
            } else if c.position == (row, col) {
                print!("@");
            } else {
                print!("{}", maze[(row,col)]);
            }
        }
        println!("");
    }

    //thread::sleep(time::Duration::from_millis(10));
}

fn bfs_search_maze(
    maze: &Grid<char>,
    start_position: (usize, usize),
    start_direction: (isize, isize),
) -> Vec<(HashSet<(usize, usize)>, usize)> {
    let mut q: Queue<SearchPoint> = queue![];

    let mut global_min_score = usize::MAX;
    let mut global_visited: HashMap<(usize, usize), usize> = HashMap::new();

    let mut results = vec![];

    // Add starting point
    q.add(SearchPoint {
        position: start_position,
        direction: start_direction,
        score: 0,
        path: HashSet::new(),
    }).unwrap();

    // BFS
    while q.size() > 0 {
        let current = q.remove().unwrap();

        // If we've been here before via a better path, this is a dead end
        if global_visited.contains_key(&current.position) {
            // Boy do I hate this...
            if current.score > *global_visited.get(&current.position).unwrap() + 1500 {
                continue;
            }
        }
        global_visited.insert(current.position, current.score);

        //print_grid(maze, &current);

        if current.score > global_min_score {
            continue;
        }

        if maze[current.position] == 'E' {
            if current.score < global_min_score {
                global_min_score = current.score;
                println!("{}", global_min_score);
            }

            let mut path_next = current.path.clone();
            path_next.insert(current.position);
            results.push((path_next, current.score));
        }

        let mut new_directions_and_scores = vec![];
        new_directions_and_scores.push((current.direction, 1));

        let (turn_left, turn_right) = match current.direction {
            (0, 1) => ((-1, 0), (1, 0)),
            (0, -1) => ((1, 0), (-1, 0)),
            (1, 0) => ((0, -1), (0, 1)),
            (-1, 0) => ((0, 1), (0, -1)),
            _ => unreachable!(),
        };

        new_directions_and_scores.push((turn_left, 1001));
        new_directions_and_scores.push((turn_right, 1001));

        for (direction_next, score_inc) in new_directions_and_scores {
            let mut path_next = current.path.clone();
            path_next.insert(current.position);

            let position_next = (
                (current.position.0 as isize + direction_next.0) as usize,
                (current.position.1 as isize + direction_next.1) as usize,
            );

            if path_next.contains(&position_next) {
                continue;
            }

            if maze[position_next] == '#' {
                continue;
            }

            q.add(SearchPoint {
                position: position_next,
                direction: direction_next,
                score: current.score + score_inc,
                path: path_next,
           }).unwrap();
        }
    }

    results
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

    let solutions = bfs_search_maze(&grid, start_position, (0, 1));

    let min_score = solutions.iter().map(|s| s.1).min().unwrap();

    let mut best_tiles = HashSet::new();
    for soln in solutions {
        if soln.1 == min_score {
            best_tiles.extend(soln.0);
        }
    }

    println!("{}", best_tiles.len());

    Ok(())
}
