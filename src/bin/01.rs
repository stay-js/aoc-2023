const VALID_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn match_string_to_number(input: String) -> Option<u32> {
    for (i, number) in VALID_NUMBERS.iter().enumerate() {
        if input.contains(number) {
            return Some(i as u32 + 1);
        }
    }

    return None;
}

fn find_most(input: &Vec<char>, only_digit: bool, left: bool) -> u32 {
    let range: Vec<usize> = match left {
        true => (0..input.len()).collect(),
        false => (0..input.len()).rev().collect(),
    };

    for i in range {
        if let Some(number) = input[i].to_digit(10) {
            return number;
        }

        if !only_digit {
            let sub_string: String = match left {
                true => input[..=i].iter().collect(),
                false => input[i..].iter().collect(),
            };

            if let Some(number) = match_string_to_number(sub_string) {
                return number;
            }
        }
    }

    panic!(
        "No {} most number found!",
        if left { "left" } else { "right" }
    );
}

fn calculate_total(input: &String, only_digit: bool) -> u32 {
    return input.lines().fold(0, |acc, item| {
        let chars: Vec<char> = item.chars().collect();

        let left_most = find_most(&chars, only_digit, true);
        let right_most = find_most(&chars, only_digit, false);

        return acc + left_most * 10 + right_most;
    });
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

    println!("\ndemo-input-2:");
    second_part(&aoc::get_input_by_name("demo-input-2.txt"));

    println!("\ninput:");
    first_part(&input);
    second_part(&input);
}
