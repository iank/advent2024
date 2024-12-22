use std::collections::HashMap;
use std::collections::HashSet;
use queues::{queue, IsQueue, Queue};

const N_DIR_ROBOTS: usize = 2;

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    num_robot_pos: (isize, isize),
    arrow_robot_pos: Vec<(isize, isize)>,
}

fn invalid_arrow_pos(pos: (isize, isize)) -> bool {
    pos == (0,0) || pos.0 < 0 || pos.1 < 0 || pos.0 > 1 || pos.1 > 2
}
fn invalid_numeric_pos(pos: (isize, isize)) -> bool {
    pos == (3,0) || pos.0 < 0 || pos.1 < 0 || pos.0 > 3 || pos.1 > 2
}

fn update_state(current: &State, button: char, arrow_pad: &HashMap<(isize, isize), char>) -> Option<(State, bool)> {
    let mut output = false;
    let mut new_state = current.clone();

    let moves: HashMap<char, (isize, isize)> = HashMap::from([
        ('<', (0, -1)),
        ('>', (0,  1)),
        ('^', (-1, 0)),
        ('v', (1, 0)),
    ]);

    let mut current_button = &button;
    let mut i = 0;
    loop {
        // Apply button to arrow pad, moving or activating the robot
        if *current_button != 'A' {
            let dpos = moves.get(&current_button).unwrap();
            new_state.arrow_robot_pos[i].0 += dpos.0;
            new_state.arrow_robot_pos[i].1 += dpos.1;

            if invalid_arrow_pos(new_state.arrow_robot_pos[i]) {
                return None;
            }
            return Some((new_state, output));
        }

        // The button was 'A', so we push a button on the next robot
        current_button = arrow_pad.get(&new_state.arrow_robot_pos[i]).unwrap();
        i += 1;

        if i == N_DIR_ROBOTS {
            break;
        }
    }

    // If we're here evreryone else pressed 'A'; this button now applies to the robot pointing to
    // the number pad
    if *current_button != 'A' {
        let dpos = moves.get(&current_button).unwrap();
        new_state.num_robot_pos.0 += dpos.0;
        new_state.num_robot_pos.1 += dpos.1;

        if invalid_numeric_pos(new_state.num_robot_pos) {
            return None;
        }
        return Some((new_state, output));
    }

    // Everyone (including numpad robot) pressed 'A'; output is generated
    output = true;
    return Some((new_state, output));
}

// find the shortest number of presses from start to goal pressed
fn find_shortest_sequence(start: (isize, isize), goal: (isize, isize), arrow_pad: &HashMap<(isize, isize), char>) -> usize {
    let mut q: Queue<(State, usize)> = queue![];
    let mut visited: HashSet<State> = HashSet::new();

    q.add((State {
        num_robot_pos: start,
        arrow_robot_pos: vec![(0, 2); N_DIR_ROBOTS],
    }, 0)).unwrap();

    while q.size() > 0 {
        let (current, depth) = q.remove().unwrap();

        if visited.contains(&current) {
            continue;
        }

        visited.insert(current.clone());

        for button in ['<', '>', '^', 'v', 'A'] {
            // compute what happens, filtering out invalid moves
            if let Some((new_state, output)) = update_state(&current, button, arrow_pad) {
                if output == true {
                    if new_state.num_robot_pos == goal {
                        return depth + 1;
                    }
                    else {
                        // Unnecessary A press, this isn't on the shortest path
                        continue;
                    }
                }

                q.add((new_state, depth + 1)).unwrap();
            }
        }
    }

    panic!("BFS finished without finding solution");
}

fn get_answer(
    code: Vec<char>,
    numeric_pad: &HashMap<char, (isize, isize)>,
    arrow_pad: &HashMap<(isize, isize), char>,
) -> usize {

    let mut number_pos = numeric_pad.get(&'A').unwrap();
    let mut presses = 0;

    for key in code.iter() {
        presses += find_shortest_sequence(*number_pos, *(numeric_pad.get(key).unwrap()), &arrow_pad);
        number_pos = numeric_pad.get(key).unwrap();
    }

    let num = (
        code[0].to_digit(10).unwrap() * 100
        + code[1].to_digit(10).unwrap() * 10
        + code[2].to_digit(10).unwrap()) as usize;

    println!("{}: {} * {}", code.iter().collect::<String>(), presses, num);
    return presses * num;
}

fn main() {
    let codes: Vec<Vec<char>> = vec![
        vec!['7', '8', '0', 'A'],
        vec!['5', '3', '9', 'A'],
        vec!['3', '4', '1', 'A'],
        vec!['1', '8', '9', 'A'],
        vec!['6', '8', '2', 'A'],
    ];
    let _example: Vec<Vec<char>> = vec![
        vec!['0', '2', '9', 'A'],
        vec!['9', '8', '0', 'A'],
        vec!['1', '7', '9', 'A'],
        vec!['4', '5', '6', 'A'],
        vec!['3', '7', '9', 'A'],
    ];

    let numeric_pad = HashMap::from([
        ('A', (3, 2)),
        ('0', (3, 1)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        (' ', (3, 0)),
    ]);

    let arrow_pad = HashMap::from([
        ((0, 2), 'A'),
        ((0, 1), '^'),
        ((1, 2), '>'),
        ((1, 0), '<'),
        ((1, 1), 'v'),
        ((0, 0), ' '),
    ]);

    let mut answer = 0;
    for code in codes {
        answer += get_answer(code.clone(), &numeric_pad, &arrow_pad);
    }
    println!("{}", answer);
}
