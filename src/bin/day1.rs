use std::{backtrace::Backtrace, fs, iter::once};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input: Vec<char> = fs::read("./input/day1.txt")?
        .iter()
        .map(|i| *i as char)
        .collect();

    println!(
        "Day 1 Part 1 Answer: {:#?}",
        eval(parse(lex(input.clone(), false)))
    );
    println!("Day 1 Part 2 Answer: {:#?}", eval(parse(lex(input, true))));

    Ok(())
}

fn lex(input: Vec<char>, is_part_two: bool) -> Vec<Token> {
    input
        .clone()
        .into_iter()
        .enumerate()
        .fold((0, Vec::new()), |acc, c| {
            let skip_count = acc.0;
            if skip_count > 0 {
                return (skip_count - 1, acc.1);
            }

            let pos = c.0;
            let c = c.1;
            let input = &input;

            let token = match c {
                'a'..='z' => {
                    if !is_part_two {
                        return (0, acc.1);
                    }

                    use Token::*;
                    let opt = [
                        ("one", One),
                        ("two", Two),
                        ("three", Three),
                        ("four", Four),
                        ("five", Five),
                        ("six", Six),
                        ("seven", Seven),
                        ("eight", Eight),
                        ("nine", Nine),
                    ]
                    .into_iter()
                    .find_map(|digit_str| {
                        let (pos, exists) = lex_ident(input, pos, digit_str.0);
                        if exists {
                            Some((pos, digit_str.1))
                        } else {
                            None
                        }
                    });

                    if let Some((end_pos, token)) = opt {
                        return (end_pos - pos - 2, append(acc.1, token));
                    } else {
                        return acc;
                    }
                }
                '0'..='9' => token_from_char(c).unwrap(),
                '\n' => Token::Newline,
                _ => return (0, acc.1),
            };

            (0, append(acc.1, token))
        })
        .1
}

fn lex_ident(input: &Vec<char>, pos: usize, ident: &str) -> (usize, bool) {
    lex_ident_recursive(input, pos, ident.as_bytes(), 0)
}

fn lex_ident_recursive(
    input: &Vec<char>,
    start_pos: usize,
    bytes: &[u8],
    index: usize,
) -> (usize, bool) {
    let pos = start_pos + index;
    if is_end(bytes, index) || is_end(input, pos) {
        (pos, true)
    } else if input[pos] != bytes[index] as char {
        (pos, false)
    } else {
        lex_ident_recursive(input, start_pos, bytes, advance(index))
    }
}

#[derive(Debug, Clone, Copy)]
enum Token {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Newline,
}

fn token_from_char(c: char) -> Option<Token> {
    use Token::*;
    match c {
        '1' => Some(One),
        '2' => Some(Two),
        '3' => Some(Three),
        '4' => Some(Four),
        '5' => Some(Five),
        '6' => Some(Six),
        '7' => Some(Seven),
        '8' => Some(Eight),
        '9' => Some(Nine),
        _ => None,
    }
}

fn token_to_usize(token: &Token) -> Option<usize> {
    use Token::*;
    match token {
        One => Some(1),
        Two => Some(2),
        Three => Some(3),
        Four => Some(4),
        Five => Some(5),
        Six => Some(6),
        Seven => Some(7),
        Eight => Some(8),
        Nine => Some(9),
        _ => None,
    }
}

fn parse(tokens: Vec<Token>) -> Vec<Pair> {
    parse_recursive(&tokens, 0, None, Vec::new())
}

fn parse_recursive(
    input: &Vec<Token>,
    pos: usize,
    pair: Option<Pair>,
    pairs: Vec<Pair>,
) -> Vec<Pair> {
    if is_end(input, pos) {
        return pairs;
    }

    let token = input[pos];
    use Token::*;
    match token {
        One | Two | Three | Four | Five | Six | Seven | Eight | Nine => {
            let digit = token_to_usize(&token).unwrap();
            if let Some(pair) = pair {
                // Always overwrite the latter digit on the pair
                parse_recursive(input, advance(pos), Some((pair.0, digit)), pairs)
            } else {
                parse_recursive(input, advance(pos), Some((digit, digit)), pairs)
            }
        }
        Newline => {
            if let Some(pair) = pair {
                parse_recursive(input, advance(pos), None, append(pairs, pair))
            } else {
                panic!("Newline with no line stack")
            }
        }
    }
}

type Pair = (usize, usize);

fn eval(pairs: Vec<Pair>) -> usize {
    eval_recursive(&pairs, 0, 0)
}

fn eval_recursive(pairs: &Vec<Pair>, pos: usize, sum: usize) -> usize {
    if is_end(pairs, pos) {
        return sum;
    } else {
        let pair = pairs[pos];
        eval_recursive(pairs, advance(pos), sum + ((pair.0 * 10) + pair.1))
    }
}

#[inline]
fn is_end<T: IntoIterator<IntoIter = U>, U: ExactSizeIterator>(input: T, pos: usize) -> bool {
    input.into_iter().len() <= pos
}

/// append works exactly like Go's `append` function.
#[inline]
fn append<T: IntoIterator<Item = U> + FromIterator<U>, U>(i: T, elem: U) -> T {
    i.into_iter().chain(once(elem)).collect()
}

#[inline]
fn advance(i: usize) -> usize {
    i + 1
}
