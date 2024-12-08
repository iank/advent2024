use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, Clone)]
struct Equation {
    testval: u64,
    operands: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn generate_operators(n: usize) -> impl Iterator<Item = Vec<Operator>> {
    let num_combinations = 3_usize.pow(n as u32);

    (0..num_combinations).map(move |i| {
        let mut value = i;
        let mut combination = Vec::with_capacity(n);

        for _ in 0..n {
            let digit = value % 3;
            value /= 3;
            combination.push(match digit {
                0 => Operator::Multiply,
                1 => Operator::Add,
                2 => Operator::Concatenate,
                _ => unreachable!(),
            });
        }

        combination
    })
}

fn read_equations(path: &Path) -> Result<Vec<Equation>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut equations = vec![];

    for line in reader.lines().map(|line| line.unwrap()) {
        let mut parts = line.split_whitespace();
        let equation = Equation {
            testval: parts.next().unwrap().trim_end_matches(':').parse().unwrap(),
            operands: parts.map(|s| s.parse().unwrap()).collect(),
        };

        equations.push(equation);
    }

    Ok(equations)
}

fn op_concat(a: u64, b: u64) -> u64 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}

fn evaluate_equation(mut equation: Equation, operators: Vec<Operator>) -> u64 {
    assert!(operators.len() == equation.operands.len() - 1);

    let mut result = equation.operands.remove(0);
    for operator in operators {
        let a = equation.operands.remove(0);
        result = match operator {
            Operator::Add => result + a,
            Operator::Multiply => result * a,
            Operator::Concatenate => op_concat(result, a),
        }
    }

    result
}

fn is_equation_solvable(equation: &Equation) -> bool {
    let operators_list = generate_operators(equation.operands.len() - 1);

    for operators in operators_list {
        if evaluate_equation(equation.clone(), operators) == equation.testval {
            return true
        }
    }

    false
}

fn get_possible_equations(equations: Vec<Equation>) -> Vec<Equation> {
    equations.iter().filter(|x| is_equation_solvable(x)).cloned().collect::<Vec<Equation>>()
}

fn sum_calibration_values(equations: Vec<Equation>) -> u64 {
    equations.iter().map(|x| x.testval).sum()
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    let equations = read_equations(&file_path)?;
    let possible_equations = get_possible_equations(equations);
    let result = sum_calibration_values(possible_equations);

    println!("{}", result);

    Ok(())
}
