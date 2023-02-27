pub fn solve(input: &str) -> (u64, u64) {
    let chars = input.chars().collect::<Vec<_>>();
    let part1 = chars
        .array_windows::<4>()
        .enumerate()
        .fold(u64::MAX, |accum, (i, array)| {
            let mut valid = true;

            for (index, char) in array.iter().enumerate() {
                if array[index + 1..].contains(char) {
                    valid = false;
                }
            }

            if accum == u64::MAX && valid {
                i as u64 + 4
            } else {
                accum
            }
        });

    let part2 = chars
        .array_windows::<14>()
        .enumerate()
        .fold(u64::MAX, |accum, (i, array)| {
            let mut valid = true;

            for (index, char) in array.iter().enumerate() {
                if array[index + 1..].contains(char) {
                    valid = false;
                }
            }

            if accum == u64::MAX && valid {
                i as u64 + 14
            } else {
                accum
            }
        });

    (part1, part2)
}
