use std::{fs, iter::once};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read("./input/day2.txt")?
        .iter()
        .map(|i| *i as char)
        .collect::<Vec<char>>();

    let games = parse(lex(input));
    println!("Day 2 Part 1 answer: {}", sum_possible_ids(&games));
    println!("Day 2 Part 2 answer: {}", sum_power(&games));

    Ok(())
}

fn lex(input: Vec<char>) -> Vec<Token> {
    lex_recursive(&input, 0, Vec::new())
}

fn lex_recursive(input: &Vec<char>, pos: usize, tokens: Vec<Token>) -> Vec<Token> {
    if is_end(&input, pos) {
        return tokens;
    }

    let pos = skip_whitespace(input, pos);
    let c = input[pos];

    let (pos, tokens) = match c {
        ':' => (advance(pos), append(tokens, Token::Colon)),
        ',' => (advance(pos), append(tokens, Token::Comma)),
        ';' => (advance(pos), append(tokens, Token::Semicolon)),
        '\n' => (advance(pos), append(tokens, Token::Newline)),
        _ => {
            if c.is_digit(10) {
                let (pos, digit) = read_number(&input, pos);
                (pos, append(tokens, Token::Digit(digit)))
            } else if c.is_alphabetic() {
                let (pos, ident) = read_ident(&input, pos);
                let token = match ident.as_str() {
                    "red" => Token::Color(Color::Red),
                    "green" => Token::Color(Color::Green),
                    "blue" => Token::Color(Color::Blue),
                    "Game" => Token::Game,
                    _ => unreachable!(),
                };
                (pos, append(tokens, token))
            } else {
                panic!("Unknown character at {}: {}", pos, c);
            }
        }
    };

    lex_recursive(input, pos, tokens)
}

fn skip_whitespace(input: &Vec<char>, pos: usize) -> usize {
    skip_whitespace_recursive(input, pos)
}

fn skip_whitespace_recursive(input: &Vec<char>, pos: usize) -> usize {
    let c = input[pos];
    if is_end(input, pos) || (c != ' ') {
        pos
    } else {
        skip_whitespace_recursive(input, advance(pos))
    }
}

fn read_number(input: &Vec<char>, pos: usize) -> (usize, usize) {
    read_number_recursive(input, pos, Vec::new())
}

fn read_number_recursive(input: &Vec<char>, pos: usize, chars: Vec<char>) -> (usize, usize) {
    let c = input[pos];
    if is_end(input, pos) || !c.is_digit(10) {
        (pos, parse_number(chars))
    } else {
        read_number_recursive(input, advance(pos), append(chars, c))
    }
}

fn read_ident(input: &Vec<char>, pos: usize) -> (usize, String) {
    read_ident_recursive(input, pos, Vec::new())
}

fn read_ident_recursive(input: &Vec<char>, pos: usize, chars: Vec<char>) -> (usize, String) {
    let c = input[pos];
    if is_end(input, pos) || !c.is_alphabetic() {
        (pos, parse_string(chars))
    } else {
        read_ident_recursive(input, advance(pos), append(chars, c))
    }
}

fn is_end<T>(input: &Vec<T>, pos: usize) -> bool {
    input.len() <= pos
}

fn parse_number(vec: Vec<char>) -> usize {
    vec.iter().collect::<String>().parse().unwrap()
}

fn parse_string(vec: Vec<char>) -> String {
    vec.iter().collect::<String>()
}

#[derive(Debug)]
enum Token {
    Digit(usize),
    /// "red", "green", "blue"
    Color(Color),
    /// ,
    Comma,
    /// :
    Colon,
    /// ;
    Semicolon,
    /// \n
    Newline,

    /// "Game"
    Game,
}

#[derive(Debug, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

fn parse(tokens: Vec<Token>) -> Vec<Game> {
    parse_recursive(tokens, 0, Vec::new())
}

fn parse_recursive(tokens: Vec<Token>, pos: usize, games: Vec<Game>) -> Vec<Game> {
    if is_end(&tokens, pos) {
        return games;
    }

    match tokens[pos] {
        Token::Game => {
            let pos = advance(pos);

            let Token::Digit(id) = tokens[pos]
            else { unreachable!() };
            let pos = advance(pos);

            // Skip colon
            let pos = advance(pos);

            let (pos, sets) = parse_sets(&tokens, pos);
            let games = append(games, Game { id, sets });
            return parse_recursive(tokens, pos, games);
        }
        _ => panic!("{:#?}: {:#?}", pos, tokens[pos]),
    }
}

fn parse_sets(tokens: &Vec<Token>, pos: usize) -> (usize, Vec<Vec<Cubes>>) {
    parse_sets_recursive(tokens, pos, Vec::new())
}

fn parse_sets_recursive(
    tokens: &Vec<Token>,
    pos: usize,
    sets: Vec<Vec<Cubes>>,
) -> (usize, Vec<Vec<Cubes>>) {
    // set grammar: [Digit, Color, Comma]+, Semicolon
    let is_end_of_sets = match tokens[pos] {
        Token::Newline => true,
        _ => false,
    };
    if is_end(tokens, pos) || is_end_of_sets {
        let pos = advance(pos);
        return (pos, sets);
    }
    let is_semicolon = match tokens[pos] {
        Token::Semicolon => true,
        _ => false,
    };
    if is_semicolon {
        let pos = advance(pos);
        return parse_sets_recursive(tokens, pos, sets);
    }

    let (pos, subsets) = parse_cubes(tokens, pos);
    let sets = append(sets, subsets);

    parse_sets_recursive(tokens, pos, sets)
}

fn parse_cubes(tokens: &Vec<Token>, pos: usize) -> (usize, Vec<Cubes>) {
    parse_cubes_recursive(tokens, pos, Vec::new())
}

fn parse_cubes_recursive(
    tokens: &Vec<Token>,
    pos: usize,
    subsets: Vec<Cubes>,
) -> (usize, Vec<Cubes>) {
    let is_end_of_subsets = match tokens[pos] {
        Token::Semicolon => true,
        Token::Newline => true,
        _ => false,
    };
    if is_end(tokens, pos) || is_end_of_subsets {
        return (pos, subsets);
    }

    let Token::Digit(count) = tokens[pos]
    else { panic!("{}: {:#?}", pos, tokens[pos]) };
    let pos = advance(pos);

    let Token::Color(ref color) = tokens[pos]
    else { panic!("{}: {:#?}", pos, tokens[pos]) };

    let subsets = append(
        subsets,
        Cubes {
            count,
            color: color.clone(),
        },
    );
    let pos = advance(pos);

    if let Token::Comma = tokens[pos] {
        parse_cubes_recursive(tokens, advance(pos), subsets)
    } else {
        parse_cubes_recursive(tokens, pos, subsets)
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Vec<Cubes>>,
}

#[derive(Debug)]
struct Cubes {
    count: usize,
    color: Color,
}

fn sum_possible_ids(games: &Vec<Game>) -> usize {
    sum_possible_ids_recursive(games, 0, 0)
}

fn sum_possible_ids_recursive(games: &Vec<Game>, pos: usize, acc: usize) -> usize {
    if is_end(games, pos) {
        return acc;
    }
    let game = &games[pos];
    let pos = advance(pos);

    if is_possible(&game) {
        let acc = acc + game.id;
        sum_possible_ids_recursive(games, pos, acc)
    } else {
        sum_possible_ids_recursive(games, pos, acc)
    }
}

fn is_possible(game: &Game) -> bool {
    !game.sets.iter().any(|set| {
        set.iter().any(|cubes| match cubes.color {
            Color::Red => cubes.count > 12,
            Color::Green => cubes.count > 13,
            Color::Blue => cubes.count > 14,
        })
    })
}

fn sum_power(games: &Vec<Game>) -> usize {
    sum_power_recursive(games, 0, 0)
}

fn sum_power_recursive(games: &Vec<Game>, pos: usize, accumulated_power: usize) -> usize {
    if is_end(games, pos) {
        return accumulated_power;
    }
    let game = &games[pos];
    let pos = advance(pos);

    sum_power_recursive(games, pos, accumulated_power + calculate_power(game))
}

fn calculate_power(game: &Game) -> usize {
    let (red_min, green_min, blue_min) = game.sets.iter().fold((0, 0, 0), |acc, set| {
        let (red_min, green_min, blue_min) =
            set.iter().fold((0, 0, 0), |acc, cubes| match cubes.color {
                Color::Red => (max(acc.0, cubes.count), acc.1, acc.2),
                Color::Green => (acc.0, max(acc.1, cubes.count), acc.2),
                Color::Blue => (acc.0, acc.1, max(acc.2, cubes.count)),
            });

        (
            max(acc.0, red_min),
            max(acc.1, green_min),
            max(acc.2, blue_min),
        )
    });

    red_min * green_min * blue_min
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else if a < b {
        b
    } else {
        a
    }
}

/// append works exactly like Go's `append` function.
fn append<T: IntoIterator<Item = U> + FromIterator<U>, U>(i: T, elem: U) -> T {
    i.into_iter().chain(once(elem)).collect()
}

fn advance(i: usize) -> usize {
    i + 1
}
