use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

impl CubeSet {
    pub fn new(data: &str) -> Self {
        let x = CubeSet::parse_set(data);
        CubeSet {
            red: *x.get("red").unwrap_or_else(|| &0),
            green: *x.get("green").unwrap_or_else(|| &0),
            blue: *x.get("blue").unwrap_or_else(|| &0),
        }
    }
    pub fn is_possible(&self, r: i32, g: i32, b: i32) -> bool {
        self.red <= r && self.green <= g && self.blue <= b
    }

    fn parse_set(data: &str) -> HashMap<String, i32> {
        let mut colors_count = HashMap::new();
        for entry in data.split(", ") {
            let (count, color) = entry.split(" ").collect_tuple().unwrap();
            colors_count.insert(color.to_string(), count.parse().unwrap());
        }
        colors_count
    }
}

struct Game {
    id: i32,
    cubes: Vec<CubeSet>,
}

impl Game {
    pub fn new(line: &str) -> Self {
        let (g, c) = line.split(": ").collect_tuple().unwrap();
        Game {
            id: g[5..].parse().unwrap(),
            cubes: c.split("; ")
                .map(|set| CubeSet::new(set))
                .collect(),
        }
    }
    pub fn is_possible(&self, r: i32, g: i32, b: i32) -> bool {
        self.cubes
            .iter()
            .all(|c| c.is_possible(r, g, b))
    }

    pub fn power(&self) -> i32 {
        self.min_cubes(|c| c.red) *
            self.min_cubes(|c| c.green) *
            self.min_cubes(|c| c.blue)
    }

    fn min_cubes(&self, extractor: impl Fn(&CubeSet) -> i32) -> i32 {
        self.cubes
            .iter()
            .map(extractor)
            .max()
            .unwrap()
    }
}

fn part2(data: &Vec<Game>) -> i32 {
    data.iter()
        .map(|g| g.power())
        .sum()
}

fn part1(data: &Vec<Game>) -> i32 {
    data.iter()
        .filter(|g| g.is_possible(12, 13, 14))
        .map(|g| g.id)
        .sum()
}

fn parse(data: &str) -> Vec<Game> {
    data.lines()
        .map(|line| Game::new(line))
        .collect()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("2.txt").unwrap();
    let games = parse(&contents);
    println!("{}", part1(&games));
    println!("{}", part2(&games));
}