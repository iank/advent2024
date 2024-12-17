use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone)]
struct Computer {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
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

impl From<u32> for Opcode {
    fn from(item: u32) -> Self {
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

fn decode_combo_operand(computer: &Computer, operand: u32) -> u32 {
    match operand {
        0..=3 => operand,
        4 => computer.reg_a,
        5 => computer.reg_b,
        6 => computer.reg_c,
        _ => panic!("Invalid combo operand"),
    }
}

fn read_program(path: &Path) -> Result<(Computer, Vec<u32>), std::io::Error> {
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
    let program: Vec<u32> = programcaps
        .get(1)
        .unwrap()
        .as_str()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    Ok((computer, program))
}

// Execute an instruction, updating computer's state. Return Some(u32)
// if an output was produced, otherwise None
fn execute_instruction(c: &mut Computer, opcode: Opcode, operand: u32) -> Option<u32> {
    match opcode {
        Opcode::ADV => {
            c.reg_a = c.reg_a / 2_u32.pow(operand);
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
            c.reg_b = c.reg_a / 2_u32.pow(operand);
            c.ip += 2
        }
        Opcode::CDV => {
            c.reg_c = c.reg_a / 2_u32.pow(operand);
            c.ip += 2
        }
    }

    None
}

fn execute_program(computer: &mut Computer, program: Vec<u32>) -> Vec<u32> {
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

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let (computer, program) = read_program(&file_path)?;
    let mut computer = computer.clone();

    let results = execute_program(&mut computer, program);
    let result = results
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", result);

    Ok(())
}
