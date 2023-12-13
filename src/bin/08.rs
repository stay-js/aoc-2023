struct Node {
    id: String,
    left: String,
    right: String,
}

fn get_data(input: &String) -> (Vec<char>, Vec<Node>) {
    let data: Vec<&str> = input.split("\n\n").collect();

    let directions: Vec<char> = data[0].chars().collect();

    let nodes = data[1]
        .lines()
        .map(|line| {
            let data = line.split(" = ").collect::<Vec<&str>>();
            let connections = data[1].split(", ").collect::<Vec<&str>>();

            return Node {
                id: data[0].to_string(),
                left: connections[0].replace('(', "").to_string(),
                right: connections[1].replace(')', "").to_string(),
            };
        })
        .collect();

    return (directions, nodes);
}

fn first_part(input: &String) {
    let (directions, nodes) = get_data(input);

    let mut steps = 0;
    let mut current_direction = 0;

    let mut current = nodes.iter().find(|node| node.id == "AAA").unwrap();

    while current.id != "ZZZ" {
        match directions[current_direction] {
            'L' => current = nodes.iter().find(|node| node.id == current.left).unwrap(),
            'R' => current = nodes.iter().find(|node| node.id == current.right).unwrap(),
            _ => panic!("Invalid direction!"),
        }

        if current_direction == directions.len() - 1 {
            current_direction = 0;
        } else {
            current_direction += 1;
        }

        steps += 1;
    }

    println!("First part: {}", steps);
}

fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a, b);

    while b != 0 {
        (a, b) = (b, a % b);
    }

    return a;
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }

    return a * b / gcd(a, b);
}

fn second_part(input: &String) {
    let (directions, nodes) = get_data(input);

    let starting_nodes: Vec<&Node> = nodes.iter().filter(|node| node.id.ends_with("A")).collect();

    let mut steps_per_node: Vec<u64> = Vec::new();

    for node in starting_nodes {
        let mut steps = 0;
        let mut current_direction = 0;
        let mut current = node;

        while !current.id.ends_with("Z") {
            match directions[current_direction] {
                'L' => current = nodes.iter().find(|node| node.id == current.left).unwrap(),
                'R' => current = nodes.iter().find(|node| node.id == current.right).unwrap(),
                _ => panic!("Invalid direction!"),
            }

            if current_direction == directions.len() - 1 {
                current_direction = 0;
            } else {
                current_direction += 1;
            }

            steps += 1;
        }

        steps_per_node.push(steps);
    }

    println!(
        "Second part: {}",
        steps_per_node.iter().fold(1, |acc, steps| lcm(*steps, acc))
    );
}

fn main() {
    let (demo_input, input) = aoc::get_input();

    println!("demo-input:");
    first_part(&demo_input);

    println!("\ndemo-input-2:");
    first_part(&aoc::get_input_by_name("demo-input-2.txt"));

    println!("\ndemo-input-3:");
    second_part(&aoc::get_input_by_name("demo-input-3.txt"));

    println!("\ninput:");
    first_part(&input);
    second_part(&input);
}
