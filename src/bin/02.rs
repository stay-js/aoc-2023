struct Config {
    red: u8,
    green: u8,
    blue: u8,
}

fn first_part(input: &String) {
    let config = Config {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut total = 0;

    for line in input.lines() {
        let data: Vec<&str> = line.split(": ").collect();

        let id: u16 = data[0].split_whitespace().nth(1).unwrap().parse().unwrap();
        let shows: Vec<&str> = data[1].split("; ").collect();

        let mut is_possible = true;

        for show in shows {
            let colors: Vec<&str> = show.split(", ").collect();

            for color in colors {
                let color_as_vec: Vec<&str> = color.split_whitespace().collect();

                match color_as_vec[1] {
                    "red" => {
                        if config.red < color_as_vec[0].parse().unwrap() {
                            is_possible = false;
                        }
                    }
                    "green" => {
                        if config.green < color_as_vec[0].parse().unwrap() {
                            is_possible = false;
                        }
                    }
                    "blue" => {
                        if config.blue < color_as_vec[0].parse().unwrap() {
                            is_possible = false;
                        }
                    }
                    _ => panic!("Invalid color input!"),
                }
            }
        }

        if is_possible {
            total += id;
        }
    }

    println!("First part: {}", total);
}

fn second_part(input: &String) {
    let mut total = 0;

    for line in input.lines() {
        let mut config = Config {
            red: 0,
            green: 0,
            blue: 0,
        };

        let shows: Vec<&str> = line.split(": ").nth(1).unwrap().split("; ").collect();

        for show in shows {
            let colors: Vec<&str> = show.split(", ").collect();

            for color in colors {
                let color_as_vec: Vec<&str> = color.split_whitespace().collect();

                match color_as_vec[1] {
                    "red" => {
                        let amount = color_as_vec[0].parse().unwrap();

                        if config.red < amount {
                            config.red = amount;
                        }
                    }
                    "green" => {
                        let amount = color_as_vec[0].parse().unwrap();

                        if config.green < amount {
                            config.green = amount;
                        }
                    }
                    "blue" => {
                        let amount = color_as_vec[0].parse().unwrap();

                        if config.blue < amount {
                            config.blue = amount;
                        }
                    }
                    _ => panic!("Invalid color input!"),
                }
            }
        }

        total += config.red as u32 * config.green as u32 * config.blue as u32;
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
