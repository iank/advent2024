use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

type Garden = HashMap<(isize, isize), char>;

fn read_garden(path: &Path) -> Result<Garden, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut garden: Garden = HashMap::new();

    for (row, line) in reader.lines().enumerate() {
        let line = line?;
        for (col, plant) in line.chars().enumerate() {
            garden.insert((row as isize, col as isize), plant);
        }
    }

    Ok(garden)
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Also return *missing* neighbors
fn region_neighbors(
    garden: &Garden,
    idx: (isize, isize),
    plant: char,
) -> (Vec<(isize, isize)>, Vec<Direction>) {
    let mut neighbors = vec![];
    let mut missing_neighbors = vec![];

    let neighbor_idxs = vec![
        ((idx.0 + 1, idx.1), Direction::South),
        ((idx.0 - 1, idx.1), Direction::North),
        ((idx.0, idx.1 + 1), Direction::East),
        ((idx.0, idx.1 - 1), Direction::West),
    ];

    for ni in neighbor_idxs {
        if let Some(&v) = garden.get(&ni.0) {
            if v == plant {
                neighbors.push(ni.0);
            } else {
                missing_neighbors.push(ni.1);
            }
        } else {
            missing_neighbors.push(ni.1);
        }
    }

    (neighbors, missing_neighbors)
}

fn condense_sides(nums: &Vec<isize>) -> usize {
    let mut numbers = nums.clone();
    numbers.sort_unstable(); // Sort the numbers first

    numbers
        .iter()
        .fold(Vec::new(), |mut acc, &x| {
            match acc.last_mut() {
                Some((_start, end)) if *end + 1 == x => *end = x,
                _ => acc.push((x, x)),
            };
            acc
        })
        .len()
}

fn count_sides(sides: Vec<((isize, isize), Direction)>) -> usize {
    // sides is a vector of fence segments
    let mut nsides = 0;

    let rowmax = sides
        .iter()
        .max_by_key(|&((y, _x), _)| y)
        .map(|((y, _x), _)| y)
        .unwrap();
    for row in 0..*rowmax+1 {
        let thisrow_north = sides
            .iter()
            .filter(|((y, _x), direction)| *direction == Direction::North && *y == row)
            .map(|((_y, x), _)| *x)
            .collect();
        let thisrow_south = sides
            .iter()
            .filter(|((y, _x), direction)| *direction == Direction::South && *y == row)
            .map(|((_y, x), _)| *x)
            .collect();
        let nsides_north = condense_sides(&thisrow_north);
        nsides += nsides_north;

        let nsides_south = condense_sides(&thisrow_south);
        nsides += nsides_south;
    }

    let colmax = sides
        .iter()
        .max_by_key(|&((_y, x), _)| x)
        .map(|((_y, x), _)| x)
        .unwrap();
    for col in 0..*colmax+1 {
        let thiscol_east = sides
            .iter()
            .filter(|((_y, x), direction)| *direction == Direction::East && *x == col)
            .map(|((y, _x), _)| *y)
            .collect();
        let thiscol_west = sides
            .iter()
            .filter(|((_y, x), direction)| *direction == Direction::West && *x == col)
            .map(|((y, _x), _)| *y)
            .collect();
        let nsides_east = condense_sides(&thiscol_east);
        nsides += nsides_east;

        let nsides_west = condense_sides(&thiscol_west);
        nsides += nsides_west;
    }

    nsides
}

fn region_cost(garden: &mut Garden) -> u64 {
    let mut area = 0;
    let mut sides = vec![];

    let mut to_visit = HashSet::new();
    let mut visited = HashSet::new();
    to_visit.insert(*(garden.keys().next().unwrap()));

    while to_visit.len() > 0 {
        let current_idx = *(to_visit.iter().next().unwrap());
        let current_plant = *(garden.get(&current_idx).unwrap());

        to_visit.remove(&current_idx);
        visited.insert(current_idx);

        let (neighbors, missing_neighbors) = region_neighbors(garden, current_idx, current_plant);

        area += 1;
        for mn in missing_neighbors {
            sides.push((current_idx, mn));
        }
        to_visit.extend(neighbors.into_iter().filter(|x| !visited.contains(x)));
    }

    // remove visited from garden
    for i in &visited {
        garden.remove(i);
    }

    let nsides = count_sides(sides) as u64;

    area * nsides
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let mut garden = read_garden(file_path)?;

    let mut total_cost = 0;
    while garden.len() > 0 {
        total_cost += region_cost(&mut garden);
    }

    println!("{}", total_cost);

    Ok(())
}
