// Bitmaps for blizzards

use std::collections::HashSet;

pub fn solve(input: &str) -> (u64, u64) {
    let width = input.lines().next().unwrap().len() as u8 - 2;
    let height = input.lines().count() as u8 - 2;

    let mut blizzards = input
        .lines()
        .skip(1)
        .take(input.lines().count() - 2)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '^' => Some(((x as u8 - 1, y as u8), Direction::North)),
                'v' => Some(((x as u8 - 1, y as u8), Direction::South)),
                '>' => Some(((x as u8 - 1, y as u8), Direction::East)),
                '<' => Some(((x as u8 - 1, y as u8), Direction::West)),
                '#' | '.' => None,
                _ => panic!(),
            })
        })
        .collect::<Vec<_>>();

    let mut time = find_first_valid_time(0, 0, &mut blizzards, width, height);
    time += find_path_to(0, 0, width - 1, height - 1, &mut blizzards, width, height);

    let part1 = time;

    step_blizzards(&mut blizzards, width, height);
    time += 1;

    time += find_first_valid_time(width - 1, height - 1, &mut blizzards, width, height);
    time += find_path_to(width - 1, height - 1, 0, 0, &mut blizzards, width, height);

    step_blizzards(&mut blizzards, width, height);
    time += 1;

    time += find_first_valid_time(0, 0, &mut blizzards, width, height);
    time += find_path_to(0, 0, width - 1, height - 1, &mut blizzards, width, height);

    let part2 = time;

    (part1, part2)
}

fn find_path_to(
    start_x: u8,
    start_y: u8,
    end_x: u8,
    end_y: u8,
    blizzards: &mut [((u8, u8), Direction)],
    width: u8,
    height: u8,
) -> u64 {
    let mut iterations = 0;
    let mut next = HashSet::new();
    let mut blocked = HashSet::new();
    let mut current = HashSet::new();
    current.insert((start_x, start_y));

    'outer: loop {
        if current.is_empty() {
            iterations += find_first_valid_time(start_x, start_y, blizzards, width, height);
            current.insert((start_x, start_y));
        }
        
        blocked.extend(blizzards.iter().map(|((x, y), _)| (*x, *y)));
        for (current_x, current_y) in current.iter().cloned() {            
            if current_x == end_x && current_y == end_y {
                break 'outer;
            }
            
            if !blocked.contains(&(current_x, current_y)) && !next.contains(&(current_x, current_y))
            {
                next.insert((current_x, current_y));
            }
            
            if current_x > 0
                && !blocked.contains(&(current_x - 1, current_y))
                && !next.contains(&(current_x - 1, current_y))
                {
                    next.insert((current_x - 1, current_y));
                }

                if current_x < width - 1
                && !blocked.contains(&(current_x + 1, current_y))
                && !next.contains(&(current_x + 1, current_y))
                {
                    next.insert((current_x + 1, current_y));
            }
            
            if current_y > 0
            && !blocked.contains(&(current_x, current_y - 1))
            && !next.contains(&(current_x, current_y - 1))
            {
                next.insert((current_x, current_y - 1));
            }
            
            if current_y < height - 1
            && !blocked.contains(&(current_x, current_y + 1))
                && !next.contains(&(current_x, current_y + 1))
                {
                next.insert((current_x, current_y + 1));
            }
        }

        step_blizzards(blizzards, width, height);
        std::mem::swap(&mut current, &mut next);
        blocked.clear();
        next.clear();
        
        iterations += 1;
    }
    iterations
}

fn find_first_valid_time(
    new_x: u8,
    new_y: u8,
    blizzards: &mut [((u8, u8), Direction)],
    width: u8,
    height: u8,
) -> u64 {
    let mut iterations = 0;
    while blizzards
        .iter()
        .any(|((x, y), _)| *x == new_x && *y == new_y)
    {
        step_blizzards(blizzards, width, height);
        iterations += 1;
    }
    iterations
}

fn step_blizzards(blizzards: &mut [((u8, u8), Direction)], width: u8, height: u8) {
    for ((x, y), blizzard) in blizzards.iter_mut() {
        match blizzard {
            Direction::North => {
                if *y > 0 {
                    *y -= 1;
                } else {
                    *y = height - 1;
                }
            }
            Direction::South => {
                if *y < height - 1 {
                    *y += 1;
                } else {
                    *y = 0;
                }
            }
            Direction::East => {
                if *x < width - 1 {
                    *x += 1;
                } else {
                    *x = 0;
                }
            }
            Direction::West => {
                if *x > 0 {
                    *x -= 1;
                } else {
                    *x = width - 1;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}
