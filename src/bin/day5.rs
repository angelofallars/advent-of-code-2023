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
    let input = read("./input/day5.txt")?;

    let almanac = parser::parse(lexer::lex(input));

    let part1_min_location_number = evaluator::eval_part1(almanac.clone());
    println!("Day 5 Part 1 answer: {part1_min_location_number}");

    let part2_min_location_number = evaluator::eval_part2(almanac);
    println!("Day 5 Part 2 answer: {part2_min_location_number}");

    Ok(())
}

mod lexer {
    use super::*;

    #[derive(Debug)]
    pub enum Token {
        Ident(String),
        Number(usize),
        /// '-'
        Slash,
        /// ':'
        Colon,
        /// '\n'
        Newline,
        /// 'to'
        To,
        /// 'map'
        Map,
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
                        "to" => To,
                        "map" => Map,
                        _ => Ident(ident),
                    };
                    (pos, token)
                }
                '-' => (advance(pos), Slash),
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

    #[derive(Debug, Clone)]
    pub struct Almanac {
        pub initial: Initial,
        pub maps: Vec<Map>,
    }

    #[derive(Debug, Clone)]
    pub enum ASTNode {
        Initial(Initial),
        Map(Map),
        IdentFail,
    }

    pub type Number = usize;

    #[derive(Debug, Clone)]
    pub struct Initial {
        pub category: String,
        pub numbers: Vec<Number>,
    }

    #[derive(Debug, Clone)]
    pub struct Range {
        pub dest_start: usize,
        pub src_start: usize,
        pub len: usize,
    }

    #[derive(Debug, Clone)]
    pub struct Map {
        pub src_category: String,
        pub dest_category: String,
        pub numbers: Vec<Range>,
    }

    pub fn parse(tokens: Vec<Token>) -> Almanac {
        let nodes = pt::transform(tokens, |tokens, pos| {
            let pos = skip_newline(tokens, pos);

            use Token::*;
            let token = &tokens[pos];
            match token {
                Ident(_) => {
                    if let Some((pos, node)) = parse_initial(tokens, pos) {
                        (pos, node)
                    } else if let Some((pos, node)) = parse_map(tokens, pos) {
                        (pos, node)
                    } else {
                        (advance(pos), ASTNode::IdentFail)
                    }
                }
                _ => panic!("Unexpected token {token:?}"),
            }
        });

        let ASTNode::Initial(initial) = nodes[0].clone()
        else { panic!("Expected initial first node, found {:?}", nodes[0]) };

        let maps = extract_maps(nodes.into_iter().skip(1).collect());
        fn extract_maps(nodes: Vec<ASTNode>) -> Vec<Map> {
            return aux(&nodes, 0, Vec::new());

            fn aux(nodes: &Vec<ASTNode>, pos: pt::Index, maps: Vec<Map>) -> Vec<Map> {
                if is_end(nodes, pos) {
                    return maps;
                }

                let node = nodes[pos].clone();
                let ASTNode::Map(map) = node
            else { panic!("Expected map node at {pos}, found {:?}", node); };

                aux(nodes, advance(pos), append(maps, map))
            }
        }

        Almanac { initial, maps }
    }

    fn skip_newline(tokens: &Vec<Token>, pos: pt::Index) -> pt::Index {
        return aux(tokens, pos);

        fn aux(tokens: &Vec<Token>, pos: pt::Index) -> pt::Index {
            let token = &tokens[pos];
            if is_end(tokens, pos) || !pt::is_token(&Token::Newline, token) {
                return pos;
            }

            aux(tokens, advance(pos))
        }
    }

    fn parse_initial(tokens: &Vec<Token>, pos: pt::Index) -> Option<(pt::Index, ASTNode)> {
        let Token::Ident(category) = &tokens[pos]
        else { unreachable!() };
        let pos = advance(pos);

        let Some(pos) = pt::is_token_at(tokens, pos, &Token::Colon)
        else { return None; };

        let (pos, numbers) = parse_numbers(tokens, pos);

        let Some(pos) = pt::is_token_at(tokens, pos, &Token::Newline)
        else { return None; };

        Some((
            pos,
            ASTNode::Initial(Initial {
                category: category.to_string(),
                numbers,
            }),
        ))
    }

    fn parse_map(tokens: &Vec<Token>, pos: pt::Index) -> Option<(pt::Index, ASTNode)> {
        let Token::Ident(src_category) = &tokens[pos]
        else { unreachable!() };
        let pos = advance(pos);

        let Some(pos) = pt::is_token_at(tokens, pos, &Token::Slash)
        else { return None; };

        let Some(pos) = pt::is_token_at(tokens, pos, &Token::To)
        else { return None; };

        let Some(pos) = pt::is_token_at(tokens, pos, &Token::Slash)
        else { return None; };

        let Token::Ident(dest_category) = &tokens[pos]
        else { return None; };
        let pos = advance(pos);

        let Some(pos) = pt::is_token_at(tokens, pos, &Token::Map)
        else { return None; };

        let Some(pos) = pt::is_token_at(tokens, pos, &Token::Colon)
        else { return None; };

        let Some(pos) = pt::is_token_at(tokens, pos, &Token::Newline)
        else { return None; };

        let (pos, numbers) = parse_map_numbers(tokens, pos);

        Some((
            pos,
            ASTNode::Map(Map {
                src_category: src_category.to_string(),
                dest_category: dest_category.to_string(),
                numbers,
            }),
        ))
    }

    fn parse_map_numbers(tokens: &Vec<Token>, pos: pt::Index) -> (pt::Index, Vec<Range>) {
        return aux(tokens, pos, Vec::new());

        fn aux(tokens: &Vec<Token>, pos: pt::Index, items: Vec<Range>) -> (pt::Index, Vec<Range>) {
            if is_end(tokens, pos) || !pt::is_token(&Token::Number(0), &tokens[pos]) {
                return (pos, items);
            }

            let (pos, numbers) = parse_numbers(tokens, pos);

            if numbers.len() != 3 {
                panic!(
                    "expected len of 3, found len {} for {:?}",
                    numbers.len(),
                    numbers
                );
            }

            let Some(pos) = pt::is_token_at(tokens, pos, &Token::Newline)
            else { panic!("expected newline, found {:?}", tokens[pos]); };

            aux(
                tokens,
                pos,
                append(
                    items,
                    Range {
                        dest_start: numbers[0],
                        src_start: numbers[1],
                        len: numbers[2],
                    },
                ),
            )
        }
    }

    fn parse_numbers(tokens: &Vec<Token>, pos: pt::Index) -> (pt::Index, Vec<usize>) {
        return aux(tokens, pos, Vec::new());

        fn aux(
            tokens: &Vec<Token>,
            pos: pt::Index,
            numbers: Vec<usize>,
        ) -> (pt::Index, Vec<usize>) {
            let token = &tokens[pos];
            if is_end(tokens, pos) || !pt::is_token(&Token::Number(0), token) {
                return (pos, numbers);
            }

            let Token::Number(number) = token
            else { unreachable!() };

            aux(tokens, advance(pos), append(numbers, *number))
        }
    }
}

mod evaluator {
    use super::parser::Almanac;
    use crate::parser::Map;

    type Mapper = Box<dyn Fn(usize) -> usize>;

    pub fn eval_part1(almanac: Almanac) -> usize {
        let mappers = maps_to_mappers(almanac.maps);
        let seeds = almanac.initial.numbers;

        seeds
            .into_iter()
            .map(|seed| traverse_categories(seed, &mappers))
            .min()
            .unwrap()
    }

    pub fn eval_part2(almanac: Almanac) -> usize {
        let mappers = maps_to_mappers(almanac.maps);
        let seeds = almanac.initial.numbers;

        seeds
            .chunks_exact(2)
            .fold(std::usize::MAX, |acc, seed_range| {
                let start = seed_range[0];
                let len = seed_range[1];

                let min = (start..(start + len))
                    .map(|seed| traverse_categories(seed, &mappers))
                    .min()
                    .unwrap();

                if acc < min {
                    acc
                } else {
                    min
                }
            })
    }

    fn maps_to_mappers(maps: Vec<Map>) -> Vec<Mapper> {
        maps.into_iter()
            .map(|map| {
                let ranges = map.numbers;
                Box::new(move |n| {
                    let Some(index) = ranges
                        .iter()
                        .position(|range| n >= range.src_start && n < range.src_start + range.len)
                    else { return n; };

                    let range = &ranges[index];

                    let start_distance = n - range.src_start;
                    range.dest_start + start_distance
                }) as Mapper
            })
            .collect()
    }

    fn traverse_categories(seed: usize, mappers: &Vec<Mapper>) -> usize {
        mappers.into_iter().fold(seed, |acc, f| f(acc))
    }
}
