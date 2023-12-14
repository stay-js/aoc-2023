type Grid = Vec<Vec<char>>;

fn tilt_north(grid: &mut Grid) {
    let h = grid.len();
    let w = grid[0].len();

    for y in 1..h {
        for x in 0..w {
            if grid[y][x] != 'O' {
                continue;
            }

            let mut i = y;

            while i > 0 && grid[i - 1][x] == '.' {
                i -= 1;
            }

            grid[y][x] = '.';
            grid[i][x] = 'O';
        }
    }
}

fn tilt_south(grid: &mut Grid) {
    let h = grid.len();
    let w = grid[0].len();

    for y in (0..h - 1).rev() {
        for x in 0..w {
            if grid[y][x] != 'O' {
                continue;
            }

            let mut i = y;

            while i < h - 1 && grid[i + 1][x] == '.' {
                i += 1;
            }

            grid[y][x] = '.';
            grid[i][x] = 'O';
        }
    }
}

fn tilt_west(grid: &mut Grid) {
    let w = grid[0].len();

    for row in grid.iter_mut() {
        for x in 0..w {
            if row[x] != 'O' {
                continue;
            }

            let mut i = x;

            while i > 0 && row[i - 1] == '.' {
                i -= 1;
            }

            row[x] = '.';
            row[i] = 'O';
        }
    }
}

fn tilt_east(grid: &mut Grid) {
    let w = grid[0].len();

    for row in grid.iter_mut() {
        for x in (0..w).rev() {
            if row[x] != 'O' {
                continue;
            }

            let mut i = x;

            while i < w - 1 && row[i + 1] == '.' {
                i += 1;
            }

            row[x] = '.';
            row[i] = 'O';
        }
    }
}

fn calculate_total(grid: &Grid) -> usize {
    return grid.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row.iter().filter(|ch| ch == &&'O').count() * (grid.len() - y)
    });
}

fn first_part(input: &String) {
    let mut grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    tilt_north(&mut grid);

    println!("First part: {}", calculate_total(&grid));
}

fn second_part(input: &String) {
    let mut grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let mut seen_states: Vec<Grid> = Vec::new();

    loop {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        if seen_states.contains(&grid) {
            break;
        }

        seen_states.push(grid.clone());
    }

    let cycle_start = seen_states.iter().position(|g| g == &grid).unwrap();
    let cycle_length = seen_states.len() - cycle_start;
    let remaining_cycles = (1_000_000_000 - cycle_start) % cycle_length;

    let result_grid = &seen_states[cycle_start + remaining_cycles];

    println!("Second part: {}", calculate_total(result_grid));
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
