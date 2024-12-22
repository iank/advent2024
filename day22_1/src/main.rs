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

fn iterate_secret(s: u64, n: usize) -> u64 {
    let mut s = s;
    for _ in 0..n {
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

    s
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let secrets = read_input(file_path)?;

    let iterated_secrets: Vec<u64> = secrets
        .into_iter()
        .map(|s| iterate_secret(s, 2000))
        .collect();

    println!("{}", iterated_secrets.into_iter().sum::<u64>());

    Ok(())
}
