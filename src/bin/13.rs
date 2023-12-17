type Mirror = Vec<Vec<char>>;

fn check_reflection(mirror: &Mirror) -> Option<usize> {
    let mut idx = (0, 0);

    for i in 1..mirror.len() {
        if mirror[i - 1] == mirror[i] {
            idx = (i - 1, i);
        }
    }

    if idx.0 == idx.1 {
        return None;
    }

    let above = &mirror[..=idx.0];
    let below = &mirror[idx.1..];

    if above.iter().rev().zip(below.iter()).any(|(a, b)| a != b) {
        return None;
    }

    return Some(idx.1);
}

fn transpose(mirror: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = mirror.len();
    let cols = mirror[0].len();

    return (0..cols)
        .map(|col| (0..rows).map(|row| mirror[row][col]).collect())
        .collect();
}

fn first_part(input: &String) {
    let mirrors: Vec<Mirror> = input
        .split("\n\n")
        .map(|x| x.split("\n").map(|y| y.chars().collect()).collect())
        .collect();

    let mut total = 0;

    for mirror in mirrors {
        if let Some(rows) = check_reflection(&mirror) {
            total += rows * 100;
        } else if let Some(cols) = check_reflection(&transpose(&mirror)) {
            total += cols;
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
