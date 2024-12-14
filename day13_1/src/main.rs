use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
use float_cmp::approx_eq;

#[derive(Debug)]
struct Machine {
    ax: f32,
    ay: f32,
    bx: f32,
    by: f32,
    prizex: f32,
    prizey: f32,
}

fn read_machines(path: &Path) -> Result<Vec<Machine>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let a_re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let b_re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut lines = reader.lines();
    let mut result = vec![];

    loop {
        let aline = lines.next().unwrap()?;
        let bline = lines.next().unwrap()?;
        let prizeline = lines.next().unwrap()?;

        let acaps = a_re.captures(aline.as_str()).unwrap();
        let bcaps = b_re.captures(bline.as_str()).unwrap();
        let prizecaps = prize_re.captures(prizeline.as_str()).unwrap();

        let machine = Machine {
            ax: acaps.get(1).unwrap().as_str().parse().unwrap(),
            ay: acaps.get(2).unwrap().as_str().parse().unwrap(),
            bx: bcaps.get(1).unwrap().as_str().parse().unwrap(),
            by: bcaps.get(2).unwrap().as_str().parse().unwrap(),
            prizex: prizecaps.get(1).unwrap().as_str().parse().unwrap(),
            prizey: prizecaps.get(2).unwrap().as_str().parse().unwrap(),
        };

        result.push(machine);

        if lines.next().is_none() { break; }
    }

    Ok(result)
}

// Find minimum # of tokens to win
fn min_tokens(m: Machine) -> Option<f32> {
    let mult = m.ax / m.bx;
    if approx_eq!(f32, m.by * mult, m.ay) {
        todo!("Colinear! {:#?}", m);
    }

    // prizex = a*ax + b*bx
    // prizey = a*ay + b*by
    let b = (m.prizex * m.ay / m.ax - m.prizey) / (m.bx * m.ay / m.ax - m.by);
    let a = m.prizex / m.ax - b * m.bx / m.ax;

    if a < 0.0 || b < 0.0 {
        return None
    }
    if ! approx_eq!(f32, a.round(), a, ulps = 100) {
        return None
    }
    if ! approx_eq!(f32, b.round(), b, ulps = 100) {
        return None
    }

    return Some(3.0*a + b);
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let machines = read_machines(&file_path)?;

    println!("{}", machines.into_iter().map(min_tokens).map(|c| c.unwrap_or(0.0)).sum::<f32>());

    Ok(())
}
