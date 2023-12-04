use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::mem;

#[path = "../functools.rs"]
mod functools;
use crate::functools::*;

use lexer::Token;
use parser::Card;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read("./input/day4.txt")?
        .iter()
        .map(|i| *i as char)
        .collect::<Vec<char>>();

    let cards = parser::parse(lexer::lex(input));
    let pt1_score = evaluator::eval_pt1(&cards);
    println!("Day 4 Part 1 answer: {:?}", pt1_score);
    let pt2_score = evaluator::eval_pt2(&cards);
    println!("Day 4 Part 2 answer: {:?}", pt2_score);

    Ok(())
}

mod lexer {
    use super::*;

    #[derive(Debug)]
    pub enum Token {
        Number(usize),
        /// ':'
        Colon,
        /// '|'
        Pipe,
        /// '\n'
        Newline,
        /// 'Card'
        Card,
    }

    pub fn lex(input: Vec<char>) -> Vec<Token> {
        lex_core(&input, 0, Vec::new())
    }
    fn lex_core(input: &Vec<char>, pos: usize, tokens: Vec<Token>) -> Vec<Token> {
        if is_end(input, pos) {
            return tokens;
        }

        let pos = skip_whitespace(input, pos);

        use Token::*;
        let c = input[pos];
        let (pos, token) = match c {
            '0'..='9' => {
                let (pos, number) = read_number(input, pos);
                (pos, Number(number))
            }
            'C' => {
                let (pos, ident) = read_ident(input, pos);
                if ident != "Card" {
                    panic!("Invalid identifier at {}: {}", pos, ident);
                }
                (pos, Card)
            }
            ':' => (advance(pos), Colon),
            '|' => (advance(pos), Pipe),
            '\n' => (advance(pos), Newline),
            _ => {
                panic!("Unknown character at {}: {}", pos, c)
            }
        };

        lex_core(input, pos, append(tokens, token))
    }

    fn skip_whitespace(input: &Vec<char>, pos: usize) -> usize {
        skip_whitespace_core(input, pos)
    }
    fn skip_whitespace_core(input: &Vec<char>, pos: usize) -> usize {
        let c = input[pos];
        if c != ' ' {
            return pos;
        }
        skip_whitespace_core(input, advance(pos))
    }

    fn read_number(input: &Vec<char>, pos: usize) -> (usize, usize) {
        read_sequence(
            input,
            pos,
            |c| c.is_digit(10),
            |chars| chars.iter().collect::<String>().parse().unwrap(),
        )
    }

    fn read_ident(input: &Vec<char>, pos: usize) -> (usize, String) {
        read_sequence(
            input,
            pos,
            |c| c.is_alphabetic(),
            |chars| chars.iter().collect(),
        )
    }

    fn read_sequence<T>(
        input: &Vec<char>,
        pos: usize,
        predicate: fn(char) -> bool,
        map: fn(Vec<char>) -> T,
    ) -> (usize, T) {
        read_sequence_core(input, pos, predicate, map, Vec::new())
    }
    fn read_sequence_core<T>(
        input: &Vec<char>,
        pos: usize,
        predicate: fn(char) -> bool,
        map: fn(Vec<char>) -> T,
        chars: Vec<char>,
    ) -> (usize, T) {
        let c = input[pos];
        if is_end(input, pos) || !predicate(c) {
            return (pos, map(chars));
        }
        read_sequence_core(input, advance(pos), predicate, map, append(chars, c))
    }
}

mod parser {
    use super::*;

    #[derive(Debug)]
    pub struct Card {
        pub id: usize,
        pub set_a: Vec<usize>,
        pub set_b: Vec<usize>,
    }

    pub fn parse(tokens: Vec<Token>) -> Vec<Card> {
        parse_core(&tokens, 0, Vec::new())
    }
    fn parse_core(tokens: &Vec<Token>, pos: usize, cards: Vec<Card>) -> Vec<Card> {
        if is_end(tokens, pos) {
            return cards;
        }

        let token = &tokens[pos];
        use Token::*;
        let (pos, card) = match token {
            Card => parse_card(tokens, pos),
            _ => panic!("Invalid token at {}: {:?}", pos, token),
        };

        parse_core(tokens, pos, append(cards, card))
    }

    fn parse_card(tokens: &Vec<Token>, pos: usize) -> (usize, Card) {
        let pos = advance(pos);
        expect_token(Token::Number(0), &tokens[pos]);
        let Token::Number(id) = tokens[pos]
    else { unreachable!() };

        let pos = advance(pos);
        expect_token(Token::Colon, &tokens[pos]);

        let pos = advance(pos);
        let (pos, set_a) = parse_set_a(tokens, pos);
        let (pos, set_b) = parse_set_b(tokens, pos);

        (pos, Card { id, set_a, set_b })
    }

    fn parse_set_a(tokens: &Vec<Token>, pos: usize) -> (usize, Vec<usize>) {
        parse_set_core(tokens, pos, mem::discriminant(&Token::Pipe), Vec::new())
    }
    fn parse_set_b(tokens: &Vec<Token>, pos: usize) -> (usize, Vec<usize>) {
        parse_set_core(tokens, pos, mem::discriminant(&Token::Newline), Vec::new())
    }
    fn parse_set_core(
        tokens: &Vec<Token>,
        pos: usize,
        end_token: mem::Discriminant<Token>,
        numbers: Vec<usize>,
    ) -> (usize, Vec<usize>) {
        if is_end(tokens, pos) {
            return (pos, numbers);
        }
        if end_token == mem::discriminant(&tokens[pos]) {
            return (advance(pos), numbers);
        }

        expect_token(Token::Number(0), &tokens[pos]);
        let Token::Number(number) = tokens[pos]
    else { unreachable!() };

        parse_set_core(tokens, advance(pos), end_token, append(numbers, number))
    }

    fn expect_token(expected_token: Token, token: &Token) {
        if mem::discriminant(&expected_token) != mem::discriminant(&token) {
            panic!("expected token {:?}, found {:?}", expected_token, token);
        }
    }
}

mod evaluator {
    use super::*;

    pub fn eval_pt1(cards: &Vec<Card>) -> usize {
        eval_pt1_core(cards, 0)
    }
    fn eval_pt1_core(cards: &Vec<Card>, pos: usize) -> usize {
        if is_end(cards, pos) {
            return 0;
        }

        let card = &cards[pos];
        calculate_points(card) + eval_pt1_core(cards, advance(pos))
    }

    fn calculate_points(card: &Card) -> usize {
        let match_count = calculate_matches(card);
        if match_count == 0 {
            return 0;
        }

        let base: usize = 2;
        base.pow((match_count - 1) as u32)
    }
    fn calculate_matches(card: &Card) -> usize {
        let a: HashSet<usize> = HashSet::from_iter(card.set_a.clone());
        let b: HashSet<usize> = HashSet::from_iter(card.set_b.clone());
        a.intersection(&b).count()
    }

    pub fn eval_pt2(cards: &Vec<Card>) -> usize {
        let points: Vec<usize> = cards
            .into_iter()
            .map(|card| calculate_matches(card))
            .collect();
        let instances: Vec<usize> = cards.into_iter().map(|_| 1).collect();
        eval_pt2_core(cards, points, 0, instances)
    }
    fn eval_pt2_core(
        cards: &Vec<Card>,
        points: Vec<usize>,
        pos: usize,
        instances: Vec<usize>,
    ) -> usize {
        if is_end(cards, pos) {
            return instances.into_iter().sum();
        }

        let multiplier = instances[pos];
        let copy_ids: Vec<usize> = (0..points[pos])
            .into_iter()
            .map(|point| point + pos + 1)
            .collect();
        let instances = instances
            .into_iter()
            .enumerate()
            .map(|(idx, instance)| {
                if copy_ids.contains(&idx) {
                    instance + multiplier
                } else {
                    instance
                }
            })
            .collect();

        eval_pt2_core(cards, points, advance(pos), instances)
    }
}