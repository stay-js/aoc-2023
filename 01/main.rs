fn first_part(input: &String) {
    let mut total = 0;

    for item in input.lines() {
        let mut left_most: Option<u32> = None;
        let mut right_most: Option<u32> = None;

        for c in item.chars() {
            if let Some(number) = c.to_digit(10) {
                if left_most == None {
                    left_most = Some(number);
                }

                right_most = Some(number);
            }
        }

        total += left_most.unwrap() * 10 + right_most.unwrap();
    }

    println!("First Part: {}", total);
}

fn match_string_to_number(input: String) -> Option<u32> {
    let valid_numbers = [
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
    ];

    for i in 0..valid_numbers.len() {
        if input.contains(&valid_numbers[i]) {
            return Some(i as u32 + 1);
        }
    }

    return None;
}

fn second_part(input: &String) {
    let mut total = 0;

    for item in input.lines() {
        let mut left_most: Option<u32> = None;
        let mut right_most: Option<u32> = None;

        let item_as_vec: Vec<char> = item.chars().collect();

        for i in 0..item.len() {
            if let Some(number) = item_as_vec[i].to_digit(10) {
                left_most = Some(number);
                break;
            }

            if let Some(number) = match_string_to_number(item_as_vec[..=i].iter().collect()) {
                left_most = Some(number);
                break;
            }
        }

        for i in (0..item.len()).rev() {
            if let Some(number) = item_as_vec[i].to_digit(10) {
                right_most = Some(number);
                break;
            }

            if let Some(number) = match_string_to_number(item_as_vec[i..].iter().collect()) {
                right_most = Some(number);
                break;
            }
        }

        total += left_most.unwrap() * 10 + right_most.unwrap();
    }

    println!("Second Part: {}", total);
}

fn main() {
    println!("demo-input.txt:");
    first_part(&std::fs::read_to_string("./01/demo-input.txt").unwrap());

    println!("\ndemo-input2.txt:");
    second_part(&std::fs::read_to_string("./01/demo-input-2.txt").unwrap());

    println!("\ninput.txt:");
    let file_as_string = std::fs::read_to_string("./01/input.txt").unwrap();
    first_part(&file_as_string);
    second_part(&file_as_string);
}
