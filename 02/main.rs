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

    let mut total: u16 = 0;

    for line in input.lines() {
        let line_as_vec = line.split(": ").collect::<Vec<&str>>();
        let id: u8 = line_as_vec[0].split(" ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let shows: Vec<&str> = line_as_vec[1].split("; ").collect();
        let mut is_possible = true;

        for show in shows {
            let colors = show.split(", ").collect::<Vec<&str>>();

            for color in colors {
                let color_as_vec = color.split(" ").collect::<Vec<&str>>();
                match color_as_vec[1] {
                    "red" => {
                        if config.red < color_as_vec[0].parse::<u8>().unwrap() {
                            is_possible = false;
                        }
                    }
                    "green" => {
                        if config.green < color_as_vec[0].parse::<u8>().unwrap() {
                            is_possible = false;
                        }
                    }
                    "blue" => {
                        if config.blue < color_as_vec[0].parse::<u8>().unwrap() {
                            is_possible = false;
                        }
                    }
                    _ => {}
                }
            }
        }

        if is_possible {
            total += id as u16;
        }
    }

    println!("First part: {}", total);
}

fn second_part(input: &String) {
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut config = Config {
            red: 0,
            green: 0,
            blue: 0,
        };

        let line_as_vec = line.split(": ").collect::<Vec<&str>>();
        let shows: Vec<&str> = line_as_vec[1].split("; ").collect();

        for show in shows {
            let colors = show.split(", ").collect::<Vec<&str>>();

            for color in colors {
                let color_as_vec = color.split(" ").collect::<Vec<&str>>();

                match color_as_vec[1] {
                    "red" => {
                        let amount = color_as_vec[0].parse::<u8>().unwrap();

                        if config.red < amount {
                            config.red = amount;
                        }
                    }
                    "green" => {
                        let amount = color_as_vec[0].parse::<u8>().unwrap();

                        if config.green < amount {
                            config.green = amount;
                        }
                    }
                    "blue" => {
                        let amount = color_as_vec[0].parse::<u8>().unwrap();

                        if config.blue < amount {
                            config.blue = amount;
                        }
                    }
                    _ => {}
                }
            }
        }

        total += config.red as u32 * config.green as u32 * config.blue as u32;
    }

    println!("Second part: {}", total);
}

fn main() {
    println!("demo-input.txt:");
    let input = std::fs::read_to_string("./02/demo-input.txt").unwrap();
    first_part(&input);
    second_part(&input);

    println!("\ninput.txt:");
    let input = std::fs::read_to_string("./02/input.txt").unwrap();
    first_part(&input);
    second_part(&input);
}
