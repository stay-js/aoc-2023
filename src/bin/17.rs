//just implemented this (https://www.youtube.com/watch?v=2pDSooPLLkI) in Rust, couldn't solve the challenge on my own

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn calculate_total_heat_loss(input: &String, min_step: usize, max_step: usize) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut seen = HashSet::new();
    let mut pq = BinaryHeap::new();

    pq.push(Reverse((0, 0, 0, 0, 0, 0)));

    while let Some(Reverse((heat_loss, y, x, delta_y, delta_x, steps))) = pq.pop() {
        if y == height - 1 && x == width - 1 && steps >= min_step {
            return heat_loss;
        }

        if seen.contains(&(y, x, delta_y, delta_x, steps)) {
            continue;
        }

        seen.insert((y, x, delta_y, delta_x, steps));

        if steps < max_step && (delta_y, delta_x) != (0, 0) {
            let new_y = y.wrapping_add(delta_y as usize);
            let new_x = x.wrapping_add(delta_x as usize);

            if new_y < height && new_x < width {
                pq.push(Reverse((
                    heat_loss + grid[new_y][new_x],
                    new_y,
                    new_x,
                    delta_y,
                    delta_x,
                    steps + 1,
                )));
            }
        }

        if steps >= min_step || (delta_y, delta_x) == (0, 0) {
            for (new_delta_x, new_delta_y) in &DIRECTIONS {
                let new_delta_x = *new_delta_x;
                let new_delta_y = *new_delta_y;

                if [(delta_y, delta_x), (-delta_y, -delta_x)].contains(&(new_delta_x, new_delta_y))
                {
                    continue;
                }

                let new_y = y.wrapping_add(new_delta_x as usize);
                let new_x = x.wrapping_add(new_delta_y as usize);

                if new_y < height && new_x < width {
                    pq.push(Reverse((
                        heat_loss + grid[new_y][new_x],
                        new_y,
                        new_x,
                        new_delta_x,
                        new_delta_y,
                        1,
                    )));
                }
            }
        }
    }

    panic!("Invalid input!");
}

fn first_part(input: &String) {
    println!("First part: {}", calculate_total_heat_loss(input, 1, 3));
}

fn second_part(input: &String) {
    println!("Second part: {}", calculate_total_heat_loss(input, 4, 10));
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
