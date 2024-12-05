use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_input(path: &Path) -> Result<(Vec<(i32, i32)>, Vec<Vec<i32>>), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut rules: Vec<(i32, i32)> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];

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
                ordered_pages[0].parse::<i32>().unwrap(),
                ordered_pages[1].parse::<i32>().unwrap(),
            ))
        } else {
            let update = line.as_str().split(",").collect::<Vec<&str>>();
            updates.push(
                update
                    .iter()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        }
    }

    Ok((rules, updates))
}

fn check_rule(update: &Vec<i32>, rule: &(i32, i32)) -> bool {
    let i1 = update.iter().position(|n| *n == rule.0);
    let i2 = update.iter().position(|n| *n == rule.1);

    match (i1, i2) {
        (Some(a), Some(b)) => a < b,    // Check rule
        _ => true                       // One or more pages not in update, pass
    }
}

fn check_update(update: &Vec<i32>, rules: &Vec<(i32,i32)>) -> bool {
    for rule in rules {
        if check_rule(update, rule) == false {
            return false;
        }
    }

    true
}

fn filter_updates(updates: Vec<Vec<i32>>, rules: Vec<(i32,i32)>) -> Vec<Vec<i32>> {
    updates.into_iter().filter(|update| check_update(update, &rules)).collect::<Vec<Vec<i32>>>()
}

fn sum_midpoints(updates: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for update in updates {
        assert!(update.len() % 2 == 1);
        result += update[update.len() / 2];
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
    let (rules, updates) = read_input(file_path)?;

    let correct_updates = filter_updates(updates, rules);
    println!("{}", sum_midpoints(correct_updates));

    Ok(())
}
