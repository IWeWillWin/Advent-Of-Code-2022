use std::collections::HashSet;

pub fn solve(input: &str) -> (u64, u64) {
    let cubes = input
        .lines()
        .flat_map(|str| {
            str.split(',')
                .map(|number| number.parse::<u32>().unwrap())
                .array_chunks::<3>()
        })
        .collect::<HashSet<_>>();

    let part1 = cubes
        .iter()
        .cloned()
        .map(|[x, y, z]| {
            let mut area = 0;
            if !cubes.contains(&[x - 1, y, z]) {
                area += 1;
            }
            if !cubes.contains(&[x + 1, y, z]) {
                area += 1;
            }

            if !cubes.contains(&[x, y - 1, z]) {
                area += 1;
            }
            if !cubes.contains(&[x, y + 1, z]) {
                area += 1;
            }

            if !cubes.contains(&[x, y, z - 1]) {
                area += 1;
            }
            if !cubes.contains(&[x, y, z + 1]) {
                area += 1;
            }
            area
        })
        .sum();

    let [max_x, max_y, max_z] =
        cubes
            .iter()
            .cloned()
            .fold([0, 0, 0], |[max_x, max_y, max_z], [x, y, z]| {
                [u32::max(max_x, x), u32::max(max_y, y), u32::max(max_z, z)]
            });

    let mut air_pockets = HashSet::new();
    for x in 0..=max_x {
        for y in 0..=max_y {
            for z in 0..=max_z {
                if !cubes.contains(&[x, y, z]) {
                    air_pockets.insert([x, y, z]);
                }
            }
        }
    }

    let mut part2 = part1;
    while !air_pockets.is_empty() {
        let mut exposed = false;
        let mut surface_area = 0;

        let mut unexplored = vec![air_pockets.iter().next().cloned().unwrap()];
        air_pockets.remove(unexplored.last().unwrap());

        while !unexplored.is_empty() {
            let [x, y, z] = unexplored.pop().unwrap();
            if cubes.contains(&[x - 1, y, z]) {
                surface_area += 1;
            } else if x != 0 && air_pockets.contains(&[x - 1, y, z]) {
                air_pockets.remove(&[x - 1, y, z]);
                unexplored.push([x - 1, y, z]);
            }
            if cubes.contains(&[x + 1, y, z]) {
                surface_area += 1;
            } else if air_pockets.contains(&[x + 1, y, z]) {
                air_pockets.remove(&[x + 1, y, z]);
                unexplored.push([x + 1, y, z]);
            }

            if cubes.contains(&[x, y - 1, z]) {
                surface_area += 1;
            } else if y != 0 && air_pockets.contains(&[x, y - 1, z]) {
                air_pockets.remove(&[x, y - 1, z]);
                unexplored.push([x, y - 1, z]);
            }

            if cubes.contains(&[x, y + 1, z]) {
                surface_area += 1;
            } else if air_pockets.contains(&[x, y + 1, z]) {
                air_pockets.remove(&[x, y + 1, z]);
                unexplored.push([x, y + 1, z]);
            }

            if cubes.contains(&[x, y, z - 1]) {
                surface_area += 1;
            } else if z != 0 && air_pockets.contains(&[x, y, z - 1]) {
                air_pockets.remove(&[x, y, z - 1]);
                unexplored.push([x, y, z - 1]);
            }
            if cubes.contains(&[x, y, z + 1]) {
                surface_area += 1;
            } else if air_pockets.contains(&[x, y, z + 1]) {
                air_pockets.remove(&[x, y, z + 1]);
                unexplored.push([x, y, z + 1]);
            }

            if x == 0 || x == max_x || y == 0 || y == max_y || z == 0 || z == max_z {
                exposed = true;
            }
        }

        if !exposed {
            part2 -= surface_area;
        }
    }

    (part1, part2)
}
