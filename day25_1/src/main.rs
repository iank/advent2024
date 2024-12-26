use itertools::iproduct;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(PartialEq, Debug)]
enum LockObjType {
    Lock,
    Key,
}

#[derive(Debug)]
struct LockObj {
    t: LockObjType,
    v: [usize; 5],
}

fn parse_lockobj(lines: &[String]) -> LockObj {
    assert!(lines.len() == 7, "Lock objects are 7 lines long");
    let id = "#####";

    let t = if lines[0] == id {
        LockObjType::Lock
    } else if lines[lines.len() - 1] == id {
        LockObjType::Key
    } else {
        panic!("Lock object is neither lock nor key");
    };

    let mut values = [0; 5];
    for col in 0..5 {
        for row in 1..=5 {
            values[col] += if lines[row].chars().nth(col).unwrap() == '#' {
                1
            } else {
                0
            }
        }
    }

    LockObj {
        t: t,
        v: values.into(),
    }
}

fn read_input(path: &Path) -> Result<Vec<LockObj>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lockobjs = vec![];
    let mut current_block = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            lockobjs.push(parse_lockobj(&current_block));
            current_block.clear();
        } else {
            current_block.push(line);
        }
    }

    if !current_block.is_empty() {
        lockobjs.push(parse_lockobj(&current_block));
    }

    return Ok(lockobjs);
}

fn key_fits_lock(lock: &LockObj, key: &LockObj) -> bool {
    assert!(lock.t == LockObjType::Lock);
    assert!(key.t == LockObjType::Key);

    (0..5).all(|i| lock.v[i] + key.v[i] <= 5)
}

fn unique_fitting_pairs(objs: Vec<LockObj>) -> usize {
    // filter all unique key,lock combinations
    //  by if all column sums are <= 5

    let locks: Vec<_> = objs.iter().filter(|o| o.t == LockObjType::Lock).collect();
    let keys: Vec<_> = objs.iter().filter(|o| o.t == LockObjType::Key).collect();

    iproduct!(locks, keys)
        .filter(|(l, k)| key_fits_lock(l, k))
        .collect::<Vec<_>>()
        .len()
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let objs = read_input(file_path)?;

    println!("{}", unique_fitting_pairs(objs));

    Ok(())
}
