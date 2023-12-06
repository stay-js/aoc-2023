fn first_part(input: &String) {
    let data: Vec<&str> = input.lines().collect();

    let time: Vec<u16> = data[0]
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let distance: Vec<u16> = data[1]
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut total: u32 = 1;

    for (time, distance) in time.iter().zip(distance.iter()) {
        let mut possibilities: u32 = 0;

        for i in 0..*time {
            if (time - i) * i > *distance {
                possibilities += 1;
            }
        }

        total *= possibilities;
    }

    println!("First part: {}", total);
}

fn second_part(input: &String) {
    let data: Vec<&str> = input.lines().collect();

    let time: u64 = data[0]
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();

    let distance: u64 = data[1]
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();

    let mut possibilities: u64 = 0;

    for i in 0..time {
        if (time - i) * i > distance {
            possibilities += 1;
        }
    }

    println!("Second part: {}", possibilities);
}

fn main() {
    println!("demo-input.txt:");
    let input = std::fs::read_to_string("inputs/06/demo-input.txt").unwrap();
    first_part(&input);
    second_part(&input);

    println!("\ninput.txt:");
    let input = std::fs::read_to_string("inputs/06/input.txt").unwrap();
    first_part(&input);
    second_part(&input);
}
