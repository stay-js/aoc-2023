#[derive(Debug)]
struct Data {
    seeds: Vec<u32>,
    seed_to_soil_map: Vec<(Vec<u32>, Vec<u32>)>,
    soil_to_fertilizer: Vec<(Vec<u32>, Vec<u32>)>,
    fertilizer_to_water: Vec<(Vec<u32>, Vec<u32>)>,
    water_to_light: Vec<(Vec<u32>, Vec<u32>)>,
    light_to_temperature: Vec<(Vec<u32>, Vec<u32>)>,
    temperature_to_humidity: Vec<(Vec<u32>, Vec<u32>)>,
    humidity_to_location: Vec<(Vec<u32>, Vec<u32>)>,
}

fn parse_item(item: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    println!("{}", item);
    return item
        .lines()
        .skip(1)
        .map(|line| {
            let data = line
                .split(" ")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let destination = (data[0]..=data[0] + data[2]).collect();
            let source = (data[1]..=data[1] + data[2]).collect();

            return (destination, source);
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
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

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

fn map_to_next(current: Vec<u32>, next: &Vec<(Vec<u32>, Vec<u32>)>) -> Vec<u32> {
    return current
        .iter()
        .map(|item| {
            for (destination, source) in next {
                if source.contains(item) {
                    let idx = source.iter().position(|x| x == item).unwrap();
                    return destination[idx];
                }
            }

            return *item;
        })
        .collect::<Vec<u32>>();
}

fn first_part(input: &String) {
    let data = get_data(input);
    println!("{:?}", data);

    let soils = map_to_next(data.seeds, &data.seed_to_soil_map);
    let fertilizers = map_to_next(soils, &data.soil_to_fertilizer);
    let waters = map_to_next(fertilizers, &data.fertilizer_to_water);
    let lights = map_to_next(waters, &data.water_to_light);
    let temperatures = map_to_next(lights, &data.light_to_temperature);
    let humidities = map_to_next(temperatures, &data.temperature_to_humidity);
    let locations = map_to_next(humidities, &data.humidity_to_location);

    println!("{}", locations.iter().min().unwrap());
}

fn main() {
    println!("demo-input.txt:");
    let input = std::fs::read_to_string("./05/demo-input.txt").unwrap();
    first_part(&input);
    // second_part(&input);

    // println!("\ninput.txt:");
    // let input = std::fs::read_to_string("./05/input.txt").unwrap();
    // first_part(&input);
    // second_part(&input);
}
