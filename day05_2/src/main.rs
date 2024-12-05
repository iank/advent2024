use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;
use itertools::izip;

fn read_input(path: &Path) -> Result<(Vec<(u32, u32)>, Vec<Vec<u32>>), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut rules: Vec<(u32, u32)> = vec![];
    let mut updates: Vec<Vec<u32>> = vec![];

    let mut first_section = true;

    for line in reader.lines() {
        let line = line?;

        if line == "" {
            first_section = false;
            continue;
        }

        if first_section {
            let ordered_pages = line.as_str().split("|").collect::<Vec<&str>>();
            rules.push((
                ordered_pages[0].parse::<u32>().unwrap(),
                ordered_pages[1].parse::<u32>().unwrap(),
            ))
        } else {
            let update = line.as_str().split(",").collect::<Vec<&str>>();
            updates.push(
                update
                    .iter()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            )
        }
    }

    Ok((rules, updates))
}

fn midpoint(update: Vec<u32>) -> u32 {
    assert!(update.len() % 2 == 1);
    update[update.len() / 2]
}

fn correct_update(update: Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    // Find correct order for this set of pages
    let update_map: HashSet<u32> = update.into_iter().collect();

    let relevant_rules = rules
        .into_iter()
        .filter(|(a, b)| update_map.contains(a) && update_map.contains(b))
        .collect::<Vec<&(u32, u32)>>();

    let g = DiGraphMap::<u32, ()>::from_edges(relevant_rules);
    toposort(&g, None).unwrap()
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let (rules, updates) = read_input(file_path)?;

    let corrected = updates.clone()
        .into_iter()
        .map(|x| correct_update(x, &rules))
        .collect::<Vec<Vec<u32>>>();

    // Sum midpoints of only reordered updates
    let mut midpoint_total = 0;
    for (update_i, corrected_i) in izip!(updates, corrected) {
        if update_i != corrected_i {
            midpoint_total += midpoint(corrected_i);
        }
    }
    println!("{}", midpoint_total);

    Ok(())
}
