use grid::Grid;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Antenna {
    frequency: char,
    row: usize,
    col: usize,
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Antinode(isize, isize);

fn read_antennas(path: &Path) -> Result<(Vec<Antenna>, usize, usize), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        rows.push(line.chars().collect());
    }

    let max_cols = rows[0].len();
    let flat_vec: Vec<char> = rows.into_iter().flatten().collect();
    let grid = Grid::from_vec(flat_vec, max_cols);

    let mut antennas = vec![];

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid[(row, col)] != '.' {
                let antenna = Antenna {
                    frequency: grid[(row, col)],
                    row: row,
                    col: col,
                };
                antennas.push(antenna);
            }
        }
    }

    Ok((antennas, grid.rows(), grid.cols()))
}

// Find all antenna pairs
fn get_antenna_pairs(antennas: Vec<Antenna>) -> Vec<(Antenna, Antenna)> {
    let mut result = vec![];

    // get list of frequencies
    let frequencies: HashSet<_> = antennas.iter().map(|x| x.frequency).collect();

    // for each frequency
    for frequency in frequencies {
        // get all antennas with that frequency
        let antenna_list: Vec<_> = antennas
            .iter()
            .filter(|x| x.frequency == frequency)
            .collect();

        // cartesian set of all of those
        let antenna_pairs: Vec<(Antenna, Antenna)> = antenna_list
            .iter()
            .cloned()
            .flat_map(|a| {
                antenna_list
                    .iter()
                    .cloned()
                    .filter(move |b| a != *b)
                    .map(move |b| (a.clone(), b.clone()))
            })
            .collect();

        // push to list
        for pair in antenna_pairs {
            result.push(pair);
        }
    }

    result
}

// Find antinodes for each antenna pair
fn calc_antinodes(pair: (Antenna, Antenna)) -> (Antinode, Antinode) {
    let (a, b) = pair;
    let offset: (isize, isize) = (
        a.row as isize - b.row as isize,
        a.col as isize - b.col as isize,
    );

    let antinode1 = Antinode(a.row as isize + offset.0, a.col as isize + offset.1);
    let antinode2 = Antinode(b.row as isize - offset.0, b.col as isize - offset.1);

    /*    println!("Antinodes:");
    println!("{:#?}, {:#?}: {:#?}, {:#?}", a, b, antinode1, antinode2);
    println!("");*/

    (antinode1, antinode2)
}

fn in_bounds(antinode: &Antinode, rows: usize, cols: usize) -> bool {
    if antinode.0 < 0 || antinode.1 < 0 {
        return false;
    }

    if antinode.0 >= rows as isize || antinode.1 >= cols as isize {
        return false;
    }

    true
}

// Count unique antinodes
fn count_unique_antinodes(antennas: Vec<Antenna>, rows: usize, cols: usize) -> usize {
    let antenna_pairs = get_antenna_pairs(antennas);
    let mut all_antinodes = HashSet::new();

    for pair in antenna_pairs {
        let antinodes = calc_antinodes(pair);
        if in_bounds(&antinodes.0, rows, cols) {
            all_antinodes.insert(antinodes.0);
        }
        if in_bounds(&antinodes.1, rows, cols) {
            all_antinodes.insert(antinodes.1);
        }
    }

    all_antinodes.len()
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let (antennas, rows, cols) = read_antennas(file_path)?;
    let antinode_count = count_unique_antinodes(antennas, rows, cols);

    println!("{}", antinode_count);

    Ok(())
}
