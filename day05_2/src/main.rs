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

fn get_complete_order(rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let g = DiGraphMap::<u32, ()>::from_edges(rules);
    toposort(&g, None).unwrap()
}

fn correct_update(update: Vec<u32>, complete_order: &Vec<u32>) -> Vec<u32> {
    let mut new_update = vec![];

    for item in complete_order {
        if update.iter().any(|&i| i == *item) {
            new_update.push(*item);
        }
    }
    new_update
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
    let complete_order = get_complete_order(&rules);

    let corrected = incorrect_updates
        .into_iter()
        .map(|x| correct_update(x, &complete_order))
        .collect::<Vec<Vec<u32>>>();
    println!("{}", sum_midpoints(corrected));

    Ok(())
}
