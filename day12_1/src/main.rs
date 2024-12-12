use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

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

fn region_neighbors(garden: &Garden, idx: (isize, isize), plant: char) -> Vec<(isize, isize)> {
    let mut neighbors = vec![];

    let neighbor_idxs = vec![
        (idx.0 + 1, idx.1    ),
        (idx.0 - 1, idx.1    ),
        (idx.0    , idx.1 + 1),
        (idx.0    , idx.1 - 1),
    ];

    for ni in neighbor_idxs {
        if let Some(&v) = garden.get(&ni) {
            if v == plant {
                neighbors.push(ni);
            }
        }
    }

    neighbors
}

fn region_cost(garden: &mut Garden) -> u64 {
    let mut area = 0;
    let mut perimeter = 0;

    let mut to_visit = HashSet::new();
    let mut visited = HashSet::new();
    to_visit.insert(*(garden.keys().next().unwrap()));

    while to_visit.len() > 0 {
        let current_idx = *(to_visit.iter().next().unwrap());
        let current_plant = *(garden.get(&current_idx).unwrap());

        to_visit.remove(&current_idx);
        visited.insert(current_idx);

        let neighbors = region_neighbors(garden, current_idx, current_plant);

        area += 1;
        perimeter += 4 - neighbors.len() as u64;
        to_visit.extend(neighbors.into_iter().filter(|x| !visited.contains(x)));
    }

    // remove visited from garden
    for i in &visited {
        garden.remove(i);
    }

    area * perimeter
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
