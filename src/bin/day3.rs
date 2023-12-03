use std::{fs, iter::once, ops::Index};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read("./input/day3.txt")?
        .iter()
        .map(|i| *i as char)
        .collect::<Vec<char>>();

    // let tokens = lex(input);
    // println!("tokens: {:?}", tokens);
    println!("Day 3 Part 1 answer: {:?}", sum(lex(&input)));
    println!("Day 3 Part 2 answer: {:?}", sum_pt2(lex_pt2(&input)));

    Ok(())
}

fn lex(input: &CharView) -> Vec<Symbol> {
    lex_recursive(
        input,
        0,
        |c| c != '.' && !c.is_digit(10) && c != '\n',
        Vec::new(),
    )
}

fn lex_pt2(input: &CharView) -> Vec<Symbol> {
    lex_recursive(input, 0, |c| c == '*', Vec::new())
}

fn lex_recursive(
    input: &CharView,
    pos: usize,
    accept: fn(char) -> bool,
    symbols: Vec<Symbol>,
) -> Vec<Symbol> {
    if is_end(input, pos) {
        return symbols;
    }

    let c = input[pos];
    let (pos, symbols) = if accept(c) {
        (advance(pos), append(symbols, new_symbol(input, pos)))
    } else {
        (advance(pos), symbols)
    };

    lex_recursive(input, pos, accept, symbols)
}

fn new_symbol(input: &CharView, start_pos: usize) -> Symbol {
    let line_len = input
        .clone()
        .into_iter()
        .collect::<String>()
        .split("\n")
        .next()
        .unwrap()
        .len()
        + 1;
    let line_count = input
        .clone()
        .into_iter()
        .collect::<String>()
        .split("\n")
        .count()
        - 1;
    // let input = &input.clone().into_iter().filter(|c| c != &'\n').collect();

    new_symbol_recursive(
        input,
        start_pos,
        line_len,
        line_count,
        Direction::first(),
        Vec::new(),
    )
}

fn new_symbol_recursive(
    input: &CharView,
    start_pos: usize,
    line_len: usize,
    line_count: usize,
    direction: Direction,
    symbol: Symbol,
) -> Symbol {
    if direction.is_end() {
        return symbol;
    }

    let symbol = if let Some(pos) = direction.pos(start_pos, line_len, line_count) {
        if let Some(chars) = find_digit_bytes(input, pos) {
            if !symbol.contains(&chars) {
                append(symbol, chars)
            } else {
                symbol
            }
        } else {
            symbol
        }
    } else {
        symbol
    };

    new_symbol_recursive(
        input,
        start_pos,
        line_len,
        line_count,
        direction.next(),
        symbol,
    )
}

fn find_digit_bytes(input: &CharView, start_pos: usize) -> Option<CharView> {
    let c = input[start_pos];
    if !c.is_digit(10) {
        return None;
    }

    let left: CharView = find_digit_bytes_recursive(input, start_pos, -1, |p| p - 1, Vec::new())
        .into_iter()
        .rev()
        .collect();
    let right = find_digit_bytes_recursive(input, start_pos, 1, |p| p + 1, Vec::new());

    Some(extend(extend(left, vec![c]), right))
}

fn find_digit_bytes_recursive(
    input: &CharView,
    start_pos: usize,
    relative_pos: isize,
    transform: fn(isize) -> isize,
    bytes: CharView,
) -> CharView {
    let pos = start_pos as isize + relative_pos;
    if is_end(input, pos as usize) || pos < 0 {
        return bytes;
    }

    let c = input[pos as usize];
    if !c.is_digit(10) {
        return bytes;
    }

    find_digit_bytes_recursive(
        input,
        start_pos,
        transform(relative_pos),
        transform,
        append(bytes, c),
    )
}

type Symbol = Vec<CharView>;
type CharView = Vec<char>;

enum Direction {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    End,
}

impl Direction {
    fn first() -> Direction {
        Direction::NorthWest
    }
    fn next(&self) -> Direction {
        use Direction::*;
        match self {
            NorthWest => North,
            North => NorthEast,
            NorthEast => East,
            East => SouthEast,
            SouthEast => South,
            South => SouthWest,
            SouthWest => West,
            West => End,
            End => panic!("Tried going next for End"),
        }
    }

    fn pos(&self, pos: usize, line_len: usize, line_count: usize) -> Option<usize> {
        use Direction::*;
        let x = pos % line_len;
        let y = pos / line_len;
        let (x_diff, y_diff): (isize, isize) = match self {
            NorthWest => (-1, -1),
            North => (0, -1),
            NorthEast => (1, -1),
            East => (1, 0),
            SouthEast => (1, 1),
            South => (0, 1),
            SouthWest => (-1, 1),
            West => (-1, 0),
            End => panic!("Tried calculating position for End"),
        };

        let out_of_bounds_left = x_diff == -1 && x == 0;
        if out_of_bounds_left {
            return None;
        }
        let out_of_bounds_right = x_diff == 1 && (x >= line_len - 1);
        if out_of_bounds_right {
            return None;
        }
        let out_of_bounds_top = y_diff == -1 && y == 0;
        if out_of_bounds_top {
            return None;
        }
        let out_of_bounds_bottom = y_diff == 1 && (y >= line_count - 1);
        if out_of_bounds_bottom {
            return None;
        }

        Some(((pos as isize + x_diff) + (line_len as isize * y_diff)) as usize)
    }
    fn is_end(&self) -> bool {
        match self {
            Direction::End => true,
            _ => false,
        }
    }
}

fn sum(symbols: Vec<Symbol>) -> usize {
    symbols.into_iter().fold(0, |acc, s| {
        acc + s.into_iter().fold(0, |acc, chars| {
            acc + chars
                .into_iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
    })
}

fn sum_pt2(symbols: Vec<Symbol>) -> usize {
    symbols
        .into_iter()
        .filter(|s| s.len() == 2)
        .fold(0, |acc, s| {
            acc + s.into_iter().fold(1, |acc, chars| {
                acc * chars
                    .into_iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
            })
        })
}

fn is_end<T>(input: &Vec<T>, pos: usize) -> bool {
    input.len() <= pos
}

/// append works exactly like Go's `append` function.
fn append<T: IntoIterator<Item = U> + FromIterator<U>, U>(i: T, elem: U) -> T {
    i.into_iter().chain(once(elem)).collect()
}

fn extend<
    T: IntoIterator<Item = V> + FromIterator<V>,
    U: IntoIterator<Item = V> + FromIterator<V>,
    V,
>(
    a: T,
    b: U,
) -> U {
    a.into_iter().chain(b).collect()
}

fn advance(i: usize) -> usize {
    i + 1
}
