use std::fs;
use regex::Regex;

struct Board {
    identifiers: Vec<Identifier>,
    symbols: Vec<Symbol>,
}

impl Board {
    pub(crate) fn part_identifiers(&self) -> Vec<&Identifier> {
        self.identifiers
            .iter()
            .filter(|&i| self.is_adjacent_to_symbol(i))
            .collect()
    }

    pub(crate) fn gear_ratios(&self) -> Vec<u32> {
        self.symbols
            .iter()
            .filter(|&s| s.value == '*')
            .map(|s| self.get_adjacent_identifiers(s))
            .filter(|i| i.len() == 2)
            .map(|gear| gear.iter().map(|x| x.value).product())
            .collect()
    }
    fn get_adjacent_identifiers(&self, symbol: &Symbol) -> Vec<&Identifier> {
        self.identifiers
            .iter()
            .filter(|&i| self.are_adjacent(i, symbol))
            .collect()
    }
    fn is_adjacent_to_symbol(&self, identifier: &Identifier) -> bool {
        self.symbols
            .iter()
            .any(|s| self.are_adjacent(identifier, s))
    }

    fn are_adjacent(&self, identifier: &Identifier, symbol: &Symbol) -> bool {
        if Board::distance(identifier.row, symbol.row) <= 1 {
            for col in identifier.col..identifier.col + identifier.length {
                if Board::distance(col, symbol.col) <= 1 {
                    return true;
                }
            }
        }
        false
    }

    fn distance(a: usize, b: usize) -> i32 {
        (a as i32 - b as i32).abs()
    }
}

struct Symbol {
    value: char,
    row: usize,
    col: usize,
}

struct Identifier {
    value: u32,
    row: usize,
    col: usize,
    length: usize,
}

fn parse(data: &str) -> Board {
    let pattern = Regex::new(r"\d+").unwrap();
    let mut board = vec![];
    let mut identifiers = vec![];
    let mut symbols = vec![];
    for (row, line) in data.lines().enumerate() {
        for m in pattern.find_iter(line) {
            let value = m.as_str().parse().unwrap();
            let col = m.start();
            let length = m.as_str().len();
            identifiers.push(Identifier {
                value,
                row,
                col,
                length,
            })
        }
        for (col, c) in line.chars().enumerate() {
            if !c.is_digit(10) && c != '.' {
                symbols.push(Symbol {
                    value: c,
                    row,
                    col,
                })
            }
        }
    }
    Board { identifiers, symbols }
}

fn part2(board: &Board) -> u32 {
    board.gear_ratios()
        .iter()
        .sum()
}

fn part1(board: &Board) -> u32 {
    board.part_identifiers()
        .iter()
        .map(|i| i.value)
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("3.txt").unwrap();
    let board = parse(&contents);
    println!("{}", part1(&board));
    println!("{}", part2(&board));
}
