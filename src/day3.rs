pub fn solve(input: &str) -> (u64, u64) {
    let part1: u64 = input
        .lines()
        .map(|line| {
            let p1 = line.split_at(line.len() / 2).0.as_bytes();
            let p2 = line.split_at(line.len() / 2).1.as_bytes();

            let mut done = [false; 52];
            for char in p1 {
                if p2.contains(char) {
                    if char.is_ascii_lowercase() {
                        let index = (char - b'a') as usize;
                        done[index] = true;
                    } else {
                        let index = (char - b'A' + 26) as usize;
                        done[index] = true;
                    }
                }
            }

            let score: u64 = done
                .into_iter()
                .enumerate()
                .filter_map(|(index, both)| if both { Some(index as u64 + 1) } else { None })
                .sum();

            score
        })
        .sum();

    let part2: u64 = input
        .lines()
        .array_chunks::<3>()
        .map(|[line0, line1, line2]| {
            let mut score = 0;
            for char in line0.chars() {
                if line1.contains(char) && line2.contains(char) {
                    score = if char.is_ascii_lowercase() {
                        (char as u8 - b'a' + 1) as u64
                    } else {
                        (char as u8 - b'A' + 27) as u64
                    };
                    break;
                }
            }
            score
        })
        .sum();

    (part1, part2)
}
