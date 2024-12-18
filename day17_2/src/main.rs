use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone)]
struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    ip: usize,
}

#[derive(Debug, PartialEq)]
enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl From<u64> for Opcode {
    fn from(item: u64) -> Self {
        match item {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            _ => panic!("Invalid opcode"),
        }
    }
}

fn decode_combo_operand(computer: &Computer, operand: u64) -> u64 {
    match operand {
        0..=3 => operand,
        4 => computer.reg_a,
        5 => computer.reg_b,
        6 => computer.reg_c,
        _ => panic!("Invalid combo operand"),
    }
}

fn read_program(path: &Path) -> Result<(Computer, Vec<u64>), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let a_re = Regex::new(r"Register A: (\d+)").unwrap();
    let b_re = Regex::new(r"Register B: (\d+)").unwrap();
    let c_re = Regex::new(r"Register C: (\d+)").unwrap();
    let program_re = Regex::new(r"Program: ([\d,]+)").unwrap();

    let mut lines = reader.lines();

    let aline = lines.next().unwrap()?;
    let bline = lines.next().unwrap()?;
    let cline = lines.next().unwrap()?;
    let _ = lines.next().unwrap()?;
    let programline = lines.next().unwrap()?;

    let acaps = a_re.captures(aline.as_str()).unwrap();
    let bcaps = b_re.captures(bline.as_str()).unwrap();
    let ccaps = c_re.captures(cline.as_str()).unwrap();

    let computer = Computer {
        reg_a: acaps.get(1).unwrap().as_str().parse().unwrap(),
        reg_b: bcaps.get(1).unwrap().as_str().parse().unwrap(),
        reg_c: ccaps.get(1).unwrap().as_str().parse().unwrap(),
        ip: 0,
    };

    let programcaps = program_re.captures(programline.as_str()).unwrap();
    let program: Vec<u64> = programcaps
        .get(1)
        .unwrap()
        .as_str()
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    Ok((computer, program))
}

// Execute an instruction, updating computer's state. Return Some(u64)
// if an output was produced, otherwise None
fn execute_instruction(c: &mut Computer, opcode: Opcode, operand: u64) -> Option<u64> {
    match opcode {
        Opcode::ADV => {
            c.reg_a = c.reg_a / 2_u64.pow(operand as u32);
            c.ip += 2
        }
        Opcode::BXL => {
            c.reg_b = c.reg_b ^ operand;
            c.ip += 2
        }
        Opcode::BST => {
            c.reg_b = operand % 8;
            c.ip += 2
        }
        Opcode::JNZ => {
            if c.reg_a == 0 {
                c.ip += 2
            } else {
                c.ip = operand as usize
            }
        }
        Opcode::BXC => {
            c.reg_b = c.reg_b ^ c.reg_c;
            c.ip += 2
        }
        Opcode::OUT => {
            c.ip += 2;
            return Some(operand % 8);
        }
        Opcode::BDV => {
            c.reg_b = c.reg_a / 2_u64.pow(operand as u32);
            c.ip += 2
        }
        Opcode::CDV => {
            c.reg_c = c.reg_a / 2_u64.pow(operand as u32);
            c.ip += 2
        }
    }

    None
}

fn execute_program(computer: &mut Computer, program: &Vec<u64>) -> Vec<u64> {
    let mut result = vec![];
    let combo_instructions = vec![
        Opcode::ADV,
        Opcode::BST,
        Opcode::OUT,
        Opcode::BDV,
        Opcode::CDV,
    ];
    loop {
        // Halt when IP is past the end
        if computer.ip + 1 > program.len() {
            break;
        }

        //        println!("{:#?}", computer);

        // Instructions didn't specify what happens here so
        // they probably don't ever do it?
        if computer.ip + 2 > program.len() {
            panic!("No operand available");
        }

        // decode instruction and operand
        let opcode = Opcode::from(program[computer.ip]);
        let mut operand = program[computer.ip + 1];

        // decode combo operand if applicable
        if combo_instructions.contains(&opcode) {
            operand = decode_combo_operand(computer, operand);
        }

        //        println!("{:#?} {:#?}", opcode, operand);

        // execute instruction and update Computer state
        match execute_instruction(computer, opcode, operand) {
            None => (),
            Some(output) => result.push(output),
        }
    }

    result
}

// Check if the 10-bit value 'a' causes computer running program to print 'p'
fn program_satisfied(a: u64, p: u64, computer: &Computer, program: &Vec<u64>) -> bool {
    let mut c = computer.clone();
    c.reg_a = a;

    let result = execute_program(&mut c, program);
    result[0] == p
}

// Find all of the 10 bit values for a that produce 'p' in the first iteration of program
fn find_all_10bit_a_for_p(p: u64, computer: &Computer, program: &Vec<u64>) -> Vec<u64> {
    let mut result = vec![];

    for a in 0..=1023 {
        if program_satisfied(a, p, computer, program) {
            result.push(a);
        }
    }
    result
}

// Produce the list of 16 (program.len()) 10 bit values that could, if overlapped, produce the
// program
fn find_all_10bit_a(computer: &Computer, program: &Vec<u64>) -> Vec<Vec<u64>> {
    let mut result = vec![];
    for p in program {
        result.push(find_all_10bit_a_for_p(*p, computer, program));
    }
    result
}

fn accumulate_3bit(acc: &Vec<u8>, rem: Option<u8>) -> u64 {
    let mut result: u64 = 0;
    for i in 0..acc.len() {
        result |= (acc[i] as u64) << (i * 3);
    }

    if let Some(remainder) = rem {
        result |= (remainder as u64) << (acc.len() * 3);
    }

    result
}

// Overlap the values to produce the possible 'a's
fn overlap_values(avec: Vec<Vec<u64>>, acc: Vec<u8>, constraint: Option<u8>) {
    if avec.len() == 0 {
        println!("{}", accumulate_3bit(&acc, constraint));
        return;
    }

    for i in 0..avec[0].len() {
        if let Some(mask) = constraint {
            if (avec[0][i] & 127) != mask as u64 {
                continue;
            }
        }

        let v = (avec[0][i] % 8) as u8;
        let rem = (avec[0][i] >> 3) as u8;

        // now we accumulate v, and find everything in avec[1-end]
        let mut acc_new = acc.clone();
        acc_new.push(v);
        overlap_values(avec[1..].to_vec(), acc_new, Some(rem));
    }
}

fn solve_quine(computer: &Computer, program: &Vec<u64>) {
    let avec = find_all_10bit_a(computer, program);
    overlap_values(avec, vec![], None);
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let (computer, program) = read_program(&file_path)?;

    /*let mut c = computer.clone();
    c.reg_a = 105706277661082;
    let result = execute_program(&mut c, &program);
    println!("{:?}", result);*/

    solve_quine(&computer, &program);

    Ok(())
}
