use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> (u64, u64) {
    let mut elves = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
        .filter_map(|((x, y), c)| {
            if c == '#' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    let mut next_valid = Direction::North;
    let mut blocked = HashSet::new();
    let mut proposed = HashMap::new();
    for _ in 1..=10 {
        blocked.clear();
        proposed.clear();
        for (x, y) in elves.iter().cloned() {
            let checked = [
                !elves.contains(&(x - 1, y - 1)),
                !elves.contains(&(x, y - 1)),
                !elves.contains(&(x + 1, y - 1)),
                !elves.contains(&(x - 1, y)),
                !elves.contains(&(x + 1, y)),
                !elves.contains(&(x - 1, y + 1)),
                !elves.contains(&(x, y + 1)),
                !elves.contains(&(x + 1, y + 1)),
            ];

            if checked.iter().all(|bool| *bool)
            {
                continue;
            }

            let picks = [
                // North
                checked[0] && checked[1] && checked[2],
                // South
                checked[5] && checked[6] && checked[7],
                // West
                checked[0] && checked[3] && checked[5],
                // East
                checked[2] && checked[4] && checked[7],
            ];

            if let Some((i, _)) = picks
                .into_iter()
                .enumerate()
                .cycle()
                .skip(next_valid as usize)
                .take(4)
                .find(|(_, bool)| *bool)
            {
                let (new_x, new_y) = match Direction::from(i) {
                    Direction::North => (x, y - 1),
                    Direction::South => (x, y + 1),
                    Direction::West => (x - 1, y),
                    Direction::East => (x + 1, y),
                };
                if blocked.insert((new_x, new_y)) {
                    proposed.insert((new_x, new_y), (x, y));
                } else {
                    proposed.remove(&(new_x, new_y));
                }
            }
        }
        next_valid = next_valid.next();

        for (&(new_x, new_y), &(old_x, old_y)) in proposed.iter() {
            elves.remove(&(old_x, old_y));
            elves.insert((new_x, new_y));
        }
    }

    let min_x = elves.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = elves.iter().map(|(x, _)| *x).max().unwrap();
    let width = (max_x + 1) - min_x;

    let min_y = elves.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = elves.iter().map(|(_, y)| *y).max().unwrap();
    let height = (max_y + 1) - min_y;

    let part1 = (width as u64 * height as u64) - elves.len() as u64;

    let mut elves = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
        .filter_map(|((x, y), c)| {
            if c == '#' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    let mut part2 = 0;
    let mut next_valid = Direction::North;
    let mut blocked = HashSet::new();
    let mut proposed = HashMap::new();
    loop {
        part2 += 1;

        blocked.clear();
        proposed.clear();
        for (x, y) in elves.iter().cloned() {
            let checked = [
                !elves.contains(&(x - 1, y - 1)),
                !elves.contains(&(x, y - 1)),
                !elves.contains(&(x + 1, y - 1)),
                !elves.contains(&(x - 1, y)),
                !elves.contains(&(x + 1, y)),
                !elves.contains(&(x - 1, y + 1)),
                !elves.contains(&(x, y + 1)),
                !elves.contains(&(x + 1, y + 1)),
            ];

            if checked.iter().all(|bool| *bool)
            {
                continue;
            }

            let picks = [
                // North
                checked[0] && checked[1] && checked[2],
                // South
                checked[5] && checked[6] && checked[7],
                // West
                checked[0] && checked[3] && checked[5],
                // East
                checked[2] && checked[4] && checked[7],
            ];

            if let Some((i, _)) = picks
                .into_iter()
                .enumerate()
                .cycle()
                .skip(next_valid as usize)
                .take(4)
                .find(|(_, bool)| *bool)
            {
                let (new_x, new_y) = match Direction::from(i) {
                    Direction::North => (x, y - 1),
                    Direction::South => (x, y + 1),
                    Direction::West => (x - 1, y),
                    Direction::East => (x + 1, y),
                };
                if blocked.insert((new_x, new_y)) {
                    proposed.insert((new_x, new_y), (x, y));
                } else {
                    proposed.remove(&(new_x, new_y));
                }
            }
        }
        next_valid = next_valid.next();

        if proposed.is_empty() {
            break;
        }

        for (&(new_x, new_y), &(old_x, old_y)) in proposed.iter() {
            elves.remove(&(old_x, old_y));
            elves.insert((new_x, new_y));
        }
    }

    (part1, part2)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North = 0,
    South = 1,
    West = 2,
    East = 3,
}

impl Direction {
    fn next(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::West,
            Self::West => Self::East,
            Self::East => Self::North,
        }
    }
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::North,
            1 => Self::South,
            2 => Self::West,
            3 => Self::East,
            _ => panic!(),
        }
    }
}
