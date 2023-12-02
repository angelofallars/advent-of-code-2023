use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read("./input/day1.txt")?
        .iter()
        .map(|i| *i as char)
        .collect::<String>();

    println!("Day 1 Part 1 answer: {:#?}", get_calibrations(input));

    Ok(())
}

fn get_calibrations(input: String) -> usize {
    return input
        .trim()
        .split("\n")
        .map(|s| {
            let bytes = s.as_bytes();
            byte_to_usize(bytes[s.find(is_ascii_digit).unwrap()]) * 10
                + byte_to_usize(bytes[s.rfind(is_ascii_digit).unwrap()])
        })
        .fold(0, |acc, e| acc + e);
}

fn byte_to_usize(b: u8) -> usize {
    b as usize - '0' as usize
}

fn is_ascii_digit(c: char) -> bool {
    c.is_ascii_digit()
}
