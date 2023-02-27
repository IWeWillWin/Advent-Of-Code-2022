fn solve_blueprint(blueprint: Blueprint, end_time: u32) -> u32 {
    let max_ore = u32::max(
        u32::max(blueprint.ore_ore, blueprint.clay_ore),
        u32::max(blueprint.obsidian_ore, blueprint.geode_ore)
    );
    let max_clay = blueprint.obsidian_clay;
    let max_obsidian = blueprint.geode_obsidian;

    let mut most_geodes = 0;
    let mut stack = vec![Res::new()];
    while !stack.is_empty() {
        let resources = stack.pop().unwrap();
        assert!(resources.time <= end_time);
        
        if resources.geodes > most_geodes {
            most_geodes = resources.geodes;
        }

        let remaining_time = end_time - resources.time;
        
        let mut potential_geodes = resources.geodes;
        let mut potential_obsidian = resources.obsidian;
        let mut potential_obsidian_bots = resources.obsidian_bots;
        for i in (0..remaining_time).rev() {
            potential_obsidian += potential_obsidian_bots;
            potential_obsidian_bots += 1;

            if potential_obsidian >= blueprint.geode_obsidian {
                potential_obsidian -= blueprint.geode_obsidian;
                potential_geodes += i;
            }
        }

        if potential_geodes < most_geodes {
            continue;
        }
        
        if resources.ore_bots > 0 && resources.ore_bots < max_ore && resources.ore < remaining_time * max_ore && remaining_time > 2 {
            let mut resources = resources;
            while resources.time < end_time {
                if resources.ore >= blueprint.ore_ore {
                    resources.ore -= blueprint.ore_ore;
                    resources.tick();
                    
                    resources.ore_bots += 1;
                    
                    stack.push(resources);
                    
                    break;
                }
                resources.tick();
            }
        }

        if resources.ore_bots > 0 && resources.clay_bots < max_clay && resources.clay < remaining_time * max_clay && remaining_time > 4 {
            let mut resources = resources;
            while resources.time < end_time {
                if resources.ore >= blueprint.clay_ore {
                    resources.ore -= blueprint.clay_ore;
                    resources.tick();
                    
                    resources.clay_bots += 1;

                    stack.push(resources);
                    
                    break;
                }
                resources.tick();
            }
        }

        if resources.ore_bots > 0 && resources.clay_bots > 0 && resources.obsidian_bots < max_obsidian && resources.obsidian < remaining_time * max_obsidian && remaining_time > 2 {
            let mut resources = resources;
            while resources.time < end_time {
                if resources.ore >= blueprint.obsidian_ore && resources.clay >= blueprint.obsidian_clay {
                    resources.clay -= blueprint.obsidian_clay;
                    resources.ore -= blueprint.obsidian_ore;
                    resources.tick();
                    
                    resources.obsidian_bots += 1;
                    
                    stack.push(resources);
                    
                    break;
                }
                resources.tick();
            }
        }

        if resources.ore_bots > 0 && resources.obsidian_bots > 0 && remaining_time > 0 {
            let mut resources = resources;
            while resources.time < end_time {
                if resources.ore >= blueprint.geode_ore && resources.obsidian >= blueprint.geode_obsidian {
                    resources.obsidian -= blueprint.geode_obsidian;
                    resources.ore -= blueprint.geode_ore;
                    resources.tick();
                    
                    resources.geodes += end_time - resources.time;

                    stack.push(resources);
                    
                    break;
                }
                resources.tick();
            }
        }
    }
    most_geodes
}

#[derive(Debug, Clone, Copy)]
pub struct Res {
    pub ore: u32,
    pub clay: u32,
    pub obsidian: u32,

    pub ore_bots: u32,
    pub clay_bots: u32,
    pub obsidian_bots: u32,

    pub geodes: u32,
    pub time: u32,
}

impl Res {
    pub fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,

            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,

            geodes: 0,
            time: 0,
        }
    }

    pub fn tick(&mut self) {
        self.ore += self.ore_bots;
        self.clay += self.clay_bots;
        self.obsidian += self.obsidian_bots;
        self.time += 1;
    }
}

pub struct Blueprint {
    pub ore_ore: u32,

    pub clay_ore: u32,

    pub obsidian_ore: u32,
    pub obsidian_clay: u32,

    pub geode_ore: u32,
    pub geode_obsidian: u32,
}

pub fn solve(input: &str) -> (u64, u64) {
    let blueprints = input
        .replace(':', "")
        .split_ascii_whitespace()
        .filter(|word| word.chars().all(|c| c.is_ascii_digit()))
        .map(|number| number.parse::<u32>().unwrap())
        .array_chunks::<7>()
        .collect::<Vec<_>>();

    let mut part1 = 0;
    for blueprint in blueprints.iter() {
        let id = blueprint[0];
        let blueprint = Blueprint {
            ore_ore: blueprint[1],
            clay_ore: blueprint[2],
            obsidian_ore: blueprint[3],
            obsidian_clay: blueprint[4],
            geode_ore: blueprint[5],
            geode_obsidian: blueprint[6],
        };
        let result = solve_blueprint(blueprint, 24);
        part1 += id as u64 * result as u64;
    }

    let mut part2 = 1;
    for blueprint in blueprints.iter().take(3) {
        let blueprint = Blueprint {
            ore_ore: blueprint[1],
            clay_ore: blueprint[2],
            obsidian_ore: blueprint[3],
            obsidian_clay: blueprint[4],
            geode_ore: blueprint[5],
            geode_obsidian: blueprint[6],
        };
        let result = solve_blueprint(blueprint, 32);
        part2 *= result as u64;
    }

    (part1, part2)
}

#[derive(Debug, Clone, Copy)]
pub struct Resources {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,

    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,

    time: u8,
}

impl Resources {
    pub fn tick(&self) -> Resources {
        Self {
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,

            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geode: self.geode + self.geode_robots,

            time: self.time + 1,
        }
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,

            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,

            time: 0,
        }
    }
}
