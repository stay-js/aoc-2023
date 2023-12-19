use std::collections::HashSet;

fn parse_line(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let data: Vec<&str> = line.split(": ").nth(1).unwrap().split(" | ").collect();

    let winning_numbers: HashSet<u32> = data[0]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let numbers: HashSet<u32> = data[1]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    return (winning_numbers, numbers);
}

fn first_part(input: &String) {
    let mut total = 0;

    for line in input.lines() {
        let (winning_numbers, numbers) = parse_line(line);

        let winning = winning_numbers.intersection(&numbers).count();

        if winning > 0 {
            total += u32::pow(2, winning as u32 - 1);
        }
    }

    println!("First part: {}", total);
}

fn second_part(input: &String) {
    let mut copies: Vec<u32> = vec![1; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let (winning_numbers, numbers) = parse_line(line);

        let mut max = i + 1 + winning_numbers.intersection(&numbers).count();

        if max > copies.len() {
            max = copies.len();
        }

        for j in (i + 1)..max {
            copies[j] += copies[i];
        }
    }

    println!("Second part: {}", copies.iter().sum::<u32>());
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
