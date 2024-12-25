use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone)]
enum LogicType {
    AND,
    OR,
    XOR,
}

type Wire = String;

#[derive(Debug, Clone)]
struct Gate {
    op: LogicType,
    a: Wire,
    b: Wire,
    c: Wire,
}

fn read_gates(path: &Path) -> Result<Vec<Gate>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let gate_re = Regex::new(r"([a-z0-9]{3}) (AND|OR|XOR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();
    let mut result = vec![];

    for line in reader.lines() {
        let line = line?;
        let caps = gate_re.captures(line.as_str()).unwrap();
        result.push(Gate {
            a: caps.get(1).unwrap().as_str().to_owned(),
            b: caps.get(3).unwrap().as_str().to_owned(),
            c: caps.get(4).unwrap().as_str().to_owned(),
            op: match caps.get(2).unwrap().as_str() {
                "OR" => LogicType::OR,
                "AND" => LogicType::AND,
                "XOR" => LogicType::XOR,
                _ => panic!(),
            }
        });
    }

    Ok(result)
}

fn evaluate_gate(g: &Gate, v: &HashMap::<&Wire, u8>) -> u8 {
    if !v.contains_key(&g.a) || !v.contains_key(&g.b) {
        panic!("Tried to evaluate Z inputs");
    }

    if v.contains_key(&g.c) {
        panic!("Tried to re-evaluate an output");
    }

    let a = v.get(&g.a).unwrap();
    let b = v.get(&g.b).unwrap();

    match g.op {
        LogicType::AND => { a & b },
        LogicType::OR => { a | b },
        LogicType::XOR => { a ^ b },
    }
}

//fn find_ready_gates(gates: &Vec<Gate>, v: &HashMap::<&Wire, u8>) -> Vec<&Gate> {
fn find_ready_gates<'a>(gates: &'a Vec<Gate>, v: &HashMap::<&Wire, u8>) -> Vec<&'a Gate> {
    gates.into_iter().filter(|ref g| v.contains_key(&g.a) && v.contains_key(&g.b) && !v.contains_key(&g.c)).collect()
}

fn evaluate_adder_truthtable(inputs: (Wire, Wire, Wire), output_wire: Wire, gates: &Vec<Gate>, half: bool) -> (bool, Vec<Wire>, Wire) {
    let adder_truth_table = if half {
        vec![
            // A B Cin Sum Carry
            (0, 0, 0, 0, 0),
            (0, 0, 1, 0, 0),
            (0, 1, 0, 1, 0),
            (0, 1, 1, 1, 0),
            (1, 0, 0, 1, 0),
            (1, 0, 1, 1, 0),
            (1, 1, 0, 0, 1),
            (1, 1, 1, 0, 1),
        ]
    } else {
        vec![
            // A B Cin Sum Carry
            (0, 0, 0, 0, 0),
            (0, 0, 1, 1, 0),
            (0, 1, 0, 1, 0),
            (0, 1, 1, 0, 1),
            (1, 0, 0, 1, 0),
            (1, 0, 1, 0, 1),
            (1, 1, 0, 0, 1),
            (1, 1, 1, 1, 1),
        ]
    };

    let mut wires_involved: HashSet::<&Wire> = HashSet::new();
    let mut unconsumed_outputs: HashSet::<&Wire> = HashSet::new();

    let mut correct = true;

    for (a, b, cin, sum, carry) in adder_truth_table {
        // Set up inputs
        let mut known_values = HashMap::from([
            (&inputs.0, a),
            (&inputs.1, b),
            (&inputs.2, cin),
        ]);

        // Evaluate system
        loop {
            let ready_gates = find_ready_gates(gates, &known_values);
            if ready_gates.len() == 0 {
                // Done evaluating
                break;
            }

            for ready_gate in ready_gates {
                // A and B can't be our carry output since they're being used now
                unconsumed_outputs.remove(&ready_gate.a);
                unconsumed_outputs.remove(&ready_gate.b);

                let result = evaluate_gate(&ready_gate, &known_values);

                // The output of this gate can be considered to be "involved"
                wires_involved.insert(&ready_gate.c);

                // The value of the output wire is now known
                known_values.insert(&ready_gate.c, result);

                // And the output wire is a candidate for our carry
                unconsumed_outputs.insert(&ready_gate.c);
            }
        }

        // Check truth table
        if unconsumed_outputs.len() != 2 {                     // There should be zxx and cary
            return (false, vec![], "".to_owned());
        }

        // assert_eq!(unconsumed_outputs.remove(&output_wire), true);  // Remove zxx
        if unconsumed_outputs.remove(&output_wire) != true {
            return (false, vec![], "".to_owned());
        }

        let carry_wire = unconsumed_outputs.iter().collect::<Vec<_>>()[0];

        if *known_values.get(&output_wire).unwrap() != sum {
            correct = false;
        }
        if *known_values.get(carry_wire).unwrap() != carry {
            correct = false;
        }
    }

    let carry_wire = unconsumed_outputs.into_iter().collect::<Vec<_>>()[0].clone();
    let wi: Vec<String> = wires_involved.iter().map(|s| (*s).clone()).collect();
    return (correct, wi, carry_wire);
}

fn generate_pairs<T: Clone>(items: &[T]) -> Vec<(T, T)> {
    let mut pairs = Vec::new();
    for (i, &ref item1) in items.iter().enumerate() {
        for item2 in &items[i + 1..] {
            pairs.push((item1.clone(), item2.clone()));
        }
    }
    pairs
}

fn swap_gates(gates: &Vec<Gate>, w1: &Wire, w2: &Wire) -> Vec<Gate> {
    let mut result = vec![];
    for gate in gates {
        let mut sg = (*gate).clone();

        if sg.c == *w1 { sg.c = w2.clone() }
        else if sg.c == *w2 { sg.c = w1.clone() }

        result.push(sg);
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

    let gates = read_gates(&file_path)?;

    let mut last_carry: String = "fake".to_string();
    let mut possibly_wrong_wires: HashSet::<Wire> = HashSet::new();
    for n in 0..=44 {
        let a = format!("x{:02}", n);
        let b = format!("y{:02}", n);
        let c = format!("z{:02}", n);
        let (correct, wires_involved, carry) = evaluate_adder_truthtable(
            (a, b, last_carry.clone()), c, &gates, n==0
        );

        if !correct {
            println!("{:02} correct?: {}", n, correct);
            //println!("{}: {}", n, last_carry);
            for wi in wires_involved {
                possibly_wrong_wires.insert(wi);
            }
        }

        last_carry = carry;
    }

    for pair in generate_pairs(&possibly_wrong_wires.into_iter().collect::<Vec<_>>()) {
        let sgates = swap_gates(&gates, &pair.0, &pair.1);

        let mut last_carry: String = "fake".to_string();
        for n in 0..=44 {
            let a = format!("x{:02}", n);
            let b = format!("y{:02}", n);
            let c = format!("z{:02}", n);
            let (correct, _wires_involved, carry) = evaluate_adder_truthtable(
                (a, b, last_carry.clone()), c, &sgates, n==0
            );

            if n == 10 && correct {
                println!("10: {:?}", pair);
            }
            else if n == 14 && correct {
                println!("14: {:?}", pair);
            }
            else if n == 25 && correct {
                println!("25: {:?}", pair);
            }
            else if n == 34 && correct {
                println!("34: {:?}", pair);
            }

            last_carry = carry;
        }
    }

    Ok(())
}
