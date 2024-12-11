use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_list(path: &Path) -> Result<Vec<u64>, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut stones_str = String::new();

    reader.read_line(&mut stones_str)?;

    let stones_strs: Vec<&str> = stones_str.trim().split(" ").collect::<Vec<&str>>();
    let stones = stones_strs
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    return Ok(stones);
}

fn split_stone(stone: u64) -> Vec<u64> {
    let s: String = stone.to_string();
    let (s1, s2) = s.split_at(s.len() / 2);

    vec![s1.parse().unwrap(), s2.parse().unwrap()]
}

fn even_digits(stone: u64) -> bool {
    let s: String = stone.to_string();
    s.len() % 2 == 0
}

fn change_stone(stone: u64) -> Vec<u64> {
    if stone == 0 {
        // Rule 1: replace 0 with 1
        return vec![1];
    } else if even_digits(stone) {
        // Rule 2: even digits split into two stones
        return split_stone(stone);
    } else {
        // Rule 3: Multiply by 2024
        return vec![stone * 2024];
    }
}

fn change_stones(stones: Vec<u64>) -> Vec<u64> {
    let mut result = vec![];
    for stone in stones {
        result.extend(change_stone(stone));
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

    let mut stones = read_list(&file_path)?;

    for _ in 0..25 {
        stones = change_stones(stones);
    }

    println!("{}", stones.len());

    Ok(())
}
