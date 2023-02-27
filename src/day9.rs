use std::collections::HashSet;

pub fn solve(input: &str) -> (u64, u64) {
    let mut tail_x = 0;
    let mut tail_y = 0;

    let mut head_x = 0;
    let mut head_y = 0;

    let mut visited = HashSet::new();
    visited.insert((0, 0));
    input.lines().for_each(|line| {
        let (dir, step) = line.split_at(1);
        let steps = step.trim_start().parse::<u8>().unwrap();

        match dir {
            "R" => {
                for _ in 0..steps {
                    head_x += 1;
                    adjust_tail(head_x, head_y, &mut tail_x, &mut tail_y);
                    visited.insert((tail_x, tail_y));
                }
            }
            "L" => {
                for _ in 0..steps {
                    head_x -= 1;
                    adjust_tail(head_x, head_y, &mut tail_x, &mut tail_y);
                    visited.insert((tail_x, tail_y));
                }
            }
            "U" => {
                for _ in 0..steps {
                    head_y += 1;
                    adjust_tail(head_x, head_y, &mut tail_x, &mut tail_y);
                    visited.insert((tail_x, tail_y));
                }
            }
            "D" => {
                for _ in 0..steps {
                    head_y -= 1;
                    adjust_tail(head_x, head_y, &mut tail_x, &mut tail_y);
                    visited.insert((tail_x, tail_y));
                }
            }
            _ => panic!(),
        }
    });
    let part1 = visited.len() as u64;

    let mut knots = [(0, 0); 10];
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    input.lines().for_each(|line| {
        let (dir, step) = line.split_at(1);
        let steps = step.trim_start().parse::<u8>().unwrap();

        match dir {
            "R" => {
                for _ in 0..steps {
                    knots[0].0 += 1;

                    for i in 1..10 {
                        adjust_tail(
                            knots[i - 1].0,
                            knots[i - 1].1,
                            &mut knots[i].0,
                            &mut knots[i].1,
                        );
                    }

                    visited.insert(knots[9]);
                }
            }
            "L" => {
                for _ in 0..steps {
                    knots[0].0 -= 1;
                    for i in 1..10 {
                        adjust_tail(
                            knots[i - 1].0,
                            knots[i - 1].1,
                            &mut knots[i].0,
                            &mut knots[i].1,
                        );
                    }
                    visited.insert(knots[9]);
                }
            }
            "U" => {
                for _ in 0..steps {
                    knots[0].1 += 1;
                    for i in 1..10 {
                        adjust_tail(
                            knots[i - 1].0,
                            knots[i - 1].1,
                            &mut knots[i].0,
                            &mut knots[i].1,
                        );
                    }
                    visited.insert(knots[9]);
                }
            }
            "D" => {
                for _ in 0..steps {
                    knots[0].1 -= 1;

                    for i in 1..10 {
                        adjust_tail(
                            knots[i - 1].0,
                            knots[i - 1].1,
                            &mut knots[i].0,
                            &mut knots[i].1,
                        );
                    }
                    visited.insert(knots[9]);
                }
            }
            _ => panic!(),
        }
    });
    let part2 = visited.len() as u64;

    (part1, part2)
}

fn adjust_tail(head_x: i32, head_y: i32, tail_x: &mut i32, tail_y: &mut i32) {
    if !touching(head_x, head_y, *tail_x, *tail_y) {
        if !diagonal(head_x, head_y, *tail_x, *tail_y) {
            if *tail_x - 2 == head_x {
                *tail_x -= 1;
            } else if *tail_x + 2 == head_x {
                *tail_x += 1;
            } else if *tail_y - 2 == head_y {
                *tail_y -= 1;
            } else if *tail_y + 2 == head_y {
                *tail_y += 1;
            }
        } else {
            if *tail_x - 2 == head_x {
                *tail_x -= 1;
            } else if *tail_x + 2 == head_x {
                *tail_x += 1;
            } else if *tail_x - 1 == head_x {
                *tail_x -= 1;
            } else if *tail_x + 1 == head_x {
                *tail_x += 1;
            }

            if *tail_y - 2 == head_y {
                *tail_y -= 1;
            } else if *tail_y + 2 == head_y {
                *tail_y += 1;
            } else if *tail_y - 1 == head_y {
                *tail_y -= 1;
            } else if *tail_y + 1 == head_y {
                *tail_y += 1;
            }
        }
    }
}

fn touching(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> bool {
    (head_x == tail_x || head_x == tail_x - 1 || head_x == tail_x + 1)
        && (head_y == tail_y || head_y == tail_y - 1 || head_y == tail_y + 1)
}

fn diagonal(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> bool {
    tail_x != head_x && tail_y != head_y
}
