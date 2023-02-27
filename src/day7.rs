pub fn solve(input: &str) -> (u64, u64) {
    let mut part1 = 0;

    let mut current_path = vec![];
    for line in input.lines() {
        match line.split_once(' ').unwrap() {
            ("dir", _) => (),
            ("$", "ls") => (),
            ("$", "cd ..") => {
                let size = current_path.pop().unwrap();
                if let Some(folder_size) = current_path.last_mut() {
                    *folder_size += size;
                }

                if size <= 100_000 {
                    part1 += size;
                }
            }
            ("$", _) => {
                current_path.push(0);
            }
            (size, _) => {
                *current_path.last_mut().unwrap() += size.parse::<u64>().unwrap();
            }
        }
    }

    for _ in 0..current_path.len() - 1 {
        let size = current_path.pop().unwrap();
        if let Some(folder_size) = current_path.last_mut() {
            *folder_size += size;
        }

        if size <= 100_000 {
            part1 += size;
        }
    }

    // Part 2
    let mut max_size = 0;
    let mut current_path = vec![];
    for line in input.lines() {
        match line.split_once(' ').unwrap() {
            ("dir", _) => (),
            ("$", "ls") => (),
            ("$", "cd ..") => {
                let size = current_path.pop().unwrap();
                if let Some(folder_size) = current_path.last_mut() {
                    *folder_size += size;
                }

                if size > max_size {
                    max_size = size;
                }
            }
            ("$", _) => {
                current_path.push(0);
            }
            (size, _) => {
                *current_path.last_mut().unwrap() += size.parse::<u64>().unwrap();
            }
        }
    }

    for _ in 0..current_path.len() {
        let size = current_path.pop().unwrap();
        if let Some(folder_size) = current_path.last_mut() {
            *folder_size += size;
        }

        if size > max_size {
            max_size = size;
        }
    }
    let current_size = 70_000_000 - max_size;

    let mut part2 = u64::MAX;
    let mut current_path = vec![];
    for line in input.lines() {
        match line.split_once(' ').unwrap() {
            ("dir", _) => (),
            ("$", "ls") => (),
            ("$", "cd ..") => {
                let size = current_path.pop().unwrap();
                *current_path.last_mut().unwrap() += size;

                if current_size + size > 30_000_000 && size < part2 {
                    part2 = size;
                }
            }
            ("$", _) => {
                current_path.push(0);
            }
            (size, _) => {
                *current_path.last_mut().unwrap() += size.parse::<u64>().unwrap();
            }
        }
    }

    for _ in 0..current_path.len() - 1 {
        let size = current_path.pop().unwrap();
        if let Some(folder_size) = current_path.last_mut() {
            *folder_size += size;
        }

        if current_size + size > 30_000_000 && size < part2 {
            part2 = size;
        }
    }

    (part1, part2)
}
