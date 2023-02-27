#![allow(clippy::identity_op)]
pub fn solve(input: &str) -> (u64, u64) {
    let part1 = input
        .lines()
        .map(|line| match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,

            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,

            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!(),
        })
        .sum();

    let part2 = input
        .lines()
        .map(|line| match line {
            "A X" => "A Z",
            "A Y" => "A X",
            "A Z" => "A Y",

            "B X" => "B X",
            "B Y" => "B Y",
            "B Z" => "B Z",

            "C X" => "C Y",
            "C Y" => "C Z",
            "C Z" => "C X",
            _ => panic!(),
        })
        .map(|line| match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,

            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,

            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!(),
        })
        .sum();

    (part1, part2)
}
