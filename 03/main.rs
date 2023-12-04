struct Number {
    value: u32,
    row: usize,
    start: usize,
    end: usize,
}

fn save_and_reset(
    numbers: &mut Vec<Number>,
    curr_vec: &mut Vec<char>,
    row: usize,
    start: &mut i32,
    end: &mut i32,
) {
    numbers.push(Number {
        value: curr_vec.iter().collect::<String>().parse::<u32>().unwrap(),
        row,
        start: *start as usize,
        end: *end as usize,
    });

    *start = -1;
    *end = -1;
    *curr_vec = Vec::new();
}

fn get_number(grid: &Vec<Vec<char>>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    let mut curr_vec: Vec<char> = Vec::new();
    let mut start: i32 = -1;
    let mut end: i32 = -1;

    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if ch.is_digit(10) {
                if start == -1 {
                    start = j as i32;
                }

                curr_vec.push(*ch);
                end = j as i32;

                if j == row.len() - 1 {
                    save_and_reset(&mut numbers, &mut curr_vec, i, &mut start, &mut end);
                }
            } else if start != -1 {
                save_and_reset(&mut numbers, &mut curr_vec, i, &mut start, &mut end);
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

    let numbers = get_number(&grid);

    let parts = numbers
        .iter()
        .filter(|num| {
            if num.start != 0
                && grid[num.row][num.start - 1] != '.'
                && !grid[num.row][num.start - 1].is_digit(10)
            {
                return true;
            }

            if num.end != w - 1
                && grid[num.row][num.end + 1] != '.'
                && !grid[num.row][num.end + 1].is_digit(10)
            {
                return true;
            }

            if num.row != 0 {
                let start = if num.start == 0 { 0 } else { num.start - 1 };
                let end = if num.end == w - 1 { w - 1 } else { num.end + 1 };

                for i in start..=end {
                    let ch = grid[num.row - 1][i];

                    if ch != '.' && !ch.is_digit(10) {
                        return true;
                    }
                }
            }

            if num.row != h - 1 {
                let start = if num.start == 0 { 0 } else { num.start - 1 };
                let end = if num.end == w - 1 { w - 1 } else { num.end + 1 };

                for i in start..=end {
                    let ch = grid[num.row + 1][i];

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

    let numbers = get_number(&grid);

    let mut total = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch != '*' {
                continue;
            }

            let adjacent_numbers: Vec<&Number> = numbers
                .iter()
                .filter(|num| {
                    num.row >= i - 1 && num.row <= i + 1 && num.start <= j + 1 && num.end >= j - 1
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
