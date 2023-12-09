use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

struct Sequence {
    history: Vec<i64>,
}

impl Sequence {
    fn new(line: &str) -> Self {
        let history = line.split(" ")
            .map(|x| x.parse().unwrap())
            .collect();
        Self {
            history
        }
    }

    fn diffs(&self) -> Self {
        let history = self.history.windows(2)
            .map(|window| window.iter().collect_tuple().unwrap())
            .map(|(a, b)| b - a)
            .collect();
        Sequence { history }
    }

    fn all_equal(&self) -> bool {
        let unique: HashSet<&i64> = self.history.iter().collect();
        unique.len() == 1
    }

    fn predict_prev(&self) -> i64 {
        return if self.all_equal() {
            *self.history.first().unwrap()
        } else {
            self.history.first().unwrap() - self.diffs().predict_prev()
        };
    }

    fn predict_next(&self) -> i64 {
        return if self.all_equal() {
            *self.history.first().unwrap()
        } else {
            self.history.last().unwrap() + self.diffs().predict_next()
        };
    }
}

fn part1(sequences: &Vec<Sequence>) -> i64 {
    sequences
        .iter()
        .map(|s| s.predict_next())
        .sum()
}

fn part2(sequences: &Vec<Sequence>) -> i64 {
    sequences
        .iter()
        .map(|s| s.predict_prev())
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("9.txt").unwrap();
    let sequences = contents.lines()
        .map(|line| Sequence::new(line))
        .collect();
    println!("{}", part1(&sequences));
    println!("{}", part2(&sequences));
}