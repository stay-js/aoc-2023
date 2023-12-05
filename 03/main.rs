struct Number {
    value: u32,
    y: usize,
    start: usize,
    end: usize,
}

impl Number {
    fn new(value: u32, y: usize, start: usize, end: usize) -> Self {
        Self {
            value,
            y,
            start,
            end,
        }
    }
}

fn get_numbers(grid: &Vec<Vec<char>>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    let mut current = String::new();
    let mut start: usize = 0;
    let mut end: usize = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if ch.is_digit(10) {
                if current.is_empty() {
                    start = x;
                }

                current.push(*ch);
                end = x;

                if x == row.len() - 1 {
                    numbers.push(Number::new(current.parse().unwrap(), y, start, end));
                    current.clear();
                }
            } else if !current.is_empty() {
                numbers.push(Number::new(current.parse().unwrap(), y, start, end));
                current.clear();
            }
        }
    }

    return numbers;
}

fn first_part(input: &String) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let h = grid.len();
    let w = grid[0].len();

    let numbers = get_numbers(&grid);

    let parts = numbers
        .iter()
        .filter(|num| {
            if num.start != 0
                && grid[num.y][num.start - 1] != '.'
                && !grid[num.y][num.start - 1].is_digit(10)
            {
                return true;
            }

            if num.end != w - 1
                && grid[num.y][num.end + 1] != '.'
                && !grid[num.y][num.end + 1].is_digit(10)
            {
                return true;
            }

            if num.y != 0 {
                let start = if num.start == 0 { 0 } else { num.start - 1 };
                let end = if num.end == w - 1 { w - 1 } else { num.end + 1 };

                for i in start..=end {
                    let ch = grid[num.y - 1][i];

                    if ch != '.' && !ch.is_digit(10) {
                        return true;
                    }
                }
            }

            if num.y != h - 1 {
                let start = if num.start == 0 { 0 } else { num.start - 1 };
                let end = if num.end == w - 1 { w - 1 } else { num.end + 1 };

                for i in start..=end {
                    let ch = grid[num.y + 1][i];

                    if ch != '.' && !ch.is_digit(10) {
                        return true;
                    }
                }
            }

            return false;
        })
        .collect::<Vec<&Number>>();

    println!(
        "First part: {}",
        parts.iter().fold(0, |acc, num| acc + num.value)
    );
}

fn second_part(input: &String) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let numbers = get_numbers(&grid);

    let mut total = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch != '*' {
                continue;
            }

            let adjacent_numbers: Vec<&Number> = numbers
                .iter()
                .filter(|num| {
                    num.y >= y - 1 && num.y <= y + 1 && num.start <= x + 1 && num.end >= x - 1
                })
                .collect();

            if adjacent_numbers.len() == 2 {
                total += adjacent_numbers[0].value * adjacent_numbers[1].value;
            }
        }
    }

    println!("Second part: {}", total);
}

fn main() {
    println!("demo-input.txt:");
    let input = std::fs::read_to_string("./03/demo-input.txt").unwrap();
    first_part(&input);
    second_part(&input);

    println!("\ninput.txt:");
    let input = std::fs::read_to_string("./03/input.txt").unwrap();
    first_part(&input);
    second_part(&input);
}
