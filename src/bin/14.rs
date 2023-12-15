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

const ITERATIONS: usize = 1_000_000_000;

fn second_part(input: &String) {
    let mut grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let mut seen_states: Vec<Grid> = Vec::new();

    for i in 0..ITERATIONS {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        if let Some(seen_at) = seen_states.iter().rposition(|g| g == &grid) {
            if (ITERATIONS - i - 1) % (i - seen_at) == 0 {
                break;
            }
        }

        seen_states.push(grid.clone());
    }

    println!("Second part: {}", calculate_total(&grid));
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
