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
struct Block(Option<usize>, usize);

fn expand_diskmap(diskmap: Vec<usize>) -> Vec<Block> {
    let mut file_id = 0;
    let mut disk = vec![];

    let mut iter = diskmap.iter();
    while let (Some(&filesz), freesz) = (iter.next(), iter.next()) {
        let freesz = freesz.unwrap_or(&0);
        disk.push(Block(Some(file_id), filesz));
        disk.push(Block(None, *freesz));

        file_id += 1;
    }

    disk
}

fn find_free_block_left_of(disk: &Vec<Block>, idx: usize, filesz: usize) -> Option<usize> {
    for idx in 0..idx {
        if disk[idx].0 == None && disk[idx].1 >= filesz {
            return Some(idx);
        }
    }

    None
}

fn compact_disk(disk: Vec<Block>) -> Vec<Block> {
    let mut compacted_disk = disk.clone();

    let mut idx = compacted_disk.len() - 1;

    while idx > 0 {
        //print_disk(&compacted_disk);
        if compacted_disk[idx].0 == None {
            idx -= 1;
            continue;
        }

        if let Block(Some(_), filesz) = compacted_disk[idx] {
            if let Some(free_idx) = find_free_block_left_of(&compacted_disk, idx, filesz) {
                let rem = compacted_disk[free_idx].1 - filesz;
                compacted_disk[free_idx] = compacted_disk[idx];
                compacted_disk[idx] = Block(None, filesz);
                compacted_disk.insert(free_idx + 1, Block(None, rem));
            }
        }

        idx -= 1;
    }

    //print_disk(&compacted_disk);
    compacted_disk
}

fn checksum(disk: Vec<Block>) -> u64 {
    let mut result = 0;
    let mut block_position = 0;

    for idx in 0..disk.len() {
        if let Block(Some(id), filesz) = disk[idx] {
            for _ in 0..filesz {
                result += block_position * id;
                block_position += 1;
            }
        }
        if let Block(None, freesz) = disk[idx] {
            block_position += freesz;
        }
    }

    result as u64
}

#[allow(dead_code)]
fn print_disk(disk: &Vec<Block>) {
    for block in disk {
        match block {
            Block(Some(a), l) => print!(
                "{}",
                String::from_utf8(vec![char::from_digit(*a as u32, 10).unwrap() as u8; *l])
                    .unwrap()
            ),
            Block(None, l) => print!("{}", String::from_utf8(vec![b'.'; *l]).unwrap()),
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
    let compacted_disk = compact_disk(disk);
    let checksum = checksum(compacted_disk);

    println!("{}", checksum);

    Ok(())
}
