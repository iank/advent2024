use grid::Grid;
use queues::{queue, IsQueue, Queue};
use std::cmp::min;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::thread;
use std::time;

fn read_input(path: &Path) -> Result<Vec<(usize, usize)>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut bytes: Vec<(usize, usize)> = vec![];

    for line in reader.lines() {
        let line = line?;

        let byte = line.as_str().split(",").collect::<Vec<&str>>();
        bytes.push((
            byte[0].parse::<usize>().unwrap(),
            byte[1].parse::<usize>().unwrap(),
        ));
    }

    Ok(bytes)
}

fn build_maze(bytes: &Vec<(usize, usize)>, n: usize, b: usize) -> Grid<char> {
    let mut maze: Grid<char> = Grid::new(n, n);
    for row in 0..maze.rows() {
        for col in 0..maze.cols() {
            maze[(row, col)] = '.';
        }
    }

    for i in 0..min(bytes.len(), b) {
        let (col, row) = bytes[i];
        maze[(row, col)] = '#';
    }

    maze
}

#[derive(Clone)]
struct SearchPoint {
    position: (usize, usize),
    depth: usize,
}

fn calc_new_positions(p: (usize, usize), n: usize) -> Vec<(usize, usize)> {
    let offsets = vec![(1, 0), (0, 1), (0, -1), (-1, 0)];

    let mut results = vec![];

    for offset in offsets {
        let new_position = (p.0 as isize + offset.0, p.1 as isize + offset.1);
        if new_position.0 < 0
            || new_position.1 < 0
            || new_position.0 >= n as isize
            || new_position.1 >= n as isize
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
    n: usize,
) -> Option<usize> {
    let mut q: Queue<SearchPoint> = queue![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    q.add(SearchPoint {
        position: start,
        depth: 0,
    })
    .unwrap();

    while q.size() > 0 {
        let current = q.remove().unwrap();

        if current.position == exit {
            return Some(current.depth);
        }

        if visited.contains(&current.position) {
            continue;
        }

        visited.insert(current.position);

        //print_grid(maze, &visited);
        let new_positions = calc_new_positions(current.position, n);

        for np in new_positions {
            if maze[np] == '#' {
                continue;
            }
            q.add(SearchPoint {
                position: np,
                depth: current.depth + 1,
            })
            .unwrap();
        }
    }

    None
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <file_path> <size>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let bytes = read_input(file_path)?;

    let n = args[2].parse::<usize>().unwrap();

    let start = (0, 0);
    let exit = (n - 1, n - 1);

    for b in 0..bytes.len() {
        let maze = build_maze(&bytes, n, b);
        //    println!("{:#?}", maze);
        let shortest_path_len = bfs_search_maze(&maze, start, exit, n);
        if shortest_path_len.is_none() {
            println!("{:?}", bytes[b - 1]);
            return Ok(());
        }
    }

    Ok(())
}
