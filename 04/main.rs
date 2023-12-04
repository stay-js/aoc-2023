fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let line_vec: Vec<&str> = line.split(": ").collect();

    let numbers_vec: Vec<&str> = line_vec[1].split(" | ").collect();

    let winning_numbers = numbers_vec[0]
        .split(" ")
        .filter(|&x| x != "")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let numbers = numbers_vec[1]
        .split(" ")
        .filter(|&x| x != "")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    return (winning_numbers, numbers);
}

fn first_part(input: &String) {
    let mut total = 0;

    for line in input.lines() {
        let (winning_numbers, numbers) = parse_line(line);

        let mut game_total = 0;

        for number in numbers {
            if winning_numbers.contains(&number) {
                if game_total == 0 {
                    game_total = 1;
                } else {
                    game_total *= 2;
                }
            }
        }

        total += game_total;
    }

    println!("First part: {}", total);
}

fn second_part(input: &String) {
    let mut copies: Vec<usize> = Vec::new();
    copies.resize(input.lines().count(), 1);

    for (i, line) in input.lines().enumerate() {
        let (winning_numbers, numbers) = parse_line(line);

        let mut game_total = 0;

        for number in numbers {
            if winning_numbers.contains(&number) {
                game_total += 1;
            }
        }

        let mut max = i + game_total + 1;

        if max > copies.len() {
            max = copies.len();
        }

        for j in (i + 1)..max {
            copies[j] += copies[i];
        }
    }

    println!("Second part: {}", copies.iter().sum::<usize>());
}

fn main() {
    println!("demo-input.txt:");
    let input = std::fs::read_to_string("./04/demo-input.txt").unwrap();
    first_part(&input);
    second_part(&input);

    println!("\ninput.txt:");
    let input = std::fs::read_to_string("./04/input.txt").unwrap();
    first_part(&input);
    second_part(&input);
}
