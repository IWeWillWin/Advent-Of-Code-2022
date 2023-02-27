use std::collections::HashSet;

pub fn solve(input: &str) -> (u64, u64) {
    let mut min_x = u32::MAX;
    let mut max_x = 500;

    let mut min_y = 0;
    let mut max_y = u32::MIN;

    let mut grid = HashSet::new();
    input.lines().for_each(|line| {
        let points = line
            .split(" -> ")
            .map(|point| {
                let (x, y) = point.split_once(',').unwrap();
                (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
            })
            .inspect(|(x, y)| {
                min_x = u32::min(min_x, *x);
                max_x = u32::max(max_x, *x);

                min_y = u32::min(min_y, *y);
                max_y = u32::max(max_y, *y);
            })
            .collect::<Vec<_>>();

        for [(x0, y0), (x1, y1)] in points.array_windows::<2>().cloned() {
            if x0 == x1 {
                for y in u32::min(y0, y1)..=u32::max(y0, y1) {
                    grid.insert((x0, y));
                }
            } else {
                for x in u32::min(x0, x1)..=u32::max(x0, x1) {
                    grid.insert((x, y0));
                }
            }
        }
    });

    let part1 = {
        let mut iterations = 0;
        let mut grid = grid.clone();
        'outer: loop {
            let mut current_x = 500;
            let mut current_y = 0;
            loop {
                if current_y >= max_y {
                    break 'outer;
                } else if !grid.contains(&(current_x, current_y + 1)) {
                    current_y += 1;
                } else if !grid.contains(&(current_x - 1, current_y + 1)) {
                    current_x -= 1;
                    current_y += 1;
                } else if !grid.contains(&(current_x + 1, current_y + 1)) {
                    current_x += 1;
                    current_y += 1;
                } else {
                    grid.insert((current_x, current_y));
                    break;
                }
            }
            iterations += 1;
        }
        iterations
    };

    let part2 = {
        let mut fallen_sand = 0;
        'outer: loop {
            let mut current_x = 500;
            let mut current_y = 0;
            loop {
                if current_y + 1 != max_y + 2 && !grid.contains(&(current_x, current_y + 1)) {
                    current_y += 1;
                } else if current_y + 1 != max_y + 2
                    && !grid.contains(&(current_x - 1, current_y + 1))
                {
                    current_x -= 1;
                    current_y += 1;
                } else if current_y + 1 != max_y + 2
                    && !grid.contains(&(current_x + 1, current_y + 1))
                {
                    current_x += 1;
                    current_y += 1;
                } else if current_x == 500 && current_y == 0 {
                    break 'outer;
                } else {
                    grid.insert((current_x, current_y));
                    break;
                }
            }
            fallen_sand += 1;
        }
        fallen_sand + 1
    };

    (part1, part2)
}
