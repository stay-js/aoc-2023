// TODO: check if loop is valid, check if next step is valid
// only works because of luck...

use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn get_north(&self) -> Option<Point> {
        if self.y > 0 {
            return Some(Point {
                x: self.x,
                y: self.y - 1,
            });
        }

        return None;
    }

    fn get_south(&self, height: usize) -> Option<Point> {
        if self.y < height - 1 {
            return Some(Point {
                x: self.x,
                y: self.y + 1,
            });
        }

        return None;
    }

    fn get_west(&self) -> Option<Point> {
        if self.x > 0 {
            return Some(Point {
                x: self.x - 1,
                y: self.y,
            });
        }

        return None;
    }

    fn get_east(&self, width: usize) -> Option<Point> {
        if self.x < width - 1 {
            return Some(Point {
                x: self.x + 1,
                y: self.y,
            });
        }

        return None;
    }

    fn move_north(&mut self) {
        self.y -= 1;
    }

    fn move_south(&mut self) {
        self.y += 1;
    }

    fn move_west(&mut self) {
        self.x -= 1;
    }

    fn move_east(&mut self) {
        self.x += 1;
    }
}

fn first_part(input: &String) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut start = Point { x: 0, y: 0 };

    for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if ch == &'S' {
                start = Point { x, y };
            }
        }
    }

    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(start);

    let mut steps = 0;
    let mut current = start;

    loop {
        if steps == 0 {
            if current.y > 0 && ['|', '7', 'F'].contains(&grid[current.y - 1][current.x]) {
                current.move_north();
            } else if current.x > 0 && ['-', 'L', 'F'].contains(&grid[current.y][current.x - 1]) {
                current.move_west();
            } else if current.y < height - 1
                && ['|', 'L', 'J'].contains(&grid[current.y + 1][current.x])
            {
                current.move_south();
            } else if current.x < width - 1
                && ['-', 'J', '7'].contains(&grid[current.y][current.x + 1])
            {
                current.move_east();
            }

            steps += 1;
            visited.insert(current);
            continue;
        }

        steps += 1;

        match grid[current.y][current.x] {
            '|' => {
                let north = current.get_north();
                let south = current.get_south(height);

                if steps > 2 && north.is_some() && start == north.unwrap()
                    || south.is_some() && start == south.unwrap()
                {
                    break;
                }

                if north.is_some() && !visited.contains(&north.unwrap()) {
                    current.move_north();
                } else if south.is_some() && !visited.contains(&south.unwrap()) {
                    current.move_south();
                }
            }
            '-' => {
                let east = current.get_east(width);
                let west = current.get_west();

                if steps > 2 && east.is_some() && start == east.unwrap()
                    || west.is_some() && start == west.unwrap()
                {
                    break;
                }

                if east.is_some() && !visited.contains(&east.unwrap()) {
                    current.move_east();
                } else if west.is_some() && !visited.contains(&west.unwrap()) {
                    current.move_west();
                }
            }
            'L' => {
                let north = current.get_north();
                let east = current.get_east(width);

                if steps > 2 && north.is_some() && start == north.unwrap()
                    || east.is_some() && start == east.unwrap()
                {
                    break;
                }

                if north.is_some() && !visited.contains(&north.unwrap()) {
                    current.move_north();
                } else if east.is_some() && !visited.contains(&east.unwrap()) {
                    current.move_east();
                }
            }
            'J' => {
                let north = current.get_north();
                let west = current.get_west();

                if steps > 2 && north.is_some() && start == north.unwrap()
                    || west.is_some() && start == west.unwrap()
                {
                    break;
                }

                if north.is_some() && !visited.contains(&north.unwrap()) {
                    current.move_north();
                } else if west.is_some() && !visited.contains(&west.unwrap()) {
                    current.move_west();
                }
            }
            '7' => {
                let south = current.get_south(height);
                let west = current.get_west();

                if steps > 2 && south.is_some() && start == south.unwrap()
                    || west.is_some() && start == west.unwrap()
                {
                    break;
                }

                if south.is_some() && !visited.contains(&south.unwrap()) {
                    current.move_south();
                } else if west.is_some() && !visited.contains(&west.unwrap()) {
                    current.move_west();
                }
            }
            'F' => {
                let south = current.get_south(height);
                let east = current.get_east(width);

                if steps > 2 && south.is_some() && start == south.unwrap()
                    || east.is_some() && start == east.unwrap()
                {
                    break;
                }

                if south.is_some() && !visited.contains(&south.unwrap()) {
                    current.move_south();
                } else if east.is_some() && !visited.contains(&east.unwrap()) {
                    current.move_east();
                }
            }
            _ => panic!("Unknown char: {}", grid[current.y][current.x]),
        }

        visited.insert(current);
    }

    println!("First part: {:?}", steps / 2);
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
