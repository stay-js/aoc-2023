fn tilt_north(grid: &mut Vec<Vec<char>>) {
    let h = grid.len();
    let w = grid[0].len();

    for y in 1..h {
        for x in 0..w {
            if grid[y][x] == 'O' {
                let mut i = y;

                while i > 0 && grid[i - 1][x] == '.' {
                    i -= 1;
                }

                grid[y][x] = '.';
                grid[i][x] = 'O';
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    let h = grid.len();
    let w = grid[0].len();

    for y in (0..h - 1).rev() {
        for x in 0..w {
            if grid[y][x] == 'O' {
                let mut i = y;

                while i < h - 1 && grid[i + 1][x] == '.' {
                    i += 1;
                }

                grid[y][x] = '.';
                grid[i][x] = 'O';
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    let w = grid[0].len();

    for row in grid.iter_mut() {
        for x in 0..w {
            if row[x] == 'O' {
                let mut i = x;

                while i > 0 && row[i - 1] == '.' {
                    i -= 1;
                }

                row[x] = '.';
                row[i] = 'O';
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    let w = grid[0].len();

    for row in grid.iter_mut() {
        for x in (0..w).rev() {
            if row[x] == 'O' {
                let mut i = x;

                while i < w - 1 && row[i + 1] == '.' {
                    i += 1;
                }

                row[x] = '.';
                row[i] = 'O';
            }
        }
    }
}

fn first_part(input: &String) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    tilt_north(&mut grid);

    let mut total = 0;

    for (y, row) in grid.iter().enumerate() {
        for ch in row {
            if ch == &'O' {
                total += grid.len() - y;
            }
        }
    }

    println!("First part: {}", total);
}

fn second_part(input: &String) {
    let original_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut grid: Vec<Vec<char>> = original_grid.clone();

    for _ in 0..1_000_000_000 {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
    }

    let mut total = 0;

    for (y, row) in grid.iter().enumerate() {
        for ch in row {
            if ch == &'O' {
                total += grid.len() - y;
            }
        }
    }

    println!("Second part: {}", total);
}

fn main() {
    let (demo_input, input) = aoc::get_input();

    println!("demo-input:");
    first_part(&demo_input);
    second_part(&demo_input);

    println!("\ninput:");
    first_part(&input);
    second_part(&input);
}
