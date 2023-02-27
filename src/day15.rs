pub fn solve(input: &str) -> (u64, u64) {
    let points = input
        .lines()
        .map(|line| {
            line.split(": ")
                .flat_map(|string| string.split(", "))
                .map(|string| {
                    string
                        .chars()
                        .filter(|char| char.is_ascii_digit() || *char == '-')
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap()
                })
                .array_chunks::<4>()
                .map(|[x0, y0, x1, y1]| {
                    (
                        (x0, y0),
                        (x0 - x1).unsigned_abs() + (y0 - y1).unsigned_abs(),
                    )
                })
                .next()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let intervals = calculate_intervals(points.as_slice(), 2_000_000);
    let part1 = intervals.into_iter().map(|(x0, x1)| x1 - x0).sum::<i32>() as u64;

    let mut part2 = 0;
    let search = 4_000_000;
    for y in 0..=search {
        let intervals = calculate_intervals(points.as_slice(), y);
        if intervals.len() == 1 {
            continue;
        } else {
            part2 = (intervals[0].1 + 1) as u64 * 4_000_000 + y as u64;
            break;
        }
    }

    (part1, part2)
}

fn calculate_intervals(points: &[((i32, i32), u32)], y: i32) -> Vec<(i32, i32)> {
    let intervals = points
        .iter()
        .cloned()
        .filter_map(|((point_x, point_y), distance)| {
            let range = distance as i32 - (y - point_y).abs();
            if range >= 0 {
                Some((point_x - range, point_x + range))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    merge_intervals(intervals)
}

fn merge_intervals(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    intervals.sort_unstable_by(|(x0, _), (x1, _)| x0.cmp(x1));

    let mut index = 0;
    for i in 1..intervals.len() {
        if intervals[index].1 + 1 >= intervals[i].0 {
            intervals[index].1 = i32::max(intervals[index].1, intervals[i].1);
        } else {
            index += 1;
            intervals[index] = intervals[i];
        }
    }
    intervals.resize(index + 1, (0, 0));
    intervals
}
