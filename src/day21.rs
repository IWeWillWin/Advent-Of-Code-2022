pub fn solve(input: &str) -> (u64, u64) {
    let monkeys = input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .collect::<Vec<_>>();

    let root = find_index("root", &monkeys).unwrap() as u32;
    let mut operations = monkeys
        .iter()
        .map(|(_, operation)| {
            if operation.contains('+') {
                let (lhs, rhs) = operation.split_once(" + ").unwrap();
                let lhs = find_index(lhs, &monkeys).unwrap() as u32;
                let rhs = find_index(rhs, &monkeys).unwrap() as u32;
                Operation::Add(lhs, rhs)
            } else if operation.contains('-') {
                let (lhs, rhs) = operation.split_once(" - ").unwrap();
                let lhs = find_index(lhs, &monkeys).unwrap() as u32;
                let rhs = find_index(rhs, &monkeys).unwrap() as u32;
                Operation::Sub(lhs, rhs)
            } else if operation.contains('*') {
                let (lhs, rhs) = operation.split_once(" * ").unwrap();
                let lhs = find_index(lhs, &monkeys).unwrap() as u32;
                let rhs = find_index(rhs, &monkeys).unwrap() as u32;
                Operation::Mul(lhs, rhs)
            } else if operation.contains('/') {
                let (lhs, rhs) = operation.split_once(" / ").unwrap();
                let lhs = find_index(lhs, &monkeys).unwrap() as u32;
                let rhs = find_index(rhs, &monkeys).unwrap() as u32;
                Operation::Div(lhs, rhs)
            } else {
                Operation::Number(operation.parse::<u32>().unwrap())
            }
        })
        .collect::<Vec<_>>();
    let part1 = collapse(root, &operations).unwrap();

    let human = find_index("humn", &monkeys).unwrap() as u32;
    let (lhs, rhs) = operations[root as usize].operands().unwrap();
    operations[root as usize] = Operation::Eq(lhs, rhs);
    operations[human as usize] = Operation::Human;
    let part2 = part2(root, &operations).unwrap();

    (part1, part2)
}

fn collapse(index: u32, operations: &[Operation]) -> Option<u64> {
    match operations[index as usize] {
        Operation::Add(lhs, rhs) => Some(collapse(lhs, operations)? + collapse(rhs, operations)?),
        Operation::Sub(lhs, rhs) => Some(collapse(lhs, operations)? - collapse(rhs, operations)?),
        Operation::Mul(lhs, rhs) => Some(collapse(lhs, operations)? * collapse(rhs, operations)?),
        Operation::Div(lhs, rhs) => Some(collapse(lhs, operations)? / collapse(rhs, operations)?),
        Operation::Eq(lhs, rhs) => {
            let lhs = collapse(lhs, operations)?;
            let rhs = collapse(rhs, operations)?;
            if lhs == rhs {
                Some(lhs)
            } else {
                None
            }
        }

        Operation::Number(num) => Some(num as u64),
        Operation::Human => None,
    }
}

fn fold(current: u64, index: u32, operations: &[Operation]) -> Option<u64> {
    if let Operation::Human = operations[index as usize] {
        return Some(current);
    }

    let (lhs, rhs) = operations[index as usize].operands().unwrap();
    let lhs_result = collapse(lhs, operations);
    let rhs_result = collapse(rhs, operations);

    match (lhs_result, rhs_result) {
        (Some(val), None) => {
            let new = match operations[index as usize] {
                Operation::Add(_, _) => {
                    // val + _ = current
                    // _ = current - val
                    current - val
                }
                Operation::Sub(_, _) => {
                    // val - _ = current
                    // _ = val - current
                    val - current
                }
                Operation::Mul(_, _) => {
                    // val * _ = current
                    // _ = current / val
                    current / val
                }
                Operation::Div(_, _) => {
                    // val / _ = current
                    // _ = val / current
                    val / current
                }
                Operation::Eq(_, _) | Operation::Number(_) | Operation::Human => panic!(),
            };
            fold(new, rhs, operations)
        }
        (None, Some(val)) => {
            let new = match operations[index as usize] {
                Operation::Add(_, _) => {
                    // _ + val = current
                    // _ = current - val
                    current - val
                }
                Operation::Sub(_, _) => {
                    // _ - val = current
                    // _ = current + val
                    current + val
                }
                Operation::Mul(_, _) => {
                    // _ * val = current
                    // _ = current / val
                    current / val
                }
                Operation::Div(_, _) => {
                    // _ / val = current
                    // _ = current * val
                    current * val
                }
                Operation::Eq(_, _) | Operation::Number(_) | Operation::Human => panic!(),
            };
            fold(new, lhs, operations)
        }
        (Some(_), Some(_)) | (None, None) => None,
    }
}

fn part2(index: u32, operations: &[Operation]) -> Option<u64> {
    if let Operation::Eq(lhs, rhs) = operations[index as usize] {
        let lhs_result = collapse(lhs, operations);
        let rhs_result = collapse(rhs, operations);

        match (lhs_result, rhs_result) {
            (None, Some(val)) => fold(val, lhs, operations),
            (Some(val), None) => fold(val, rhs, operations),
            (Some(_), Some(_)) | (None, None) => None,
        }
    } else {
        panic!();
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u32, u32),
    Sub(u32, u32),
    Mul(u32, u32),
    Div(u32, u32),
    Eq(u32, u32),
    Number(u32),
    Human,
}

impl Operation {
    fn operands(&self) -> Option<(u32, u32)> {
        match *self {
            Operation::Add(lhs, rhs) => Some((lhs, rhs)),
            Operation::Sub(lhs, rhs) => Some((lhs, rhs)),
            Operation::Mul(lhs, rhs) => Some((lhs, rhs)),
            Operation::Div(lhs, rhs) => Some((lhs, rhs)),
            Operation::Eq(lhs, rhs) => Some((lhs, rhs)),

            Operation::Number(_) | Operation::Human => None,
        }
    }
}

fn find_index(name: &str, slice: &[(&str, &str)]) -> Option<usize> {
    slice.iter().position(|(label, _)| *label == name)
}
