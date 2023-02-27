pub fn solve(input: &str) -> (u64, u64) {
    let jet_directions = input.chars().map(|c| c == '<').collect::<Vec<_>>();

    let mut part1 = 0;
    let mut part2 = 0;

    let mut jet_index = 0;
    let mut shape_index = 0;

    let mut found = false;
    let mut well = Well::default();
    let mut history = Vec::new();
    while shape_index < 1_000_000_000_000 {
        if shape_index == 2022 {
            part1 = well.height();
        }

        if well.drop_rock(
            Shape::rock_shapes()[shape_index % 5],
            &jet_directions,
            &mut jet_index,
        ) {
            if !found && part1 != 0 {
                if let Some((prior_shape, _, prior_height)) =
                    history.iter().find(|(prior_shape, prior_jet_index, _)| {
                        *prior_shape % 5 == shape_index % 5 && *prior_jet_index == jet_index
                    })
                {
                    let cycle_height = well.height() - prior_height;
                    let cycle_rocks = shape_index - *prior_shape;

                    let required_cycles = (1_000_000_000_000 - shape_index) / cycle_rocks;
                    part2 += required_cycles as u64 * cycle_height;
                    shape_index += required_cycles * cycle_rocks;

                    found = true;
                }
            } else {
                history.push((shape_index, jet_index, well.height()));
            }
        }
        shape_index += 1;
    }
    part2 += well.height();

    (part1, part2)
}

#[derive(Default)]
pub struct Well {
    tower: Vec<u8>,
}

impl Well {
    fn drop_rock(&mut self, mut shape: Shape, jets: &[bool], jet_index: &mut usize) -> bool {
        let mut height = self.tower.len() + 3;

        let mut cycled = false;
        loop {
            shape.blow(jets[*jet_index], self.mask(height));

            *jet_index += 1;
            if *jet_index == jets.len() {
                *jet_index = 0;
                cycled = true;
            }

            if height > self.tower.len() {
                height -= 1;
            } else if height == 0 || shape.intersects(self.mask(height - 1)) {
                for row in shape.as_bytes() {
                    if height < self.tower.len() {
                        self.tower[height] |= row;
                    } else {
                        self.tower.push(row);
                    }
                    height += 1;
                }
                return cycled;
            } else {
                height -= 1;
            }
        }
    }

    fn mask(&self, y: usize) -> u32 {
        if y >= self.tower.len() {
            0
        } else {
            self.tower
                .iter()
                .skip(y)
                .take(4)
                .rev()
                .fold(0, |accum, b| (accum << 8) | *b as u32)
        }
    }

    fn height(&self) -> u64 {
        self.tower.len() as u64
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Shape(u32);

impl Shape {
    const fn rock_shapes() -> [Self; 5] {
        [
            Self(0x_00_00_00_1E),
            Self(0x_00_08_1C_08),
            Self(0x_00_04_04_1C),
            Self(0x_10_10_10_10),
            Self(0x_00_00_18_18),
        ]
    }

    fn blow(&mut self, left: bool, mask: u32) {
        let new_position = if left {
            if self.0 & 0x40404040 == 0 {
                self.0 << 1
            } else {
                return;
            }
        } else if self.0 & 0x01010101 == 0 {
            self.0 >> 1
        } else {
            return;
        };
        if new_position & mask == 0 {
            self.0 = new_position;
        }
    }

    fn intersects(&self, mask: u32) -> bool {
        self.0 & mask != 0
    }

    fn as_bytes(&self) -> impl Iterator<Item = u8> {
        self.0.to_le_bytes().into_iter().take_while(|b| *b != 0)
    }
}
