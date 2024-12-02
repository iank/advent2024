use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_reports(path: &Path) -> Result<Vec<Vec<i32>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut reports = vec![];

    for line in reader.lines() {
        let l = line?;
        let ll = (l.as_str()).split(" ").collect::<Vec<&str>>();

        let lll = ll
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        reports.push(lll);
    }

    return Ok(reports);
}

fn is_approximately_gradual(report: Vec<i32>) -> bool {
    if is_gradual(&report) {
        return true;
    }

    for index in 0..report.len() {
        // Remove index i and test
        let dampened_report = report
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if i != index { Some(x.clone()) } else { None })
            .collect();

        if is_gradual(&dampened_report) {
            return true;
        }
    }

    false
}

fn is_gradual(report: &Vec<i32>) -> bool {
    // ensure max delta is 3 and min delta is 1
    // ensure all deltas have the same sign
    let deltas: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();

    let all_gradual = deltas.iter().all(|&x| x.abs() >= 1 && x.abs() <= 3);
    let all_positive = deltas.iter().all(|&x| x >= 0);
    let all_negative = deltas.iter().all(|&x| x <= 0);

    /*    println!(
        "{:?}, {:?}, {}, {}, {}",
        report, deltas, all_gradual, all_positive, all_negative
    );*/

    return all_gradual && (all_positive || all_negative);
}

fn count_safe_reports(reports: Vec<Vec<i32>>) -> usize {
    let mut safe_reports = 0;
    for report in reports {
        if is_approximately_gradual(report) {
            safe_reports += 1;
        }
    }

    safe_reports
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let reports = read_reports(&file_path)?;

    let num_safe_reports = count_safe_reports(reports);

    println!("{}", num_safe_reports);

    Ok(())
}
