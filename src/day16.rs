use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) -> (u32, u32) {
    let valves = input
        .lines()
        .map(|line| {
            let (valve, tunnels) = line.split_once("; ").unwrap();
            let (name, flow) = valve.split_once("has flow rate=").unwrap();
            let name = [
                name["Valve ".len()..].as_bytes()[0],
                name["Valve ".len()..].as_bytes()[1],
            ];
            let flow = flow.parse::<u8>().unwrap();

            let tunnels = tunnels.replace(',', "");
            let tunnels = tunnels
                .split(' ')
                .filter(|str| str.len() == 2)
                .skip(1)
                .map(|str| [str.as_bytes()[0], str.as_bytes()[1]])
                .collect::<Vec<_>>();

            (name, flow, tunnels)
        })
        .collect::<Vec<_>>();
    let start_index = valves
        .iter()
        .position(|(label, _, _)| label == b"AA")
        .unwrap() as u32;

    let mut optimized = Vec::new();
    for root in 0..valves.len() as u32 {
        let mut queue = VecDeque::new();
        let mut explored = vec![root];
        queue.push_back((root, 0u32));

        let mut tunnels = Vec::new();
        while !queue.is_empty() {
            let (index, depth) = queue.pop_front().unwrap();

            for index in valves[index as usize]
                .2
                .iter()
                .map(|label| valves.iter().position(|(id, _, _)| *label == *id).unwrap() as u32)
            {
                if !explored.contains(&index) {
                    queue.push_back((index, depth + 1));
                    explored.push(index);

                    if valves[index as usize].1 != 0 {
                        tunnels.push((valves[index as usize].0, depth + 1));
                    }
                }
            }
        }
        optimized.push((valves[root as usize].0, valves[root as usize].1, tunnels));
    }

    let optimized = optimized
        .iter()
        .map(|(_, flow, tunnels)| {
            let tunnels = tunnels
                .iter()
                .map(|(label, cost)| {
                    (
                        optimized
                            .iter()
                            .position(|(id, _, _)| *label == *id)
                            .unwrap() as u32,
                        *cost + 1,
                    )
                })
                .collect::<Vec<_>>();

            (*flow, tunnels)
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;
    let mut to_explore = vec![(0, start_index, 0, vec![])];
    while !to_explore.is_empty() {
        let (time, position, released, explored) = to_explore.pop().unwrap();

        let available_time = 30 - time;
        for (tunnel, cost) in optimized[position as usize].1.iter() {
            if *cost < available_time && !explored.contains(tunnel) {
                let opened_time = available_time - cost;
                let released = released + opened_time * optimized[*tunnel as usize].0 as u32;

                let mut explored = explored.clone();
                explored.push(*tunnel);

                to_explore.push((time + cost, *tunnel, released, explored));
            }
        }

        if released > part1 {
            part1 = released;
        }
    }

    let mut fastest = HashMap::new();
    let mut to_explore = vec![(0, start_index, 0, vec![])];
    while !to_explore.is_empty() {
        let (time, position, released, mut explored) = to_explore.pop().unwrap();

        let available_time = 26 - time;
        for (tunnel, cost) in optimized[position as usize].1.iter() {
            if *cost < available_time && !explored.contains(tunnel) {
                let opened_time = available_time - cost;
                let released = released + opened_time * optimized[*tunnel as usize].0 as u32;

                let mut explored = explored.clone();
                explored.push(*tunnel);

                to_explore.push((time + cost, *tunnel, released, explored));
            }
        }
        explored.sort();

        if let Some(total) = fastest.get(&explored) {
            if *total < released {
                fastest.insert(explored, released);
            }
        } else {
            fastest.insert(explored, released);
        }
    }

    let mut part2 = 0;
    for (human_visited, human_released) in fastest.iter() {
        for (elephant_visited, elephant_released) in fastest.iter() {
            if human_released + elephant_released > part2
                && human_visited
                    .iter()
                    .all(|item| !elephant_visited.contains(item))
            {
                part2 = *human_released + *elephant_released;
            }
        }
    }

    (part1, part2)
}
