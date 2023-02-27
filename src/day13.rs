use std::cmp::Ordering;
use std::str::Chars;

pub fn solve(input: &str) -> (u64, u64) {
    let pairs = input
        .split("\n\n")
        .map(|packet| {
            let (left, right) = packet.split_once('\n').unwrap();
            (parse(left), parse(right))
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;
    for (i, (left, right)) in pairs.iter().enumerate() {
        if compare(left, right) == ControlFlow::Correct {
            part1 += i as u64 + 1;
        }
    }

    let mut packets = pairs
        .into_iter()
        .flat_map(|(left, right)| [left, right])
        .collect::<Vec<_>>();
    packets.push(Kind::List(vec![Kind::List(vec![Kind::Integer(2)])]));
    packets.push(Kind::List(vec![Kind::List(vec![Kind::Integer(6)])]));

    for i in 0..packets.len() {
        for j in i..packets.len() {
            if compare(&packets[i], &packets[j]) == ControlFlow::Incorrect {
                packets.swap(i, j);
            }
        }
    }

    let first = packets
        .iter()
        .position(|packet| packet == &Kind::List(vec![Kind::List(vec![Kind::Integer(2)])]))
        .unwrap()
        + 1;
    let second = packets
        .iter()
        .position(|packet| packet == &Kind::List(vec![Kind::List(vec![Kind::Integer(6)])]))
        .unwrap()
        + 1;
    let part2 = first as u64 * second as u64;

    (part1, part2)
}

fn compare(left: &Kind, right: &Kind) -> ControlFlow {
    match (left, right) {
        (Kind::Integer(left), Kind::Integer(right)) => match left.cmp(right) {
            Ordering::Less => ControlFlow::Correct,
            Ordering::Equal => ControlFlow::Continue,
            Ordering::Greater => ControlFlow::Incorrect,
        },
        (Kind::List(left), Kind::List(right)) => {
            let mut left_iter = left.iter();
            let mut right_iter = right.iter();

            loop {
                match (left_iter.next(), right_iter.next()) {
                    (None, None) => return ControlFlow::Continue,
                    (None, Some(_)) => return ControlFlow::Correct,
                    (Some(_), None) => return ControlFlow::Incorrect,
                    (Some(left), Some(right)) => {
                        if let result @ (ControlFlow::Correct | ControlFlow::Incorrect) =
                            compare(left, right)
                        {
                            return result;
                        }
                    }
                }
            }
        }
        (Kind::List(_), Kind::Integer(right)) => {
            compare(left, &Kind::List(vec![Kind::Integer(*right)]))
        }
        (Kind::Integer(left), Kind::List(_)) => {
            compare(&Kind::List(vec![Kind::Integer(*left)]), right)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlFlow {
    Correct,
    Continue,
    Incorrect,
}

fn parse(input: &str) -> Kind {
    parse_value(&mut input.chars())
}

fn parse_value(chars: &mut Chars) -> Kind {
    match chars.clone().next().unwrap() {
        '0'..='9' => {
            let mut number = 0;
            while let Some(c @ '0'..='9') = chars.clone().next() {
                number = number * 10 + (c as u8 - b'0');
                chars.next();
            }

            Kind::Integer(number)
        }
        '[' => {
            chars.next();

            let mut list = vec![];
            loop {
                if let Some(']') = chars.clone().next() {
                    break;
                }

                let x = parse_value(chars);
                list.push(x);

                if let Some(',') = chars.clone().next() {
                    chars.next();
                    continue;
                } else {
                    break;
                }
            }

            assert_eq!(Some(']'), chars.next());

            Kind::List(list)
        }
        c => panic!("{c}"),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Kind {
    List(Vec<Kind>),
    Integer(u8),
}
