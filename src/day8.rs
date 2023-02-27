pub fn solve(input: &str) -> (u64, u64) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let grid = input
        .as_bytes()
        .iter()
        .cloned()
        .filter(u8::is_ascii_digit)
        .map(|u8| u8 - b'0')
        .collect::<Vec<_>>();

    let mut part1 = 0;
    for y in 0..height {
        for x in 0..width {
            let tree = grid[y * width + x];

            let left = (0..x).map(|x| grid[y * width + x]).max().unwrap_or(0);
            let right = (x + 1..width)
                .map(|x| grid[y * width + x])
                .max()
                .unwrap_or(0);
            let top = (0..y).map(|y| grid[y * width + x]).max().unwrap_or(0);
            let bottom = (y + 1..height)
                .map(|y| grid[y * width + x])
                .max()
                .unwrap_or(0);

            if (x == 0 || x == width - 1 || y == 0 || y == width - 1)
                || (tree > left || tree > right || tree > top || tree > bottom)
            {
                part1 += 1;
            }
        }
    }

    let mut part2 = 0;
    for y in 0..height {
        for x in 0..width {
            let tree = grid[y * width + x];

            let mut left = 0;
            for x in (0..x).rev() {
                left += 1;
                if tree <= grid[y * width + x] {
                    break;
                }
            }

            let mut right = 0;
            for x in x + 1..width {
                right += 1;
                if tree <= grid[y * width + x] {
                    break;
                }
            }

            let mut up = 0;
            for y in (0..y).rev() {
                up += 1;
                if tree <= grid[y * width + x] {
                    break;
                }
            }

            let mut down = 0;
            for y in y + 1..height {
                down += 1;
                if tree <= grid[y * width + x] {
                    break;
                }
            }

            let score = left * right * up * down;
            if score > part2 {
                part2 = score;
            }
        }
    }

    (part1, part2)
}
