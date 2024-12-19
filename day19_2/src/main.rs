use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

type Towel = String;
type Design = String;

fn read_input(path: &Path) -> Result<(HashSet<Towel>, Vec<Design>), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut towels: HashSet<Towel> = HashSet::new();
    let mut designs: Vec<Design> = vec![];

    let mut lines = reader.lines();

    let line = lines.next().unwrap()?;
    let towel_strs = line.as_str().split(", ").collect::<Vec<&str>>();
    for towel_str in towel_strs {
        towels.insert(towel_str.to_owned());
    }

    let line = lines.next().unwrap()?;
    assert!(line == "");

    for line in lines {
        let line = line?;
        designs.push(line.to_owned());
    }

    Ok((towels, designs))
}

fn n_possible_arrangements(
    design: &Design,
    towels: &HashSet<Towel>,
    start: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if memo.contains_key(&start) {
        return *memo.get(&start).unwrap();
    }

    if start == design.len() {
        return 1;
    }

    let mut result = 0;
    for end in (start + 1)..=design.len() {
        if towels.contains(&design[start..end]) {
            result += n_possible_arrangements(design, towels, end, memo);
        }
    }

    memo.insert(start, result);
    result
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let (towels, designs) = read_input(file_path)?;

    let mut npossible = 0;
    for design in designs {
        let mut memo = HashMap::new();
        npossible += n_possible_arrangements(&design, &towels, 0, &mut memo);
    }
    println!("{}", npossible);

    Ok(())
}
