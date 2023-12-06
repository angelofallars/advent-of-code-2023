use std::error::Error;

#[path = "../read.rs"]
mod read;
use crate::read::read;

#[path = "../parsetools.rs"]
mod parsetools;
use crate::parsetools as pt;
use crate::parsetools::lextools as lt;

#[path = "../functools.rs"]
mod functools;
use crate::functools::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read("./input/day6.txt")?;

    let tokens = lexer::lex(input);

    let part1_races = parser::parse_part1(&tokens);
    let part1_ways = evaluator::eval(&part1_races);
    println!("{part1_ways}");

    let part2_race = parser::parse_part2(&tokens);
    let part2_ways = evaluator::calc_record_beaters(&part2_race);
    println!("{part2_ways}");

    Ok(())
}

mod lexer {
    use super::*;

    #[derive(Debug)]
    pub enum Token {
        Number(usize),
        /// ':'
        Colon,
        /// '\n'
        Newline,
        /// 'Time'
        Time,
        /// 'Distance'
        Distance,
    }

    pub fn lex(input: Vec<char>) -> Vec<Token> {
        pt::transform(input, |input, pos| {
            let pos = lt::skip_whitespace(input, pos);

            use Token::*;
            let c = input[pos];
            match c {
                '0'..='9' => {
                    let (pos, number) = lt::read_number(input, pos);
                    (pos, Number(number))
                }
                c if c.is_ascii_alphabetic() => {
                    let (pos, ident) = lt::read_identifier(input, pos);
                    let token = match ident.as_str() {
                        "Time" => Time,
                        "Distance" => Distance,
                        _ => panic!("Unknown identifier at {pos}: {ident}"),
                    };
                    (pos, token)
                }
                ':' => (advance(pos), Colon),
                '\n' => (advance(pos), Newline),
                _ => panic!("Unknown character at {pos}: {c}"),
            }
        })
    }
}

mod parser {
    use super::lexer::Token;
    use super::*;

    pub type Duration = usize;
    pub type Distance = usize;

    #[derive(Debug)]
    pub struct Race {
        pub duration: Duration,
        pub record_distance: Distance,
    }

    pub fn parse_part1(tokens: &Vec<Token>) -> Vec<Race> {
        let pos = 0;

        let (pos, durations) = parse_durations(tokens, pos);
        let (_, distances) = parse_distances(tokens, pos);

        durations
            .into_iter()
            .zip(distances)
            .map(|(duration, distance)| Race {
                duration,
                record_distance: distance,
            })
            .collect()
    }

    pub fn parse_part2(tokens: &Vec<Token>) -> Race {
        let races = parse_part1(tokens);

        let durations = races.iter().map(|race| (race.duration));
        let distances = races.iter().map(|race| (race.record_distance));

        let duration = concat_numbers(durations);
        let record_distance = concat_numbers(distances);

        Race {
            duration,
            record_distance,
        }
    }

    fn parse_durations(tokens: &Vec<Token>, pos: pt::Index) -> (pt::Index, Vec<Duration>) {
        parse_section(tokens, pos, Token::Time)
    }

    fn parse_distances(tokens: &Vec<Token>, pos: pt::Index) -> (pt::Index, Vec<Distance>) {
        parse_section(tokens, pos, Token::Distance)
    }

    fn parse_section(
        tokens: &Vec<Token>,
        pos: pt::Index,
        first_token: Token,
    ) -> (pt::Index, Vec<Distance>) {
        let pos = pt::expect_token_at(tokens, pos, first_token);
        let pos = pt::expect_token_at(tokens, pos, Token::Colon);
        let (pos, numbers) = parse_numbers(tokens, pos);
        let pos = pt::expect_token_at(tokens, pos, Token::Newline);
        (pos, numbers)
    }

    fn parse_numbers(tokens: &Vec<Token>, pos: pt::Index) -> (pt::Index, Vec<usize>) {
        pt::parse_numbers(tokens, pos, Token::Number(0), |t| {
            if let Token::Number(number) = t {
                number
            } else {
                unreachable!()
            }
        })
    }

    fn concat_numbers<T: IntoIterator<Item = usize>>(numbers: T) -> usize {
        String::from_utf8(numbers.into_iter().fold(Vec::new(), |acc, duration| {
            extend(acc, duration.to_string().into_bytes())
        }))
        .unwrap()
        .parse()
        .unwrap()
    }
}

mod evaluator {
    use super::parser::{Distance, Duration, Race};

    pub fn eval(races: &Vec<Race>) -> usize {
        races
            .into_iter()
            .map(|race| calc_record_beaters(&race))
            .reduce(|acc, next| acc * next)
            .unwrap()
    }

    pub fn calc_record_beaters(race: &Race) -> usize {
        (0..=race.duration)
            .filter(|hold_duration| {
                calc_distance_travelled(race, *hold_duration) > race.record_distance
            })
            .count()
    }

    fn calc_distance_travelled(race: &Race, hold_duration: Duration) -> Distance {
        let boat_speed_per_second = hold_duration;

        let remaining_duration = race.duration - hold_duration;

        boat_speed_per_second * remaining_duration
    }
}
