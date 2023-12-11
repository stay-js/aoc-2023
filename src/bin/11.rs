struct Point {
    x: usize,
    y: usize,
}

fn first_part(input: &String) {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let w = grid[0].len();

    let mut empty_rows: Vec<usize> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.push(y);
        }
    }

    for (i, y) in empty_rows.iter().enumerate() {
        grid.insert(y + i, (0..w).map(|_| '.').collect());
    }

    let h = grid.len();
    let w = grid[0].len();

    let mut empty_cols: Vec<usize> = Vec::new();

    for x in 0..w {
        let mut y = 0;

        while y < h && grid[y][x] == '.' {
            y += 1;
        }

        if y >= h {
            empty_cols.push(x);
        }
    }

    for row in grid.iter_mut() {
        for (i, x) in empty_cols.iter().enumerate() {
            row.insert(x + i, '.');
        }
    }

    let mut galaxies: Vec<Point> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '#' {
                galaxies.push(Point { x, y });
            }
        }
    }

    let mut total = 0;

    for a in &galaxies {
        for b in &galaxies {
            total += i32::abs(a.x as i32 - b.x as i32) + i32::abs(a.y as i32 - b.y as i32);
        }
    }

    println!("First part: {}", total / 2);
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
