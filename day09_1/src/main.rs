use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_diskmap(path: &Path) -> Result<Vec<usize>, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut diskmap_str = String::new();
    reader.read_line(&mut diskmap_str)?;

    let diskmap: Vec<usize> = diskmap_str
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    Ok(diskmap)
}

#[derive(Clone, Debug, PartialEq, Copy)]
struct Block(Option<usize>);

fn expand_diskmap(diskmap: Vec<usize>) -> Vec<Block> {
    let mut file_id = 0;
    let mut disk = vec![];

    let mut iter = diskmap.iter();
    while let (Some(&filesz), freesz) = (iter.next(), iter.next()) {
        let freesz = freesz.unwrap_or(&0);
        disk.extend(vec![Block(Some(file_id)); filesz]);
        disk.extend(vec![Block(None); *freesz]);

        file_id += 1;
    }

    disk
}

fn compact_disk(disk: Vec<Block>) -> Vec<Block> {
    let mut idx = 0;
    let mut end_idx = disk.len() - 1;

    let mut compacted_disk = disk.clone();

    loop {
        //print_disk(&compacted_disk);
        if compacted_disk[idx] == Block(None) {
            while compacted_disk[end_idx] == Block(None) {
                end_idx -= 1;
            }
            if idx >= end_idx {
                break;
            }

            compacted_disk[idx] = compacted_disk[end_idx];
            compacted_disk[end_idx] = Block(None);

            idx += 1;
        }
        else {
            idx += 1;
        }
    }

    compacted_disk
}

fn checksum(disk: Vec<Block>) -> u64 {
    let mut result = 0;
    for idx in 0..disk.len() {
        if let Block(Some(id)) = disk[idx] {
            result += idx*id;
        }
    }

    result as u64
}

#[allow(dead_code)]
fn print_disk(disk: &Vec<Block>) {
    for block in disk {
        match block {
            Block(Some(a)) => print!("{}", a),
            Block(None) => print!("."),
        }
    }
    println!();
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let diskmap = read_diskmap(file_path)?;
    let disk = expand_diskmap(diskmap);
//    print_disk(&disk);
    let compacted_disk = compact_disk(disk);
//    print_disk(&compacted_disk);
    let checksum = checksum(compacted_disk);

    println!("{}", checksum);

    Ok(())
}
