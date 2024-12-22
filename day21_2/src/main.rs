// Cribbed heavily from Github user 'hextree'
use std::collections::HashMap;

fn get_paths(a: &(isize, isize), b: &(isize, isize)) -> Vec<Vec<char>> {
    let dx = b.1 - a.1;
    let dy = b.0 - a.0;

    let mut result = vec![vec!['A']; 2];
    let mut xpress = vec![];
    let mut ypress = vec![];

    if dy > 0 {
        ypress.extend(vec!['v'; dy.abs() as usize]);
    } else if dy < 0 {
        ypress.extend(vec!['^'; dy.abs() as usize]);
    }

    if dx > 0 {
        xpress.extend(vec!['>'; dx.abs() as usize]);
    } else if dx < 0 {
        xpress.extend(vec!['<'; dx.abs() as usize]);
    }

    // X first
    result[0].extend(xpress.clone());
    result[0].extend(ypress.clone());

    // Y first
    result[1].extend(ypress);
    result[1].extend(xpress);

    return result;
}

const N_ROBOTS: usize = 26;

fn update_position(c: &(isize, isize), key: char) -> (isize, isize) {
    match key {
        '>' => (c.0, c.1 + 1),
        '<' => (c.0, c.1 - 1),
        '^' => (c.0 - 1, c.1),
        'v' => (c.0 + 1, c.1),
        _ => (c.0, c.1),
    }
}

fn shortest_seq(robot_id: usize, start_key: char, dest_key: char, memo: &mut HashMap<(usize, char, char), usize>) -> usize {
    let cache_key = (robot_id, start_key, dest_key);
    if memo.contains_key(&cache_key) {
        return *memo.get(&cache_key).unwrap();
    }

    let numeric_pad: HashMap<char, (isize, isize)> = HashMap::from([
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

    let arrow_pad: HashMap<char, (isize, isize)> = HashMap::from([
        ('A', (0, 2)),
        ('^', (0, 1)),
        ('>', (1, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        (' ', (0, 0)),
    ]);

    let pad = if robot_id == 0 { numeric_pad } else { arrow_pad };

    let avoid = pad.get(&' ').unwrap();

    let start_pos = pad.get(&start_key).unwrap();
    let dest_pos = pad.get(&dest_key).unwrap();

    // base case: if robot_id = total_robots - 1 return manhattan distance + 1 (for the A press)
    if robot_id == N_ROBOTS - 1 {
        return ((dest_pos.0 - start_pos.0).abs() + (dest_pos.1 - start_pos.1).abs()) as usize + 1;
    }

    let mut candidates = vec![];
    let paths = get_paths(start_pos, dest_pos);

    for path in paths {
        let mut cost = 0;
        let mut current_position = *start_pos;
        let mut invalid = false;

        for i in 1..path.len() {
            cost += shortest_seq(robot_id + 1, path[i-1], path[i], memo);
            current_position = update_position(&current_position, path[i]);
            if current_position == *avoid {
                invalid = true;
                break;
            }
        }

        if !invalid {
            cost += shortest_seq(robot_id + 1, path[path.len()-1], 'A', memo);
            candidates.push(cost);
        }
    }

    let result = candidates.into_iter().min().unwrap();
    memo.insert(cache_key, result);
//    println!("robot {} from {} to {}: {}", robot_id, start_key, dest_key, result);
    return result;
}

fn get_answer(
    code: Vec<char>,
) -> usize {
    let mut memo = HashMap::new();
    let presses = shortest_seq(0, 'A', code[0], &mut memo) +
                  shortest_seq(0, code[0], code[1], &mut memo) +
                  shortest_seq(0, code[1], code[2], &mut memo) +
                  shortest_seq(0, code[2], 'A', &mut memo);

    let num = (
        code[0].to_digit(10).unwrap() * 100
        + code[1].to_digit(10).unwrap() * 10
        + code[2].to_digit(10).unwrap()) as usize;

    println!("{:?}:{}*{}", code.iter().collect::<String>(), presses, num);
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

    let mut answer = 0;
    for code in codes {
        answer += get_answer(code.clone());
    }
    println!("{}", answer);
}
