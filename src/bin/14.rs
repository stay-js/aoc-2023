fn first_part(input: &String) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

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

    let mut total = 0;

    for (y, row) in grid.iter().enumerate() {
        for ch in row {
            if ch == &'O' {
                total += h - y;
            }
        }
    }

    println!("First part: {}", total);
}

fn main() {
    let (demo_input, input) = aoc::get_input();

    println!("demo-input:");
    first_part(&demo_input);

    println!("\ninput:");
    first_part(&input);
    // // second_part(&input);
}
