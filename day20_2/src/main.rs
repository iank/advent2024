use grid::Grid;
use queues::{queue, IsQueue, Queue};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::thread;
use std::time;

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
    depth: usize,
    path: Vec<(usize, usize)>,
}

fn calc_new_positions(p: (usize, usize), rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let offsets = vec![(1, 0), (0, 1), (0, -1), (-1, 0)];

    let mut results = vec![];

    for offset in offsets {
        let new_position = (p.0 as isize + offset.0, p.1 as isize + offset.1);
        if new_position.0 < 0
            || new_position.1 < 0
            || new_position.0 >= rows as isize
            || new_position.1 >= cols as isize
        {
            continue;
        }

        results.push((new_position.0 as usize, new_position.1 as usize));
    }

    results
}

#[allow(dead_code)]
fn print_grid(maze: &Grid<char>, visited: &HashSet<(usize, usize)>) {
    print!("\x1B[2J\x1B[1;1H");

    for row in 0..maze.rows() {
        for col in 0..maze.cols() {
            if visited.contains(&(row, col)) {
                print!("O");
            } else {
                print!("{}", maze[(row, col)]);
            }
        }
        println!("");
    }

    thread::sleep(time::Duration::from_millis(10));
}

fn bfs_search_maze(
    maze: &Grid<char>,
    start: (usize, usize),
    exit: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut q: Queue<SearchPoint> = queue![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    q.add(SearchPoint {
        position: start,
        depth: 0,
        path: vec![start],
    })
    .unwrap();

    while q.size() > 0 {
        let current = q.remove().unwrap();

        if current.position == exit {
            return Some(current.path);
        }

        if visited.contains(&current.position) {
            continue;
        }

        visited.insert(current.position);

        //print_grid(maze, &visited);
        let new_positions = calc_new_positions(current.position, maze.rows(), maze.cols());

        for np in new_positions {
            if maze[np] == '#' {
                continue;
            }
            let mut path = current.path.clone();
            path.push(np);
            q.add(SearchPoint {
                position: np,
                depth: current.depth + 1,
                path: path,
            })
            .unwrap();
        }
    }

    None
}

fn find_positions(maze: &Grid<char>, sq: char) -> Vec<(usize, usize)> {
    let mut results = vec![];

    for row in 0..maze.rows() {
        for col in 0..maze.cols() {
            if maze[(row, col)] == sq {
                results.push((row, col));
            }
        }
    }

    results
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    (((a.0 as isize) - (b.0 as isize)).abs() + ((a.1 as isize) - (b.1 as isize)).abs()) as usize
}

fn find_cheat_ends(start: (usize, usize), path: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for p in path {
        if manhattan_distance(*p, start) <= 20 {
            result.push(*p);
        }
    }
    result
}

fn compute_cheats(path: Vec<(usize, usize)>) -> Vec<usize> {
    // 1) turn path into hashmap (key is coord, value is score)

    let mut pathscores: HashMap<(usize, usize), usize> = HashMap::new();
    let mut result = vec![];
    for (i, p) in path.iter().enumerate() {
        pathscores.insert(*p, i);
    }
    for cheat_start in &path {
        let cheat_ends = find_cheat_ends(*cheat_start, &path);
        for cheat_end in cheat_ends {
            let current_score = pathscores.get(&cheat_start).unwrap();
            let new_score = pathscores.get(&cheat_end).unwrap();
            let dist = manhattan_distance(*cheat_start, cheat_end);
            if *new_score > *current_score + dist {
                result.push(new_score - current_score - dist);
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
    let grid = read_grid(file_path)?;
    let start_position = find_positions(&grid, 'S')[0];
    let end_position = find_positions(&grid, 'E')[0];

    let path = bfs_search_maze(&grid, start_position, end_position).unwrap();

    let cheats = compute_cheats(path);
    let n_good_cheats = cheats
        .into_iter()
        .filter(|c| *c >= 100)
        .collect::<Vec<usize>>()
        .len();
    println!("{:?}", n_good_cheats);
    Ok(())
}
