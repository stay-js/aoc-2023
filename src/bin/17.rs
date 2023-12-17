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

    let h = grid.len();
    let w = grid[0].len();

    let mut seen = HashSet::new();
    let mut pq = BinaryHeap::new();

    pq.push(Reverse((0, 0, 0, 0, 0, 0)));

    while let Some(Reverse((heat_loss, y, x, dy, dx, steps))) = pq.pop() {
        if y == h - 1 && x == w - 1 && steps >= min_step {
            return heat_loss;
        }

        if seen.contains(&(y, x, dy, dx, steps)) {
            continue;
        }

        seen.insert((y, x, dy, dx, steps));

        if steps < max_step && (dy, dx) != (0, 0) {
            let ny = y.wrapping_add(dy as usize);
            let nx = x.wrapping_add(dx as usize);

            if ny < h && nx < w {
                pq.push(Reverse((
                    heat_loss + grid[ny][nx],
                    ny,
                    nx,
                    dy,
                    dx,
                    steps + 1,
                )));
            }
        }

        if steps >= min_step || (dy, dx) == (0, 0) {
            for (ndx, ndy) in &DIRECTIONS {
                if [(&dy, &dx), (&-dy, &-dx)].contains(&(ndx, ndy)) {
                    continue;
                }

                let ny = y.wrapping_add(*ndx as usize);
                let nx = x.wrapping_add(*ndy as usize);

                if ny < h && nx < w {
                    pq.push(Reverse((heat_loss + grid[ny][nx], ny, nx, *ndx, *ndy, 1)));
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
