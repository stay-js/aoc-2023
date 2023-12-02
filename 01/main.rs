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

fn second_part(input: &String) {
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

    let mut total = 0;

    for item in input.lines() {
        let mut left_most: Option<u32> = None;
        let mut right_most: Option<u32> = None;

        let mut curr: Vec<char> = Vec::new();

        for c in item.chars() {
            if let Some(number) = c.to_digit(10) {
                if left_most == None {
                    left_most = Some(number);
                }

                right_most = Some(number);
                curr.clear();
                continue;
            }

            curr.push(c);

            let curr_string: String = curr.iter().collect();

            for item in valid_numbers.iter() {
                if curr_string.contains(item) {
                    let value =
                        Some(valid_numbers.iter().position(|r| r == item).unwrap() as u32 + 1);

                    if left_most == None {
                        left_most = value;
                    }

                    right_most = value;
                    curr.clear();
                    break;
                }
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
