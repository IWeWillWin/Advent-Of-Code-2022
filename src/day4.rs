pub fn solve(input: &str) -> (u64, u64) {
    let part1 = input
        .lines()
        .filter(|line| {
            let mut iter = line.split(',').map(|elf| {
                let mut iter = elf
                    .split('-')
                    .map(|num| num.parse::<u32>().unwrap())
                    .take(2);
                (iter.next().unwrap(), iter.next().unwrap())
            });

            let (low0, high0) = iter.next().unwrap();
            let (low1, high1) = iter.next().unwrap();

            (low0 <= low1 && high0 >= high1) || (low1 <= low0 && high1 >= high0)
        })
        .count() as u64;

    let part2 = input
        .lines()
        .filter(|line| {
            let mut iter = line.split(',').map(|elf| {
                let mut iter = elf
                    .split('-')
                    .map(|num| num.parse::<u32>().unwrap())
                    .take(2);
                (iter.next().unwrap(), iter.next().unwrap())
            });

            let (low0, high0) = iter.next().unwrap();
            let (low1, high1) = iter.next().unwrap();

            (low0..=high0).contains(&low1)
                || (low0..=high0).contains(&high1)
                || (low1..=high1).contains(&low0)
                || (low1..=high1).contains(&high0)
        })
        .count() as u64;

    (part1, part2)
}
