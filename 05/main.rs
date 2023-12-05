struct Map {
    destination_min: u64,
    source_min: u64,
    length: u64,
}

fn parse_item(item: &str) -> Vec<Map> {
    return item
        .lines()
        .skip(1)
        .map(|line| {
            let data: Vec<u64> = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();

            return Map {
                destination_min: data[0],
                source_min: data[1],
                length: data[2],
            };
        })
        .collect();
}

fn map_to_next(current: Vec<u64>, next: Vec<Map>) -> Vec<u64> {
    return current
        .iter()
        .map(|item| {
            for map in next.iter() {
                if map.source_min <= *item && *item < map.source_min + map.length {
                    return map.destination_min + (*item - map.source_min);
                }
            }

            return *item;
        })
        .collect();
}

fn first_part(input: &String) {
    let data: Vec<&str> = input.split("\n\n").collect();

    let mut current = data[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    for i in 1..data.len() {
        current = map_to_next(current, parse_item(data[i]));
    }

    println!("First part: {}", current.iter().min().unwrap());
}

fn second_part_bf(input: &String) {
    let data: Vec<&str> = input.split("\n\n").collect();

    let mut current = data[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .flat_map(|x| x[0]..x[0] + x[1])
        .collect();

    for i in 1..data.len() {
        current = map_to_next(current, parse_item(data[i]));
    }

    println!(
        "Second part (brute force): {}",
        current.iter().min().unwrap()
    );
}

fn main() {
    println!("demo-input.txt:");
    let input = std::fs::read_to_string("./05/demo-input.txt").unwrap();
    first_part(&input);
    second_part_bf(&input);

    println!("\ninput.txt:");
    let input = std::fs::read_to_string("./05/input.txt").unwrap();
    first_part(&input);
    second_part_bf(&input);
}
