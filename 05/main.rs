struct Map {
    destination_min: u64,
    source_min: u64,
    length: u64,
}

struct Data {
    seeds: Vec<u64>,
    seed_to_soil_map: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

fn parse_item(item: &str) -> Vec<Map> {
    return item
        .lines()
        .skip(1)
        .map(|line| {
            let data = line
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            return Map {
                destination_min: data[0],
                source_min: data[1],
                length: data[2],
            };
        })
        .collect();
}

fn get_data(input: &String) -> Data {
    let data = input.split("\n\n").collect::<Vec<&str>>();

    let seeds = data[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    return Data {
        seeds,
        seed_to_soil_map: parse_item(data[1]),
        soil_to_fertilizer: parse_item(data[2]),
        fertilizer_to_water: parse_item(data[3]),
        water_to_light: parse_item(data[4]),
        light_to_temperature: parse_item(data[5]),
        temperature_to_humidity: parse_item(data[6]),
        humidity_to_location: parse_item(data[7]),
    };
}

fn map_to_next(current: Vec<u64>, next: &Vec<Map>) -> Vec<u64> {
    return current
        .iter()
        .map(|item| {
            for map in next {
                if map.source_min <= *item && *item < map.source_min + map.length {
                    return map.destination_min + (*item - map.source_min);
                }
            }

            return *item;
        })
        .collect();
}

fn first_part(input: &String) {
    let data = get_data(input);

    let soils = map_to_next(data.seeds, &data.seed_to_soil_map);
    let fertilizers = map_to_next(soils, &data.soil_to_fertilizer);
    let waters = map_to_next(fertilizers, &data.fertilizer_to_water);
    let lights = map_to_next(waters, &data.water_to_light);
    let temperatures = map_to_next(lights, &data.light_to_temperature);
    let humidities = map_to_next(temperatures, &data.temperature_to_humidity);
    let locations = map_to_next(humidities, &data.humidity_to_location);

    println!("First part: {}", locations.iter().min().unwrap());
}

fn main() {
    println!("demo-input.txt:");
    let input = std::fs::read_to_string("./05/demo-input.txt").unwrap();
    first_part(&input);
    // second_part(&input);

    println!("\ninput.txt:");
    let input = std::fs::read_to_string("./05/input.txt").unwrap();
    first_part(&input);
    // second_part(&input);
}
