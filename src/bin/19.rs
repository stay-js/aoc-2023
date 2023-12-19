struct Workflow {
    name: String,
    rules: Vec<String>,
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn first_part(input: &String) {
    let data = input.split("\n\n").collect::<Vec<&str>>();

    let workflows: Vec<Workflow> = data[0]
        .lines()
        .map(|line| {
            let line = line.replace('}', "");
            let data = line.split("{").collect::<Vec<&str>>();

            return Workflow {
                name: data[0].to_string(),
                rules: data[1].split(",").map(|x| x.to_string()).collect(),
            };
        })
        .collect();

    let parts: Vec<Part> = data[1]
        .lines()
        .map(|x| {
            let part = &x[1..&x.len() - 1].split(",").collect::<Vec<&str>>();

            let values: Vec<u32> = part
                .iter()
                .map(|x| x.split("=").nth(1).unwrap().parse().unwrap())
                .collect();

            return Part {
                x: values[0],
                m: values[1],
                a: values[2],
                s: values[3],
            };
        })
        .collect();

    let mut total = 0;

    for part in parts {
        let mut current = workflows.iter().position(|x| &x.name == "in").unwrap();

        let mut is_done = false;

        while !is_done {
            for rule in &workflows[current].rules {
                if rule == "A" {
                    total += part.x + part.m + part.a + part.s;
                    is_done = true;
                    break;
                }

                if rule == "R" {
                    is_done = true;
                    break;
                }

                if !rule.contains(":") {
                    current = workflows.iter().position(|x| &x.name == rule).unwrap();
                    break;
                }

                let parts = rule.split(":").collect::<Vec<&str>>();

                let operator = if rule.contains('<') { '<' } else { '>' };

                let expression = parts[0].split(operator).collect::<Vec<&str>>();

                let variable = match expression[0] {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    var => panic!("unknown variable: {}", var),
                };

                let value = expression[1].parse::<u32>().unwrap();

                if (operator == '<' && variable < value) || (operator == '>' && variable > value) {
                    if parts[1] == "A" {
                        total += part.x + part.m + part.a + part.s;
                        is_done = true;
                        break;
                    }

                    if parts[1] == "R" {
                        is_done = true;
                        break;
                    }

                    current = workflows.iter().position(|x| &x.name == parts[1]).unwrap();
                    break;
                }
            }
        }
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
