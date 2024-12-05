use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;

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

fn check_rule(update: &Vec<u32>, rule: &(u32, u32)) -> bool {
    let i1 = update.iter().position(|n| *n == rule.0);
    let i2 = update.iter().position(|n| *n == rule.1);

    match (i1, i2) {
        (Some(a), Some(b)) => a < b, // Check rule
        _ => true,                   // One or more pages not in update, pass
    }
}

fn is_invalid_update(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for rule in rules {
        if check_rule(update, rule) == false {
            return true;
        }
    }

    false
}

fn filter_updates(updates: Vec<Vec<u32>>, rules: &Vec<(u32, u32)>) -> Vec<Vec<u32>> {
    updates
        .into_iter()
        .filter(|update| is_invalid_update(update, rules))
        .collect::<Vec<Vec<u32>>>()
}

fn sum_midpoints(updates: Vec<Vec<u32>>) -> u32 {
    let mut result = 0;

    for update in updates {
        assert!(update.len() % 2 == 1);
        result += update[update.len() / 2];
    }

    result
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

    let incorrect_updates = filter_updates(updates, &rules);

    let corrected = incorrect_updates
        .into_iter()
        .map(|x| correct_update(x, &rules))
        .collect::<Vec<Vec<u32>>>();
    println!("{}", sum_midpoints(corrected));

    Ok(())
}
