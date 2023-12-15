struct Lens {
    label: String,
    focusing_power: u32,
}

fn calculate_hash(input: &str) -> u32 {
    let mut current = 0;

    for ch in input.chars() {
        current += ch as u32;
        current *= 17;
        current %= 256;
    }

    return current;
}

fn first_part(input: &String) {
    let total = input
        .split(",")
        .fold(0, |acc, item| acc + calculate_hash(item));

    println!("First part: {}", total);
}

fn second_part(input: &String) {
    let data: Vec<&str> = input.split(",").collect();

    let mut boxes = (0..256).map(|_| Vec::new()).collect::<Vec<Vec<Lens>>>();

    for item in data {
        let label: String = item.chars().filter(|ch| ch.is_alphabetic()).collect();
        let current_box = &mut boxes[calculate_hash(label.as_str()) as usize];

        if item.contains("=") {
            let focusing_power = item.split("=").nth(1).unwrap().parse().unwrap();

            if let Some(lens_id) = current_box.iter().position(|lens| lens.label == label) {
                current_box[lens_id].focusing_power = focusing_power;
            } else {
                current_box.push(Lens {
                    label,
                    focusing_power,
                });
            }
        } else if let Some(lens_id) = current_box.iter().position(|lens| lens.label == label) {
            current_box.remove(lens_id);
        }
    }

    let mut total = 0;

    for (i, item) in boxes.iter().enumerate() {
        for (j, lens) in item.iter().enumerate() {
            total += (i as u32 + 1) * (j as u32 + 1) * lens.focusing_power;
        }
    }

    println!("Second part: {}", total);
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
