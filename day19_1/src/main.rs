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

fn is_design_possible(design: &Design, towels: &HashSet<Towel>) -> bool {
    let mut dp = vec![false; design.len() + 1];
    dp[0] = true; // Trivial case

    // Which parts can we break into substrings?
    for i in 1..=design.len() {
        // See if there's a substring-able prefix + a suffix in the wordlist. If so,
        // then we have a new substring-able chunk.
        for j in 0..i {
            if dp[j] && towels.contains(&design[j..i]) {
                dp[i] = true;
            }
        }
    }

    dp[design.len()]
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let (towels, designs) = read_input(file_path)?;

    //    println!("{:#?} {:#?}", towels, designs);
    let npossible = designs
        .iter()
        .map(|d| is_design_possible(d, &towels))
        .filter(|b| *b)
        .count();
    println!("{}", npossible);

    Ok(())
}
