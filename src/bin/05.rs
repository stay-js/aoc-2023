// part 2 heavily inspired by https://github.com/Vzaa/aoc2023/blob/main/day05/src/main.rs

type Range = (u64, u64);

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
            let data: Vec<u64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

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
                    return map.destination_min + (item - map.source_min);
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
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for item in data.iter().skip(1) {
        current = map_to_next(current, parse_item(item));
    }

    println!("First part: {}", current.iter().min().unwrap());
}

fn intersect_range(target: Range, source: Range) -> (Option<Range>, Vec<Range>) {
    let (target_min, target_len) = target;
    let target_max = target_min + target_len - 1;

    let (source_min, source_len) = source;
    let source_max = source_min + source_len - 1;

    if source_min >= target_min && source_min <= target_max {
        if source_max >= target_min && source_max <= target_max {
            return (Some(source), Vec::new());
        }

        return (
            Some((source_min, target_len - (source_min - target_min))),
            vec![(target_min + target_len, (source_max) - (target_max))],
        );
    }

    if source_max >= target_min && source_max <= target_max {
        return (
            Some((target_min, source_max - target_min + 1)),
            vec![(source_min, target_min - source_min)],
        );
    }

    if source_min < target_min && source_max > target_max {
        return (
            Some(target),
            vec![
                (source_min, target_min - source_min),
                (target_min + target_len, source_max - target_max),
            ],
        );
    }

    return (None, vec![source]);
}

fn second_part(input: &String) {
    let data: Vec<&str> = input.split("\n\n").collect();

    let mut current: Vec<Range> = data[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|item| (item[0], item[1]))
        .collect();

    for item in data.iter().skip(1) {
        let stage = parse_item(item);
        let mut next = Vec::new();

        for map in stage {
            let mut remain = Vec::new();

            while let Some(item) = current.pop() {
                let (intersection, extra) = intersect_range((map.source_min, map.length), item);

                if let Some((intersecting_min, intersecting_max)) = intersection {
                    next.push((
                        map.destination_min + (intersecting_min - map.source_min),
                        intersecting_max,
                    ));
                }

                remain.extend(extra);
            }

            current.extend(remain);
        }

        current.extend(next);
    }

    println!(
        "Second part: {}",
        current.iter().map(|(min, _)| min).min().unwrap()
    );
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
