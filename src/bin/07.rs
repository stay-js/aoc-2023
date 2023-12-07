use std::collections::HashMap;

struct Hand {
    strength: u8,
    hand: String,
    bid: u32,
}

fn get_hand_strength(input: &str, joker: bool) -> u8 {
    let mut cards = HashMap::new();

    for card in input.chars() {
        let count = cards.entry(card).or_insert(0);
        *count += 1;
    }

    if joker {
        if let Some(&joker_count) = cards.get(&'J') {
            if joker_count != 5 {
                cards.remove(&'J');

                let max_count = cards.values_mut().max().unwrap();
                *max_count += joker_count;
            }
        }
    }

    return match (cards.len(), cards.values().max().unwrap()) {
        (1, _) => 6,
        (2, 4) => 5,
        (2, 3) => 4,
        (3, 3) => 3,
        (3, 2) => 2,
        (4, _) => 1,
        (5, _) => 0,
        _ => panic!("Invalid hand!"),
    };
}

fn get_card_value(card: char, joker: bool) -> u8 {
    if card.is_digit(10) {
        return card.to_digit(10).unwrap() as u8;
    }

    let j_value = if joker { 1 } else { 11 };

    return match card {
        'T' => 10,
        'J' => j_value,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card!"),
    };
}

fn calculate_total_winnings(input: &String, joker: bool) -> u32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let data: Vec<&str> = line.split_whitespace().collect();

            return Hand {
                strength: get_hand_strength(data[0], joker),
                hand: data[0].to_string(),
                bid: data[1].parse().unwrap(),
            };
        })
        .collect();

    hands.sort_by(|a, b| {
        if a.strength != b.strength {
            return a.strength.cmp(&b.strength);
        }

        for (a_card, b_card) in a.hand.chars().zip(b.hand.chars()) {
            if a_card != b_card {
                return get_card_value(a_card, joker).cmp(&get_card_value(b_card, joker));
            }
        }

        return std::cmp::Ordering::Equal;
    });

    let mut total = 0;

    for (i, item) in hands.iter().enumerate() {
        total += (i as u32 + 1) * item.bid;
    }

    return total;
}

fn first_part(input: &String) {
    println!("First part: {}", calculate_total_winnings(input, false));
}

fn second_part(input: &String) {
    println!("Second part: {}", calculate_total_winnings(input, true));
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
