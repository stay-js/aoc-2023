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

fn find_left_most(input: &Vec<char>, only_digit: bool) -> u32 {
    for i in 0..input.len() {
        if let Some(number) = input[i].to_digit(10) {
            return number;
        }

        if !only_digit {
            if let Some(number) = match_string_to_number(input[..=i].iter().collect()) {
                return number;
            }
        }
    }

    panic!("No left most number found!");
}

fn find_right_most(input: &Vec<char>, only_digit: bool) -> u32 {
    for i in (0..input.len()).rev() {
        if let Some(number) = input[i].to_digit(10) {
            return number;
        }

        if !only_digit {
            if let Some(number) = match_string_to_number(input[i..].iter().collect()) {
                return number;
            }
        }
    }

    panic!("No right most number found!");
}

fn calculate_total(input: &String, only_digit: bool) -> u32 {
    let mut total = 0;

    for item in input.lines() {
        let chars: Vec<char> = item.chars().collect();

        let left_most = find_left_most(&chars, only_digit);
        let right_most = find_right_most(&chars, only_digit);

        total += left_most * 10 + right_most;
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

    println!("\ndemo-input-2:");
    second_part(&aoc::get_input_2());

    println!("\ninput:");
    first_part(&input);
    second_part(&input);
}
