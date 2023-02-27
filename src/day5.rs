pub fn solve(input: &str) -> (String, String) {
    let mut stacks: Vec<Vec<String>> = Vec::new();
    input
        .lines()
        .filter(|line| line.contains('['))
        .for_each(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, char)| ('A'..='Z').contains(char))
                .for_each(|(i, char)| {
                    let index = i / 4;

                    if stacks.len() <= index {
                        for _ in stacks.len()..=index {
                            stacks.push(Vec::new());
                        }
                    }
                    stacks[index].insert(0, char.to_string());
                })
        });

    let stacks_part2 = stacks.clone();

    input
        .lines()
        .filter(|line| line.contains("move"))
        .map(|line| {
            let mut array = [u8::MAX, u8::MAX, u8::MAX];
            line.split_ascii_whitespace()
                .filter(|string| string.contains(char::is_numeric))
                .take(3)
                .enumerate()
                .for_each(|(i, item)| {
                    array[i] = item.parse::<u8>().unwrap();
                });
            [array[0], array[1] - 1, array[2] - 1]
        })
        .for_each(|[count, from, to]| {
            for _ in 0..count {
                let item = stacks[from as usize].pop().unwrap();
                stacks[to as usize].push(item);
            }
        });

    let part1 = stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap().chars().next().unwrap())
        .collect::<String>();

    let mut stacks = stacks_part2;
    input
        .lines()
        .filter(|line| line.contains("move"))
        .map(|line| {
            let mut array = [u8::MAX, u8::MAX, u8::MAX];
            line.split_ascii_whitespace()
                .filter(|string| string.contains(char::is_numeric))
                .take(3)
                .enumerate()
                .for_each(|(i, item)| {
                    array[i] = item.parse::<u8>().unwrap();
                });
            [array[0], array[1] - 1, array[2] - 1]
        })
        .for_each(|[count, from, to]| {
            let stack = (0..count)
                .map(|_| stacks[from as usize].pop().unwrap())
                .collect::<Vec<String>>();
            stacks[to as usize].extend(stack.into_iter().rev());
        });

    let part2 = stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap().chars().next().unwrap())
        .collect::<String>();

    (part1, part2)
}
