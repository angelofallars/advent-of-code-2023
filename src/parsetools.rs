use std::fmt;
use std::mem;

#[path = "./functools.rs"]
mod functools;
use functools::*;

pub type Index = usize;

/// Works like a map
pub fn transform<T, U>(input: Vec<T>, map: fn(&Vec<T>, Index) -> (Index, U)) -> Vec<U> {
    return aux(&input, map, 0, Vec::new());

    fn aux<T, U>(
        input: &Vec<T>,
        map: fn(&Vec<T>, usize) -> (usize, U),
        pos: Index,
        outputs: Vec<U>,
    ) -> Vec<U> {
        if is_end(input, pos) {
            return outputs;
        }

        let (pos, output) = map(input, pos);

        aux(input, map, pos, append(outputs, output))
    }
}

pub mod lextools {
    use super::*;

    pub fn skip_whitespace(input: &Vec<char>, pos: Index) -> Index {
        let (pos, _) = read_sequence(input, pos, |c| c == ' ', |_| ());
        pos
    }

    pub fn read_number(input: &Vec<char>, pos: Index) -> (Index, usize) {
        read_sequence(
            input,
            pos,
            |c| c.is_digit(10),
            |chars| chars.iter().collect::<String>().parse().unwrap(),
        )
    }

    pub fn read_identifier(input: &Vec<char>, pos: Index) -> (Index, String) {
        read_sequence(
            input,
            pos,
            |c| c.is_alphabetic(),
            |chars| chars.iter().collect(),
        )
    }

    pub fn read_sequence<T>(
        input: &Vec<char>,
        pos: Index,
        predicate: fn(char) -> bool,
        map: fn(Vec<char>) -> T,
    ) -> (Index, T) {
        return aux(input, pos, predicate, map, Vec::new());

        fn aux<T>(
            input: &Vec<char>,
            pos: Index,
            predicate: fn(char) -> bool,
            map: fn(Vec<char>) -> T,
            chars: Vec<char>,
        ) -> (Index, T) {
            let c = input[pos];
            if is_end(input, pos) || !predicate(c) {
                return (pos, map(chars));
            }
            aux(input, advance(pos), predicate, map, append(chars, c))
        }
    }
}

pub fn is_token<T>(expected_token: &T, token: &T) -> bool {
    mem::discriminant(expected_token) == mem::discriminant(token)
}

pub fn is_token_at<T: fmt::Debug>(input: &Vec<T>, pos: Index, expected_token: &T) -> Option<Index> {
    let token = &input[pos];
    if is_token(expected_token, token) {
        Some(advance(pos))
    } else {
        None
    }
}

pub fn parse_numbers<T>(
    tokens: &Vec<T>,
    pos: Index,
    number_token: T,
    extract: fn(&T) -> &usize,
) -> (Index, Vec<usize>) {
    return aux(tokens, pos, number_token, extract, Vec::new());

    fn aux<T>(
        tokens: &Vec<T>,
        pos: Index,
        number_token: T,
        extract: fn(&T) -> &usize,
        numbers: Vec<usize>,
    ) -> (Index, Vec<usize>) {
        let token = &tokens[pos];

        if is_end(tokens, pos) || !is_token(&number_token, token) {
            return (pos, numbers);
        }

        let number = extract(token);

        aux(
            tokens,
            advance(pos),
            number_token,
            extract,
            append(numbers, *number),
        )
    }
}

pub fn expect_token<T: fmt::Debug>(expected_token: &T, token: &T) {
    if !is_token(expected_token, &token) {
        panic!("expected token {expected_token:?}, found {token:?}");
    }
}

pub fn expect_token_at<T: fmt::Debug>(input: &Vec<T>, pos: Index, expected_token: T) -> Index {
    if let Some(pos) = is_token_at(input, pos, &expected_token) {
        pos
    } else {
        panic!("expected token {expected_token:?}, found {:?}", &input[pos]);
    }
}
