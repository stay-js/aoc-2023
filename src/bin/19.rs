struct Expression {
    variable: char,
    operator: char,
    value: u32,
}

enum Rule {
    Jump(String),
    JumpIf(Expression, String),
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
    value: u32,
}

fn first_part(input: &String) {
    let data: Vec<&str> = input.split("\n\n").collect();

    let workflows: Vec<Workflow> = data[0]
        .lines()
        .map(|line| {
            let line = line.replace('}', "");
            let data: Vec<&str> = line.split("{").collect();

            let rules: Vec<Rule> = data[1]
                .split(",")
                .map(|rule| {
                    if !rule.contains(":") {
                        return Rule::Jump(rule.to_string());
                    }

                    let parts: Vec<&str> = rule.split(":").collect();
                    let operator = if rule.contains('<') { '<' } else { '>' };
                    let expression: Vec<&str> = parts[0].split(operator).collect();

                    return Rule::JumpIf(
                        Expression {
                            variable: expression[0].chars().nth(0).unwrap(),
                            operator,
                            value: expression[1].parse().unwrap(),
                        },
                        parts[1].to_string(),
                    );
                })
                .collect();

            return Workflow {
                name: data[0].to_string(),
                rules,
            };
        })
        .collect();

    let parts: Vec<Part> = data[1]
        .lines()
        .map(|x| {
            let part: &Vec<&str> = &x[1..&x.len() - 1].split(",").collect();

            let values: Vec<u32> = part
                .iter()
                .map(|x| x.split("=").nth(1).unwrap().parse().unwrap())
                .collect();

            return Part {
                x: values[0],
                m: values[1],
                a: values[2],
                s: values[3],
                value: values.iter().sum(),
            };
        })
        .collect();

    let mut total = 0;

    for part in parts {
        let mut current = workflows.iter().position(|x| &x.name == "in").unwrap();
        let mut is_done = false;

        while !is_done {
            for rule in &workflows[current].rules {
                if let Rule::Jump(to) = rule {
                    if to == "A" {
                        total += part.value;
                        is_done = true;
                        break;
                    }

                    if to == "R" {
                        is_done = true;
                        break;
                    }

                    current = workflows.iter().position(|x| &x.name == to).unwrap();
                    break;
                }

                if let Rule::JumpIf(expression, to) = rule {
                    let variable = match expression.variable {
                        'x' => part.x,
                        'm' => part.m,
                        'a' => part.a,
                        's' => part.s,
                        var => panic!("unknown variable: {}", var),
                    };

                    if (expression.operator == '<' && variable < expression.value)
                        || (expression.operator == '>' && variable > expression.value)
                    {
                        if to == "A" {
                            total += part.value;
                            is_done = true;
                            break;
                        }

                        if to == "R" {
                            is_done = true;
                            break;
                        }

                        current = workflows.iter().position(|x| &x.name == to).unwrap();
                        break;
                    }
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
