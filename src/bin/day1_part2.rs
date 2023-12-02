use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read("./input/day1.txt")?
        .iter()
        .map(|i| *i as char)
        .collect::<String>();

    println!("Day 1 Part 2 answer: {:#?}", get_calibrations(input));

    Ok(())
}

const NUMBERS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_calibrations(input: String) -> usize {
    return input
        .trim()
        .split("\n")
        .map(|s| get_number(s, Direction::Left) * 10 + get_number(s, Direction::Right))
        .fold(0, |acc, e| acc + e);
}

fn get_number(s: &str, direction: Direction) -> usize {
    let collected = if let Some(index) = find(s, &direction) {
        vec![(index, s.as_bytes()[index] as usize - '0' as usize)]
    } else {
        vec![]
    };
    get_number_recursive(s, &direction, 1, collected)
}

fn get_number_recursive(
    s: &str,
    direction: &Direction,
    numbers_index: usize,
    collected: Vec<(usize, usize)>,
) -> usize {
    if numbers_index == 10 {
        return collected
            .iter()
            .reduce(|acc, e| {
                let cmp = match direction {
                    Direction::Left => e.0 < acc.0,
                    Direction::Right => e.0 > acc.0,
                };
                if cmp {
                    e
                } else {
                    acc
                }
            })
            .map(|collected| collected.1)
            .unwrap();
    }

    if let Some(index) = find_letter(s, numbers_index, &direction) {
        let collected = [collected, vec![(index, numbers_index)]].concat();
        return get_number_recursive(s, direction, numbers_index + 1, collected);
    } else {
        return get_number_recursive(s, direction, numbers_index + 1, collected);
    }
}

enum Direction {
    Left,
    Right,
}

fn find(s: &str, direction: &Direction) -> Option<usize> {
    use Direction::*;
    match direction {
        Left => s.find(is_ascii_digit),
        Right => s.rfind(is_ascii_digit),
    }
}

fn find_letter(s: &str, numbers_index: usize, direction: &Direction) -> Option<usize> {
    use Direction::*;
    match direction {
        Left => s.find(NUMBERS[numbers_index - 1]),
        Right => s.rfind(NUMBERS[numbers_index - 1]),
    }
}

fn is_ascii_digit(c: char) -> bool {
    c.is_ascii_digit()
}
