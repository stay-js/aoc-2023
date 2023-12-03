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

fn first_part(input: &String) {
    let data = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let h = data.len();
    let w = data[0].len();

    let mut numbers: Vec<Number> = Vec::new();

    let mut curr_vec: Vec<char> = Vec::new();
    let mut start: i32 = -1;
    let mut end: i32 = -1;

    for (i, row) in data.iter().enumerate() {
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

    let parts = numbers
        .iter()
        .filter(|x| {
            if x.start != 0
                && data[x.row][x.start - 1] != '.'
                && !data[x.row][x.start - 1].is_digit(10)
            {
                return true;
            }

            if x.end != w - 1
                && data[x.row][x.end + 1] != '.'
                && !data[x.row][x.end + 1].is_digit(10)
            {
                return true;
            }

            if x.row != 0 {
                let start = if x.start == 0 { 0 } else { x.start - 1 };
                let end = if x.end == w - 1 { w - 1 } else { x.end + 1 };

                for i in start..=end {
                    let ch = data[x.row - 1][i];

                    if ch != '.' && !ch.is_digit(10) {
                        return true;
                    }
                }
            }

            if x.row != h - 1 {
                let start = if x.start == 0 { 0 } else { x.start - 1 };
                let end = if x.end == w - 1 { w - 1 } else { x.end + 1 };

                for i in start..=end {
                    let ch = data[x.row + 1][i];

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
        parts.iter().fold(0, |acc, x| acc + x.value)
    );
}

fn main() {
    println!("demo-input.txt:");
    let input = std::fs::read_to_string("./03/demo-input.txt").unwrap();
    first_part(&input);
    // second_part(&input);

    println!("\ninput.txt:");
    let input = std::fs::read_to_string("./03/input.txt").unwrap();
    first_part(&input);
    // second_part(&input);
}
