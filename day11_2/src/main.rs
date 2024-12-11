use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_list(path: &Path) -> Result<HashMap<u64, usize>, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut stones_str = String::new();

    reader.read_line(&mut stones_str)?;

    let stones_strs: Vec<&str> = stones_str.trim().split(" ").collect::<Vec<&str>>();

    let mut result = HashMap::new();
    for stone_str in stones_strs {
        let stone = stone_str.parse::<u64>().unwrap();
        match result.get(&stone) {
            Some(count) => {
                result.insert(stone, count + 1);
            }
            None => {
                result.insert(stone, 1);
            }
        }
    }

    return Ok(result);
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

fn change_stones(stones: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut result = HashMap::new();

    for (stone, count) in stones.into_iter() {
        let new_stones = change_stone(stone);
        for new_stone in new_stones {
            match result.get(&new_stone) {
                None => {
                    result.insert(new_stone, count);
                }
                Some(i) => {
                    result.insert(new_stone, i + count);
                }
            }
        }
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

    for _ in 0..75 {
        stones = change_stones(stones);
    }

    println!(
        "{}",
        stones
            .into_iter()
            .map(|(_stone, count)| count)
            .sum::<usize>()
    );

    Ok(())
}
