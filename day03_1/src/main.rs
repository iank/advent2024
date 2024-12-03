use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

fn read_instructions(path: &Path) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    Ok(lines)
}

fn do_multiplies(instructions: String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results = 0;

    for (_, [m1, m2]) in re.captures_iter(instructions.as_str()).map(|c| c.extract()) {
        results += m1.parse::<i32>().unwrap()*m2.parse::<i32>().unwrap();
    }

    results
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let instructions = read_instructions(&file_path)?;
    let result = do_multiplies(instructions.join(""));

    println!("{}", result);

    Ok(())
}
