use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
use itertools::Itertools;
use regex::Regex;

struct Game {
    winning: HashSet<u32>,
    ours: HashSet<u32>,
}

impl Game {
    fn new(line: &str) -> Self {
        let (_, game) = line.split(": ").collect_tuple().unwrap();
        let (winning, ours) = game.split(" | ")
            .map(|data| Game::extract_ints(data))
            .collect_tuple()
            .unwrap();
        Game {
            winning,
            ours,
        }
    }

    pub fn score(&self) -> u32 {
        let m = self.matching();
        if m > 0 {
            (2u32).pow(m - 1)
        } else {
            0
        }
    }

    fn matching(&self) -> u32 {
        self.winning.intersection(&self.ours).count() as u32
    }

    fn extract_ints(data: &str) -> HashSet<u32> {
        let pattern = Regex::new(r"\d+").unwrap();
        pattern.find_iter(data)
            .map(|v| v.as_str().parse().unwrap())
            .collect()
    }
}

fn part2(games: &Vec<Game>) -> u32 {
    let mut multipliers: HashMap<usize, u32> = HashMap::new();
    for (index, game) in games.iter().enumerate() {
        if !multipliers.contains_key(&index) {
            multipliers.insert(index, 1);
        }
        let multiplier = multipliers.get(&index).unwrap().clone();
        let matched = game.matching();
        let cards_left = (matched as usize).min(games.len() - index);
        for next_card in 0..cards_left {
            let next_index = index + next_card + 1;
            let next_multiplier = multipliers.get(&next_index).unwrap_or(&1) + multiplier;
            multipliers.insert(next_index, next_multiplier);
        }
    }
    multipliers.values()
        .sum()
}

fn part1(games: &Vec<Game>) -> u32 {
    games.iter()
        .map(|g| g.score())
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("4.txt").unwrap();
    let games = contents
        .lines()
        .map(|line| Game::new(line))
        .collect();
    println!("{}", part1(&games));
    println!("{}", part2(&games));
}
