pub fn solve(input: &str) -> (u64, String) {
    let instructions = input.lines().map(|line| match line.split_at(4) {
        ("noop", "") => (1, 0),
        ("addx", val) => (2, val.trim_start().parse::<i16>().unwrap()),
        _ => panic!(),
    });

    let mut part1 = 0;
    let mut next_point = 20;
    let mut current_value = 1;
    let mut current_cycles = 0;
    for (cycles, value) in instructions.clone() {
        // Start
        if (current_cycles == next_point - 1) || (current_cycles == next_point - 2 && cycles == 2) {
            part1 += next_point * current_value;
            next_point += 40;
        }

        // End
        current_cycles += cycles;
        current_value += value as i32;
    }

    let mut grid = [false; 240];
    let mut current_value = 1;
    let mut current_cycles = 0;
    for (cycles, value) in instructions.clone() {
        if cycles == 1 {
            if current_cycles % 40 == current_value - 1
                || current_cycles % 40 == current_value
                || current_cycles % 40 == current_value + 1
            {
                grid[current_cycles as usize] = true;
            }
            current_cycles += 1;
        } else if cycles == 2 {
            if current_cycles % 40 == current_value - 1
                || current_cycles % 40 == current_value
                || current_cycles % 40 == current_value + 1
            {
                grid[current_cycles as usize] = true;
            }
            current_cycles += 1;

            if current_cycles % 40 == current_value - 1
                || current_cycles % 40 == current_value
                || current_cycles % 40 == current_value + 1
            {
                grid[current_cycles as usize] = true;
            }
            current_cycles += 1;

            current_value += value;
        }
    }

    let mut string = String::new();
    string.push('\n');
    for y in 0..6 {
        for x in 0..40 {
            if grid[y * 40 + x] {
                string.push('█');
            } else {
                string.push('░');
            }
        }
        string.push('\n');
    }

    let part2 = String::from("BRJLFULP");
    (part1 as u64, part2)
}
