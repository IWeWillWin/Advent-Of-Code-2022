pub fn solve(input: &str) -> (u64, u64) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut start_x = 0;
    let mut start_y = 0;

    let mut end_x = 0;
    let mut end_y = 0;

    let mut grid = vec![u8::MAX; width * height];
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid[y * width + x] = if char == 'S' {
                start_x = x;
                start_y = y;
                0
            } else if char == 'E' {
                end_x = x;
                end_y = y;
                b'z' - b'a'
            } else {
                char as u8 - b'a'
            }
        }
    }

    let mut distance = 1;
    let mut current = vec![(end_x, end_y)];
    let mut distances = vec![0; width * height];
    let mut visited = vec![(end_x, end_y)];

    let mut part2 = u64::MAX;
    while !current.is_empty() {
        let mut to_explore = Vec::new();
        while !current.is_empty() {
            let (x, y) = current.pop().unwrap();
            let local_height = grid[y * width + x];

            if local_height == 0 && distance < part2 {
                part2 = distance - 1;
            }

            if x > 0 {
                let x1 = x - 1;
                let y1 = y;

                let h1 = grid[y1 * width + x1];
                if h1 + 1 >= local_height && !visited.contains(&(x1, y1)) {
                    distances[y1 * width + x1] = distance;
                    to_explore.push((x1, y1));
                    visited.push((x1, y1));
                }
            }
            if x < width - 1 {
                let x1 = x + 1;
                let y1 = y;

                let h1 = grid[y1 * width + x1];
                if h1 + 1 >= local_height && !visited.contains(&(x1, y1)) {
                    distances[y1 * width + x1] = distance;
                    to_explore.push((x1, y1));
                    visited.push((x1, y1));
                }
            }

            if y > 0 {
                let x1 = x;
                let y1 = y - 1;

                let h1 = grid[y1 * width + x1];
                if h1 + 1 >= local_height && !visited.contains(&(x1, y1)) {
                    distances[y1 * width + x1] = distance;
                    to_explore.push((x1, y1));
                    visited.push((x1, y1));
                }
            }
            if y < height - 1 {
                let x1 = x;
                let y1 = y + 1;

                let h1 = grid[y1 * width + x1];
                if h1 + 1 >= local_height && !visited.contains(&(x1, y1)) {
                    distances[y1 * width + x1] = distance;
                    to_explore.push((x1, y1));
                    visited.push((x1, y1));
                }
            }
        }
        std::mem::swap(&mut current, &mut to_explore);
        distance += 1;
    }
    let part1 = distances[start_y * width + start_x];

    (part1, part2)
}
