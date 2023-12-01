use std::fs;
use regex::Regex;

fn convert_number(value: &str) -> Option<&str> {
    let mapping: Vec<(&str, &str)> = vec![
        ("1", "1"), ("2", "2"), ("3", "3"), ("4", "4"), ("5", "5"), ("6", "6"), ("7", "7"), ("8", "8"), ("9", "9"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    for (k, v) in mapping {
        if value.starts_with(k) {
            return Some(v);
        }
    }
    None
}

fn parse2(line: &str) -> Vec<String> {
    (0..line.len())
        .map(|start_index| convert_number(&line[start_index..]))
        .filter(|maybe_number| maybe_number.is_some())
        .map(|number| number.unwrap().to_string())
        .collect()
}

fn parse1(line: &str) -> Vec<String> {
    let pattern = Regex::new(r"\d").unwrap();
    pattern.find_iter(line)
        .map(|m| m.as_str().to_string())
        .collect()
}

fn f(data: &str, parse_line: impl Fn(&str) -> Vec<String>) -> i32 {
    data.lines()
        .map(|x| parse_line(x))
        .map(|line_numbers| line_numbers.first().unwrap().to_string() + &line_numbers.last().unwrap())
        .map(|x| x.parse::<i32>().unwrap())
        .sum()
}

fn part2(data: &str) -> i32 {
    f(data, parse2)
}

fn part1(data: &str) -> i32 {
    f(data, parse1)
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("1.txt").unwrap();
    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}