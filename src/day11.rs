use std::collections::VecDeque;

pub fn solve(input: &str) -> (u64, u64) {
    let mut monkeys1 = input
        .split("\n\n")
        .map(|monkey| {
            let mut lines = monkey.lines().skip(1);

            let items = lines
                .next()
                .unwrap()
                .split_at(18)
                .1
                .split(", ")
                .map(|number| number.parse::<u64>().unwrap())
                .collect::<VecDeque<_>>();

            let operation = match lines.next().unwrap().split_at(23).1.split_at(2) {
                ("* ", "old") => Operation::Pow,
                ("+ ", x) => Operation::Add(x.parse::<u64>().unwrap()),
                ("* ", x) => Operation::Mul(x.parse::<u64>().unwrap()),
                _ => panic!(),
            };

            let branch = lines.next().unwrap().split_at(21).1.parse::<u64>().unwrap();
            let if_true = lines.next().unwrap().split_at(29).1.parse::<u32>().unwrap();
            let if_false = lines.next().unwrap().split_at(30).1.parse::<u32>().unwrap();

            (items, operation, branch, if_true, if_false)
        })
        .collect::<Vec<_>>();
    let mut monkeys2 = monkeys1.clone();

    let mut inspections = vec![0; monkeys1.len()];
    for _ in 0..20 {
        for i in 0..monkeys1.len() {
            inspections[i] += monkeys1[i].0.len();
            for _ in 0..monkeys1[i].0.len() {
                let worry = monkeys1[i].0.pop_front().unwrap();
                let worry = match monkeys1[i].1 {
                    Operation::Mul(x) => worry * x,
                    Operation::Add(x) => worry + x,
                    Operation::Pow => worry * worry,
                };
                let worry = worry / 3;

                if worry % monkeys1[i].2 == 0 {
                    let index = monkeys1[i].3 as usize;
                    monkeys1[index].0.push_back(worry);
                } else {
                    let index = monkeys1[i].4 as usize;
                    monkeys1[index].0.push_back(worry);
                }
            }
        }
    }
    inspections.sort();
    let part1: u64 = inspections
        .iter()
        .rev()
        .take(2)
        .map(|value| *value as u64)
        .product();

    let modulo = monkeys2
        .iter()
        .fold(1, |accum, (_, _, test, _, _)| accum * test);

    let mut inspections = vec![0; monkeys2.len()];
    for _ in 0..10_000 {
        for i in 0..monkeys2.len() {
            inspections[i] += monkeys2[i].0.len();
            for _ in 0..monkeys2[i].0.len() {
                let worry = monkeys2[i].0.pop_front().unwrap();
                let worry = match monkeys2[i].1 {
                    Operation::Mul(x) => worry * x,
                    Operation::Add(x) => worry + x,
                    Operation::Pow => worry * worry,
                };
                let worry = worry % modulo;

                if worry % monkeys2[i].2 == 0 {
                    let index = monkeys2[i].3 as usize;
                    monkeys2[index].0.push_back(worry);
                } else {
                    let index = monkeys2[i].4 as usize;
                    monkeys2[index].0.push_back(worry);
                }
            }
        }
    }
    inspections.sort();
    let part2: u64 = inspections
        .iter()
        .rev()
        .take(2)
        .map(|value| *value as u64)
        .product();

    (part1, part2)
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Mul(u64),
    Add(u64),
    Pow,
}
