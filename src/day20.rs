pub fn solve(input: &str) -> (u64, u64) {
    let mut array = input
        .lines()
        .enumerate()
        .map(|(i, line)| (i as u32, line.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();
    let movements = array.clone();

    for (i, movement) in movements.iter().cloned().enumerate() {
        let old_index = array
            .iter()
            .position(|(initial, _)| *initial == i as u32)
            .unwrap();
        array.remove(old_index);
        let new_index =
            (old_index as isize + movement.1 as isize).rem_euclid(array.len() as isize) as usize;
        array.insert(new_index, movement);
    }

    let part1 = array
        .into_iter()
        .map(|(_, element)| element)
        .cycle()
        .skip_while(|element| *element != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum::<i32>();

    let mut array = input
        .lines()
        .enumerate()
        .map(|(i, line)| (i as u32, line.parse::<i64>().unwrap() * 811589153))
        .collect::<Vec<_>>();
    let movements = array.clone();

    for _ in 0..10 {
        for (i, movement) in movements.iter().cloned().enumerate() {
            let old_index = array
                .iter()
                .position(|(initial, _)| *initial == i as u32)
                .unwrap();
            array.remove(old_index);
            let new_index = (old_index as isize + movement.1 as isize)
                .rem_euclid(array.len() as isize) as usize;
            array.insert(new_index, movement);
        }
    }

    let part2 = array
        .into_iter()
        .map(|(_, element)| element)
        .cycle()
        .skip_while(|element| *element != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum::<i64>();

    (part1 as u64, part2 as u64)
}
