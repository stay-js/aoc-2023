struct Point {
    x: usize,
    y: usize,
}

fn calculate_total(input: &String, expansion_ratio: usize) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let h = grid.len();
    let w = grid[0].len();

    let mut empty_rows: Vec<usize> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.push(y);
        }
    }

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
            let x_min = usize::min(a.x, b.x);
            let x_max = usize::max(a.x, b.x);
            let y_min = usize::min(a.y, b.y);
            let y_max = usize::max(a.y, b.y);

            let x_expansion = (x_min..=x_max).filter(|x| empty_cols.contains(&x)).count();
            let y_expansion = (y_min..=y_max).filter(|y| empty_rows.contains(&y)).count();

            total += (x_max - x_min + x_expansion * (expansion_ratio - 1))
                + (y_max - y_min + y_expansion * (expansion_ratio - 1));
        }
    }

    return total / 2;
}

fn first_part(input: &String) {
    println!("First part: {}", calculate_total(input, 2));
}

fn second_part(input: &String) {
    println!("Second part: {}", calculate_total(input, 1000000));
}

fn main() {
    let (demo_input, input) = aoc::get_input();

    println!("demo-input:");
    first_part(&demo_input);

    println!("\ninput:");
    first_part(&input);
    second_part(&input);
}
