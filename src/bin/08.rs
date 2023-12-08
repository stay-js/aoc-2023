#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn first_part(input: &String) {
    let data: Vec<&str> = input.split("\n\n").collect();

    let directions: Vec<char> = data[0].chars().collect();

    let nodes: Vec<Node> = data[1]
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

    let mut steps = 0;
    let mut current_direction = 0;

    let mut current = nodes.iter().find(|node| node.id == "AAA").unwrap();

    while current.id != "ZZZ" {
        if directions[current_direction] == 'L' {
            current = nodes.iter().find(|node| node.id == current.left).unwrap();
        } else {
            current = nodes.iter().find(|node| node.id == current.right).unwrap();
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

fn main() {
    let (demo_input, input) = aoc::get_input();

    println!("demo-input:");
    first_part(&demo_input);
    // second_part(&demo_input);

    println!("\ninput:");
    first_part(&input);
    // second_part(&input);
}
