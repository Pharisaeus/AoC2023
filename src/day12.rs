use std::collections::HashMap;
use std::fs;
use std::io::Read;
use itertools::Itertools;
use crate::day12::Spring::{BROKEN, OPERATIONAL, UNKNOWN};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Spring {
    OPERATIONAL,
    BROKEN,
    UNKNOWN,
}

impl Spring {
    fn new(c: &char) -> Self {
        match c {
            '#' => OPERATIONAL,
            '.' => BROKEN,
            '?' => UNKNOWN,
            &_ => panic!()
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct State {
    current_counts: Vec<u8>,
    current_group_size: u8,
    position: usize,
}

struct Row {
    springs: Vec<Spring>,
    counts: Vec<u8>,
}

impl Row {
    fn new(line: &str) -> Self {
        let (map, numbers) = line.split(" ").collect_tuple().unwrap();
        let springs = map.chars()
            .map(|c| Spring::new(&c))
            .collect();
        let counts = numbers.split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        Self {
            springs,
            counts,
        }
    }

    fn closed_group(&self, current_counts: &Vec<u8>, current_group_size: u8) -> Vec<u8> {
        if current_group_size > 0 {
            [current_counts.clone(), vec![current_group_size]].concat()
        } else {
            current_counts.clone()
        }
    }
    fn verify(&self, current_counts: &Vec<u8>, current_group_size: u8) -> bool {
        self.counts.eq(&self.closed_group(current_counts, current_group_size))
    }

    fn count_arrangements(&self) -> u64 {
        self.caching_arrangements(&vec![], 0, 0, &mut HashMap::new())
    }

    fn caching_arrangements(&self, current_counts: &Vec<u8>, current_group_size: u8, position: usize, known: &mut HashMap<State, u64>) -> u64 {
        let current_state = State { current_counts: current_counts.clone(), current_group_size: current_group_size.clone(), position };
        return if known.contains_key(&current_state) {
            *(known.get(&current_state).unwrap())
        } else {
            let arr = self.arrangements(current_counts, current_group_size, position, known);
            known.insert(current_state, arr);
            arr
        };
    }

    fn arrangements(&self, current_counts: &Vec<u8>, current_group_size: u8, position: usize, known: &mut HashMap<State, u64>) -> u64 {
        return if self.counts.len() < current_counts.len() { // too many groups already
            0
        } else if !&self.counts[0..current_counts.len()].eq(current_counts.as_slice()) { // prefix doesn't match
            0
        } else if position == self.springs.len() { // fully processed
            if self.verify(current_counts, current_group_size) {
                1
            } else {
                0
            }
        } else {
            let current = self.springs.get(position).unwrap();
            if current == &BROKEN {
                let closed_counts = self.closed_group(current_counts, current_group_size);
                self.caching_arrangements(&closed_counts, 0, position + 1, known)
            } else if current == &OPERATIONAL {
                self.caching_arrangements(current_counts, current_group_size + 1, position + 1, known)
            } else {// wildcard
                // put operational
                let mut sub_arrangements = self.caching_arrangements(current_counts, current_group_size + 1, position + 1, known);
                // put broken
                let closed_counts = self.closed_group(current_counts, current_group_size);
                sub_arrangements += self.caching_arrangements(&closed_counts, 0, position + 1, known);
                sub_arrangements
            }
        };
    }

    fn expand(&self) -> Row {
        let mut expanded_springs = vec![];
        let mut expanded_counts = vec![];
        for i in 0..5 {
            for s in &self.springs {
                expanded_springs.push(*s)
            }
            if i < 4 {
                expanded_springs.push(UNKNOWN);
            }
            for c in &self.counts {
                expanded_counts.push(*c)
            }
        }
        Row {
            springs: expanded_springs,
            counts: expanded_counts,
        }
    }
}

fn part2(rows: &Vec<Row>) -> u64 {
    rows.iter()
        .map(Row::expand)
        .map(|r| r.count_arrangements())
        .sum()
}

fn part1(rows: &Vec<Row>) -> u64 {
    rows.iter()
        .map(Row::count_arrangements)
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("12.txt").unwrap();
    let rows = contents.lines()
        .map(Row::new)
        .collect();
    println!("{}", part1(&rows));
    println!("{}", part2(&rows));
}