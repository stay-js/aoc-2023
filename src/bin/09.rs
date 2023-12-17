fn calculate_total(input: &String, future: bool) -> i64 {
    let sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            return line
                .split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect();
        })
        .collect();

    let mut total = 0;

    for sequence in sequences {
        let mut differences = vec![sequence];

        while differences
            .iter()
            .last()
            .unwrap()
            .iter()
            .any(|diff| *diff != 0)
        {
            let diffs = differences
                .last()
                .unwrap()
                .windows(2)
                .map(|items| items[1] - items[0])
                .collect();

            differences.push(diffs);
        }

        if future {
            for i in (0..differences.len() - 1).rev() {
                let last_diff = *differences[i].last().unwrap();
                let current_diff = *differences[i + 1].last().unwrap();

                differences[i].push(last_diff + current_diff);
            }

            total += differences.first().unwrap().last().unwrap();
        } else {
            for i in (0..differences.len() - 1).rev() {
                let last_diff = *differences[i].first().unwrap();
                let current_diff = *differences[i + 1].first().unwrap();

                differences[i].insert(0, last_diff - current_diff);
            }

            total += differences.first().unwrap().first().unwrap();
        }
    }

    return total;
}

fn first_part(input: &String) {
    println!("First part: {}", calculate_total(input, true));
}

fn second_part(input: &String) {
    println!("Second part: {}", calculate_total(input, false));
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
