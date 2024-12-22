use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_input(path: &Path) -> Result<Vec<u64>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result = vec![];

    for line in reader.lines() {
        let line = line?;
        result.push(line.as_str().parse::<u64>().unwrap());
    }

    Ok(result)
}

fn get_prices(s: u64, n: usize) -> Vec<i32> {
    let mut result = vec![];
    let mut s = s;
    for _ in 0..n {
        result.push((s % 10) as i32);

        // Calculate the result of multiplying the secret number by 64. Then, mix this result into
        // the secret number. Finally, prune the secret number.
        s ^= s * 64;
        s = s % 16777216;

        // Calculate the result of dividing the secret number by 32. Round the result down to the
        // nearest integer. Then, mix this result into the secret number. Finally, prune the secret
        // number.
        s ^= s / 32;
        s = s % 16777216;

        // Calculate the result of multiplying the secret number by 2048. Then, mix this result
        // into the secret number. Finally, prune the secret number.
        s ^= s * 2048;
        s = s % 16777216;
    }

    result
}

fn construct_sequence_map(prices: &Vec<i32>) -> HashMap<Vec<i32>, i32> {
    let mut sm = HashMap::new();
    for i in 4..prices.len() {
        let seq = vec![
            prices[i-3] - prices[i-4],
            prices[i-2] - prices[i-3],
            prices[i-1] - prices[i-2],
            prices[i] - prices[i-1],
        ];
        if sm.contains_key(&seq) {
            continue;
        }
        else {
            sm.insert(seq, prices[i]);
        }
    }
    sm
}

fn construct_sequence_map_all(prices: &Vec<Vec<i32>>) -> HashMap<Vec<i32>, i32> {
    let mut sm_all = HashMap::new();

    for i in 0..prices.len() {
        let sm = construct_sequence_map(&prices[i]);
        for (sequence, price) in sm.iter() {
            *sm_all.entry(sequence.clone()).or_insert(0) += price;
        }
    }

    sm_all
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let secrets = read_input(file_path)?;

    let prices: Vec<Vec<i32>> = secrets
        .iter()
        .map(|s| get_prices(*s, 2000))
        .collect();

    // Find sequence
    let sequencemap = construct_sequence_map_all(&prices);
    println!("{}", sequencemap.values().max().unwrap());

    Ok(())
}
