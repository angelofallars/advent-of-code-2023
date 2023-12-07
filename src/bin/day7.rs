use std::error::Error;

#[path = "../read.rs"]
mod read;
use crate::parser::Hand;
use crate::read::read;

#[path = "../parsetools.rs"]
mod parsetools;
use crate::parsetools as pt;
use crate::parsetools::lextools as lt;
use crate::parsetools::Index;

#[path = "../functools.rs"]
mod functools;
use crate::functools::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read("./input/day7.txt")?;

    let tokens = lexer::lex(input);
    let hands = parser::parse(&tokens);

    let part1_total_winnings = eval_part1::eval(&hands);
    println!("Day 7 Part 1 answer: {part1_total_winnings}");

    let part2_total_winnings = eval_part2::eval(&hands);
    println!("Day 7 Part 1 answer: {part2_total_winnings}");

    Ok(())
}

mod lexer {
    use super::*;

    #[derive(Debug)]
    pub enum Token {
        Char(char),
        /// ' '
        Space,
        /// '\n'
        Newline,
    }

    pub fn lex(input: Vec<char>) -> Vec<Token> {
        pt::transform(input, |input, pos| {
            use Token::*;
            let c = input[pos];
            match c {
                '0'..='9' | 'A' | 'K' | 'Q' | 'J' | 'T' => (advance(pos), Char(c)),
                ' ' => (advance(pos), Space),
                '\n' => (advance(pos), Newline),
                _ => panic!("Unknown character at {pos}: {c}"),
            }
        })
    }
}

mod parser {
    use super::lexer::Token;
    use super::*;

    #[derive(Debug)]
    pub struct Hand {
        pub cards: FiveLabels,
        pub bid: BidAmount,
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Label {
        A = 13,
        K = 12,
        Q = 11,
        J = 10,
        T = 9,
        _9 = 8,
        _8 = 7,
        _7 = 6,
        _6 = 5,
        _5 = 4,
        _4 = 3,
        _3 = 2,
        _2 = 1,
    }

    impl Label {
        fn try_from(c: char) -> Option<Label> {
            use Label::*;
            let label = match c {
                'A' => A,
                'K' => K,
                'Q' => Q,
                'J' => J,
                'T' => T,
                '9' => _9,
                '8' => _8,
                '7' => _7,
                '6' => _6,
                '5' => _5,
                '4' => _4,
                '3' => _3,
                '2' => _2,
                _ => return None,
            };
            return Some(label);
        }
    }

    pub type BidAmount = usize;
    pub type FiveLabels = [Label; 5];

    pub fn parse(tokens: &Vec<Token>) -> Vec<Hand> {
        return aux(tokens, 0, Vec::new());

        fn aux(tokens: &Vec<Token>, pos: Index, hands: Vec<Hand>) -> Vec<Hand> {
            tail_end!(tokens[pos], return hands);

            let (pos, hand) = parse_hand(tokens, pos);

            aux(tokens, pos, append(hands, hand))
        }
    }

    fn parse_hand(tokens: &Vec<Token>, pos: Index) -> (Index, Hand) {
        let (pos, cards) = parse_cards(tokens, pos);

        let pos = pt::expect_token_at(tokens, pos, Token::Space);

        let (pos, bid) = parse_number(tokens, pos);

        let pos = pt::expect_token_at(tokens, pos, Token::Newline);

        let hand = Hand { cards, bid };

        (pos, hand)
    }

    fn parse_cards(tokens: &Vec<Token>, pos: Index) -> (Index, FiveLabels) {
        let (pos, labels) = aux(tokens, pos, 0, Vec::new());

        fn aux(
            tokens: &Vec<Token>,
            pos: Index,
            relative_pos: usize,
            labels: Vec<Label>,
        ) -> (Index, Vec<Label>) {
            tail_end!(tokens[pos], if relative_pos >= 5, return (pos, labels));

            let Token::Char(c) = &tokens[pos]
            else { panic!("expected Char token, found {:?}", &tokens[pos]); };
            let label = Label::try_from(*c).unwrap();

            aux(
                tokens,
                advance(pos),
                advance(relative_pos),
                append(labels, label),
            )
        }

        if labels.len() != 5 {
            panic!("Expected label length of 5, found {}", labels.len());
        }

        return (
            pos,
            [
                labels[0].clone(),
                labels[1].clone(),
                labels[2].clone(),
                labels[3].clone(),
                labels[4].clone(),
            ],
        );
    }

    fn parse_number(tokens: &Vec<Token>, pos: Index) -> (Index, usize) {
        return aux(tokens, pos, 0);

        fn aux(tokens: &Vec<Token>, pos: Index, number: usize) -> (Index, usize) {
            tail_end!(tokens[pos], return (pos, number));

            let (pos, Token::Char(c)) = (advance(pos), &tokens[pos])
            else { return (pos, number); };

            let digit = c.to_digit(10).unwrap() as usize;

            aux(tokens, pos, (number * 10) + digit)
        }
    }
}

mod eval_common {
    use crate::parser::Hand;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HandType {
        FiveOfAKind = 7,
        FourOfAKind = 6,
        FullHouse = 5,
        ThreeOfAKind = 4,
        TwoPair = 3,
        OnePair = 2,
        HighCard = 1,
    }

    impl PartialEq for Hand {
        fn eq(&self, _: &Self) -> bool {
            false
        }
    }

    pub fn eval_base(hands: &Vec<Hand>) -> usize {
        let mut hands: Vec<_> = hands.into_iter().map(|hand| hand.clone()).collect();
        hands.sort_by(|a, b| (*a).partial_cmp(*b).unwrap());

        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, hand| acc + (hand.1.bid * (hand.0 + 1)))
    }
}

mod eval_part1 {
    use std::cmp::Ordering;
    use std::collections::HashMap;

    use crate::parser::FiveLabels;

    use super::eval_common::{eval_base, HandType};
    use super::parser::Hand;

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let ord = {
                let self_type = HandType::from(self.cards.clone());
                let other_type = HandType::from(other.cards.clone());

                if self_type > other_type {
                    Ordering::Greater
                } else if self_type < other_type {
                    Ordering::Less
                } else {
                    let mut ordering = None;

                    for (self_label, other_label) in self.cards.iter().zip(other.cards.iter()) {
                        if self_label > other_label {
                            ordering = Some(Ordering::Greater);
                            break;
                        } else if self_label < other_label {
                            ordering = Some(Ordering::Less);
                            break;
                        }
                    }

                    match ordering {
                        Some(ordering) => ordering,
                        None => Ordering::Equal,
                    }
                }
            };

            Some(ord)
        }
    }

    impl From<FiveLabels> for HandType {
        fn from(labels: FiveLabels) -> Self {
            let mut occurence_map = HashMap::new();

            for label in labels {
                let count = if let Some(count) = occurence_map.get(&label) {
                    count + 1
                } else {
                    1
                };
                occurence_map.insert(label, count);
            }

            let mut occ: Vec<usize> = occurence_map.into_iter().map(|(_, count)| count).collect();
            occ.sort_by(|a, b| b.cmp(a));

            use HandType::*;
            let kind = if 5 == occ[0] {
                FiveOfAKind
            } else if 4 == occ[0] {
                FourOfAKind
            } else if 3 == occ[0] {
                if 2 == occ[1] {
                    FullHouse
                } else {
                    ThreeOfAKind
                }
            } else if occ[0] == 2 {
                if 2 == occ[1] {
                    TwoPair
                } else {
                    OnePair
                }
            } else {
                HighCard
            };
            return kind;
        }
    }

    pub fn eval(hands: &Vec<Hand>) -> usize {
        eval_base(hands)
    }
}

mod eval_part2 {
    use std::cmp::Ordering;
    use std::collections::HashMap;

    use crate::parser::{FiveLabels, Label};

    use super::eval_common::HandType;
    use super::parser::Hand;

    fn cmp_hand(first: &Hand, second: &Hand) -> Option<Ordering> {
        let ord = {
            let first_type = hand_type_from_labels(first.cards.clone());
            let second_type = hand_type_from_labels(second.cards.clone());

            if first_type > second_type {
                Ordering::Greater
            } else if first_type < second_type {
                Ordering::Less
            } else {
                let mut ordering = None;

                for (first_label, second_label) in first.cards.iter().zip(second.cards.iter()) {
                    let first_is_j = if let Label::J = first_label {
                        true
                    } else {
                        false
                    };
                    let second_is_j = if let Label::J = second_label {
                        true
                    } else {
                        false
                    };

                    if !first_is_j && second_is_j {
                        ordering = Some(Ordering::Greater);
                        break;
                    }

                    if first_is_j && !second_is_j {
                        ordering = Some(Ordering::Less);
                        break;
                    }

                    if first_label > second_label {
                        ordering = Some(Ordering::Greater);
                        break;
                    } else if first_label < second_label {
                        ordering = Some(Ordering::Less);
                        break;
                    }
                }

                match ordering {
                    Some(ordering) => ordering,
                    None => Ordering::Equal,
                }
            }
        };

        Some(ord)
    }

    fn hand_type_from_labels(labels: FiveLabels) -> HandType {
        let mut occurence_map = HashMap::new();

        let mut joker_count = 0;
        for label in &labels {
            if let Label::J = label {
                joker_count += 1;
                continue;
            }

            let count = if let Some(count) = occurence_map.get(&label) {
                count + 1
            } else {
                1
            };
            occurence_map.insert(label, count);
        }

        let mut occ: Vec<usize> = occurence_map.into_iter().map(|(_, count)| count).collect();
        occ.sort_by(|a, b| b.cmp(a));

        if let Some(_) = occ.get(0) {
            occ[0] += joker_count;
        } else {
            occ.push(joker_count);
        }

        use HandType::*;
        let kind = if 5 == occ[0] {
            FiveOfAKind
        } else if 4 == occ[0] {
            FourOfAKind
        } else if 3 == occ[0] {
            if 2 == occ[1] {
                FullHouse
            } else {
                ThreeOfAKind
            }
        } else if occ[0] == 2 {
            if 2 == occ[1] {
                TwoPair
            } else {
                OnePair
            }
        } else {
            HighCard
        };

        return kind;
    }

    pub fn eval(hands: &Vec<Hand>) -> usize {
        let mut hands: Vec<_> = hands.into_iter().map(|hand| hand.clone()).collect();
        hands.sort_by(|a, b| cmp_hand(*a, *b).unwrap());

        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, hand| acc + (hand.1.bid * (hand.0 + 1)))
    }
}
