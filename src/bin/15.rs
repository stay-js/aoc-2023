fn first_part(input: &String) {
    let data: Vec<&str> = input.split(",").collect();

    let mut total = 0;

    for item in data {
        let mut current = 0;

        for ch in item.chars() {
            current += ch as u32;
            current *= 17;
            current %= 256;
        }

        total += current;
    }

    println!("First part: {}", total);
}

fn main() {
    let (demo_input, input) = aoc::get_input();

    println!("demo-input:");
    first_part(&demo_input);
    // second_part(&demo_input);

    println!("\ninput:");
    first_part(&input);
    // second_part(&input);
}
