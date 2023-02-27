use std::collections::HashMap;

pub fn solve(input: &str) -> (u64, u64) {
    let (map_str, instruction_stream) = input.split_once("\n\n").unwrap();
    let map = map_str
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let y = y as i32;
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(((x as i32, y), Tile::Block)),
                '.' => Some(((x as i32, y), Tile::Empty)),
                ' ' => None,
                _ => panic!(),
            })
        })
        .collect::<HashMap<_, _>>();

    let mut instructions = Vec::new();
    let mut chars = instruction_stream.chars();
    while chars.clone().next().is_some() {
        let mut number = 0;
        while let Some(c @ '0'..='9') = chars.clone().next() {
            chars.next();

            number *= 10;
            number += (c as u8 - b'0') as u32;
        }

        if number != 0 {
            instructions.push(Instruction::Move(number));
        } else {
            let direction = match chars.clone().next() {
                Some('R') => Instruction::TurnRight,
                Some('L') => Instruction::TurnLeft,
                None => break,

                Some(c) => panic!("Unexpected character: {c}"),
            };
            chars.next();
            instructions.push(direction);
        }
    }

    let mut current_y = 0;
    let mut current_x = map
        .iter()
        .filter_map(|(&(x, y), tile)| {
            if let Tile::Empty = tile {
                if current_y == y {
                    Some(x)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .min()
        .unwrap();
    let mut current_direction = Direction::East;

    for instruction in instructions.iter().cloned() {
        match instruction {
            Instruction::Move(value) => {
                for _ in 0..value {
                    let new_x = current_x + current_direction.x_offset();
                    let new_y = current_y + current_direction.y_offset();

                    if let Some(Tile::Empty) = map.get(&(new_x, new_y)) {
                        current_x = new_x;
                        current_y = new_y;
                    } else if let Some(Tile::Block) = map.get(&(new_x, new_y)) {
                        break;
                    } else {
                        let new_x = match current_direction {
                            Direction::East => map
                                .iter()
                                .filter(|(&(_, y), _)| y == current_y)
                                .fold(i32::MAX, |accum, ((x, _), _)| i32::min(accum, *x)),
                            Direction::West => map
                                .iter()
                                .filter(|(&(_, y), _)| y == current_y)
                                .fold(0, |accum, ((x, _), _)| i32::max(accum, *x)),
                            Direction::North | Direction::South => current_x,
                        };
                        let new_y = match current_direction {
                            Direction::South => map
                                .iter()
                                .filter(|(&(x, _), _)| x == current_x)
                                .fold(i32::MAX, |accum, ((_, y), _)| i32::min(accum, *y)),
                            Direction::North => map
                                .iter()
                                .filter(|(&(x, _), _)| x == current_x)
                                .fold(0, |accum, ((_, y), _)| i32::max(accum, *y)),
                            Direction::East | Direction::West => current_y,
                        };

                        if let Some(Tile::Empty) = map.get(&(new_x, new_y)) {
                            current_x = new_x;
                            current_y = new_y;
                        } else {
                            break;
                        }
                    }
                }
            }
            Instruction::TurnLeft => current_direction.turn_left(),
            Instruction::TurnRight => current_direction.turn_right(),
        }
    }
    let part1 =
        1000 * (current_y as u64 + 1) + 4 * (current_x as u64 + 1) + current_direction.score();

    // Part Two
    let width = map.iter().map(|((x, _), _)| *x).max().unwrap() + 1;
    let height = map.iter().map(|((_, y), _)| *y).max().unwrap() + 1;
    let side_length = gcd(width, height);

    let mut index = 0;
    let mut squares = [(0, 0); 6];
    for y in (0..height).step_by(side_length as usize) {
        for x in (0..width).step_by(side_length as usize) {
            if map.contains_key(&(x, y)) {
                squares[index] = (x, y);
                index += 1;
            }
        }
    }

    // [North, East, South, West]
    let mut transitions = [[(u8::MAX, Direction::North); 4]; 6];
    for (i, &(origin_x, origin_y)) in squares.iter().enumerate() {
        if let Some(index) = squares
            .iter()
            .position(|(x, y)| *x == origin_x && *y + side_length == origin_y)
        {
            transitions[i][0] = (index as u8, Direction::North);
        }
        if let Some(index) = squares
            .iter()
            .position(|(x, y)| *x == origin_x + side_length && *y == origin_y)
        {
            transitions[i][1] = (index as u8, Direction::East);
        }
        if let Some(index) = squares
            .iter()
            .position(|(x, y)| *x == origin_x && *y - side_length == origin_y)
        {
            transitions[i][2] = (index as u8, Direction::South);
        }
        if let Some(index) = squares
            .iter()
            .position(|(x, y)| *x == origin_x - side_length && *y == origin_y)
        {
            transitions[i][3] = (index as u8, Direction::West);
        }
    }

    let mut connections = Vec::new();
    let mut edges = vec![(0, Direction::North)];
    loop {
        let &(last_face, last_dir) = edges.last().unwrap();
        if transitions[last_face as usize][last_dir.right() as usize].0 != 255 {
            let next_face = transitions[last_face as usize][last_dir.right() as usize].0;
            if transitions[next_face as usize][last_dir as usize].0 == 255 {
                // Straight
                let next = (next_face, last_dir);
                if edges.contains(&(next)) {
                    break;
                }
                edges.push((next_face, last_dir));
            } else {
                // Concave corner
                let next_face = transitions[next_face as usize][last_dir as usize].0;
                let next = (next_face, last_dir.left());
                if edges.contains(&(next)) {
                    break;
                }
                connections.push((edges.len(), edges.len() - 1));
                edges.push(next);
            }
        } else {
            // Convex corner
            let next = (last_face, last_dir.right());
            if edges.contains(&(next)) {
                break;
            }
            edges.push(next);
        }
    }

    while !connections.is_empty() {
        let (i, j) = connections.pop().unwrap();
        let (face0, dir0) = edges[i];
        let (face1, dir1) = edges[j];

        transitions[face0 as usize][dir0 as usize] = (face1, dir1.opposite());
        transitions[face1 as usize][dir1 as usize] = (face0, dir0.opposite());

        let next_i = (i + 1) % edges.len();
        let next_j = (j - 1) % edges.len();

        let (next_face0, next_dir0) = edges[next_i];
        let (next_face1, next_dir1) = edges[next_j];

        if next_dir0 != dir0 && next_dir1 != dir1 {
            continue;
        }
        if transitions[next_face0 as usize][next_dir0 as usize].0 != 255
            || transitions[next_face1 as usize][next_dir1 as usize].0 != 255
        {
            continue;
        }
        connections.push((next_i, next_j));
    }

    let mut current_y = 0;
    let mut current_x = map
        .iter()
        .filter_map(|(&(x, y), tile)| {
            if let Tile::Empty = tile {
                if current_y == y {
                    Some(x)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .min()
        .unwrap();
    let mut current_direction = Direction::East;

    let mut visited = HashMap::new();
    for instruction in instructions.iter().cloned() {
        match instruction {
            Instruction::Move(value) => {
                for _ in 0..value {
                    visited
                        .entry((current_x, current_y))
                        .or_insert(current_direction);
                    let new_x = current_x + current_direction.x_offset();
                    let new_y = current_y + current_direction.y_offset();

                    if let Some(Tile::Empty) = map.get(&(new_x, new_y)) {
                        current_x = new_x;
                        current_y = new_y;
                    } else if let Some(Tile::Block) = map.get(&(new_x, new_y)) {
                        break;
                    } else {
                        let current_tile = squares
                            .iter()
                            .position(|&(start_x, start_y)| {
                                (start_x <= current_x && current_x < start_x + side_length)
                                    && (start_y <= current_y && current_y < start_y + side_length)
                            })
                            .unwrap();

                        let (next_tile, new_direction) =
                            transitions[current_tile][current_direction as usize];
                        let (start_x, start_y) = squares[next_tile as usize];

                        let tile_x = current_x % side_length;
                        let tile_y = current_y % side_length;

                        let (new_x, new_y) = match (current_direction, new_direction) {
                            (Direction::North, Direction::North) => {
                                (start_x + tile_x, start_y + side_length - 1)
                            }
                            (Direction::North, Direction::East) => (start_x, start_y + tile_x),
                            (Direction::North, Direction::South) => {
                                (start_x + side_length - 1 - tile_x, start_y)
                            }
                            (Direction::North, Direction::West) => {
                                (start_x + side_length - 1, start_y + side_length - 1 - tile_x)
                            }
                            (Direction::East, Direction::North) => {
                                (start_x + tile_y, start_y + side_length - 1)
                            }
                            (Direction::East, Direction::East) => (start_x, start_y + tile_y),
                            (Direction::East, Direction::South) => {
                                (start_x + side_length - 1 - tile_y, start_y)
                            }
                            (Direction::East, Direction::West) => {
                                (start_x + side_length - 1, start_y + side_length - 1 - tile_y)
                            }
                            (Direction::South, Direction::North) => (
                                start_x + side_length - 1 - tile_x,
                                start_y + side_length - 1,
                            ),
                            (Direction::South, Direction::East) => {
                                (start_x, start_y + side_length - 1 - tile_x)
                            }
                            (Direction::South, Direction::South) => (start_x + tile_x, start_y),
                            (Direction::South, Direction::West) => {
                                (start_x + side_length - 1, start_y + tile_x)
                            }
                            (Direction::West, Direction::North) => {
                                (start_x + side_length - 1 - tile_y, start_y + side_length - 1)
                            }
                            (Direction::West, Direction::East) => (start_x, start_y + tile_y),
                            (Direction::West, Direction::South) => {
                                (start_x + tile_y, start_y)
                            }
                            (Direction::West, Direction::West) => {
                                (start_x + side_length - 1, start_y + tile_y)
                            }
                        };

                        if let Some(Tile::Empty) = map.get(&(new_x, new_y)) {
                            current_direction = new_direction;
                            current_x = new_x;
                            current_y = new_y;
                        } else {
                            break;
                        }
                    }
                }
            }
            Instruction::TurnLeft => current_direction.turn_left(),
            Instruction::TurnRight => current_direction.turn_right(),
        }
    }
    let part2 =
        1000 * (current_y as u64 + 1) + 4 * (current_x as u64 + 1) + current_direction.score();

    (part1, part2)
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Block,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Move(u32),
    TurnRight,
    TurnLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn score(&self) -> u64 {
        match self {
            Self::East => 0,
            Self::South => 1,
            Self::West => 2,
            Self::North => 3,
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    fn x_offset(&self) -> i32 {
        match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 0,
            Self::West => -1,
        }
    }

    fn y_offset(&self) -> i32 {
        match self {
            Self::North => -1,
            Self::East => 0,
            Self::South => 1,
            Self::West => 0,
        }
    }

    fn left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn turn_left(&mut self) {
        *self = match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        };
    }

    fn turn_right(&mut self) {
        *self = match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        };
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// RdYKjZDH
