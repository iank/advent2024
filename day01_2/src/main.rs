use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn read_lists(path: &Path) -> Result<(Vec<i32>, Vec<i32>), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in reader.lines() {
        let l = line?;
        let ll = (l.as_str()).split("   ").collect::<Vec<&str>>();

        list1.push(ll[0].parse::<i32>().unwrap());
        list2.push(ll[1].parse::<i32>().unwrap());
    }

    return Ok((list1, list2));
}

fn occurences(list: &Vec<i32>, x: &i32) -> i32 {
    list.iter().filter(|&n| *n == *x).count() as i32
}

fn list_similarity(list1: &mut Vec<i32>, list2: &mut Vec<i32>) -> i32 {
    let mut similarity = 0;

    for x in list1.iter() {
        similarity += x * occurences(list2, x);
    }

    similarity
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let (mut list1, mut list2) = read_lists(&file_path)?;
    let similarity = list_similarity(&mut list1, &mut list2);

    println!("{}", similarity);

    Ok(())
}
