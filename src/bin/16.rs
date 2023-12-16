use std::collections::HashSet;

type Grid = Vec<Vec<char>>;

#[derive(PartialEq)]
enum Heading {
    North,
    South,
    West,
    East,
}

struct Beam {
    x: usize,
    y: usize,
    heading: Heading,
    is_finished: bool,
}

impl Beam {
    fn new(x: usize, y: usize, heading: Heading) -> Self {
        return Self {
            x,
            y,
            heading,
            is_finished: false,
        };
    }

    fn next(&mut self, height: usize, width: usize) -> Option<(usize, usize)> {
        match self.heading {
            Heading::North => {
                if self.y == 0 {
                    return None;
                }

                self.y -= 1;
            }
            Heading::South => {
                if self.y == height - 1 {
                    return None;
                }

                self.y += 1;
            }
            Heading::West => {
                if self.x == 0 {
                    return None;
                }

                self.x -= 1;
            }
            Heading::East => {
                if self.x == width - 1 {
                    return None;
                }

                self.x += 1;
            }
        };

        return Some((self.x, self.y));
    }
}

fn first_part(input: &String) {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    let mut beams: Vec<Beam> = vec![Beam::new(0, 0, Heading::East)];

    while !beams.iter().all(|beam| beam.is_finished) {
        let mut new_beams: Vec<Beam> = Vec::new();

        for beam in beams.iter_mut() {
            if beam.is_finished {
                continue;
            }

            if let Some((x, y)) = beam.next(height, width) {
                energized.insert((x, y));

                match grid[y][x] {
                    '/' => {
                        beam.heading = match beam.heading {
                            Heading::North => Heading::East,
                            Heading::South => Heading::West,
                            Heading::East => Heading::North,
                            Heading::West => Heading::South,
                        };
                    }
                    '\\' => {
                        beam.heading = match beam.heading {
                            Heading::North => Heading::West,
                            Heading::South => Heading::East,
                            Heading::East => Heading::South,
                            Heading::West => Heading::North,
                        };
                    }
                    '-' => {
                        if beam.heading == Heading::North || beam.heading == Heading::South {
                            beam.heading = Heading::East;
                            new_beams.push(Beam::new(x, y, Heading::West));
                        };
                    }
                    '|' => {
                        if beam.heading == Heading::East || beam.heading == Heading::West {
                            beam.heading = Heading::North;
                            new_beams.push(Beam::new(x, y, Heading::South));
                        };
                    }
                    '.' => {}
                    _ => panic!("Unknown character: {}!", grid[y][x]),
                }

                continue;
            }

            beam.is_finished = true;
        }

        beams.extend(new_beams);
    }

    println!("First part: {}", energized.len());
}

fn main() {
    let (demo_input, input) = aoc::get_input();

    println!("demo-input:");
    first_part(&demo_input);
    // second_part(&demo_input);

    println!("\ninput:");
    first_part(&input);
    // second_part(&input);
}
