use std::collections::{HashSet};
use std::fs;
use itertools::Itertools;

#[derive(Eq, PartialEq, Hash)]
struct Position {
    x: u64,
    y: u64,
}

impl Position {
    fn steps(&self, other: &Self) -> (HashSet<u64>, HashSet<u64>) {
        let start_x = self.x.min(other.x);
        let end_x = self.x.max(other.x);
        let start_y = self.y.min(other.y);
        let end_y = self.y.max(other.y);
        ((start_x..end_x).collect(), (start_y..end_y).collect())
    }
}

struct Sky {
    galaxies: HashSet<Position>,
    expanding_x: HashSet<u64>,
    expanding_y: HashSet<u64>,
}

impl Sky {
    fn new(data: &str) -> Self {
        let mut galaxies = HashSet::new();
        for (y, row) in data.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    galaxies.insert(Position { x: x as u64, y: y as u64 });
                }
            }
        }
        let size = data.lines().count() as u64;
        let mut expanding_x = HashSet::new();
        let mut expanding_y = HashSet::new();
        for i in 0..size {
            if (0..size).all(|j| !galaxies.contains(&Position { x: i, y: j })) {
                expanding_x.insert(i);
            }
            if (0..size).all(|j| !galaxies.contains(&Position { x: j, y: i })) {
                expanding_y.insert(i);
            }
        }
        Sky {
            galaxies,
            expanding_x,
            expanding_y,
        }
    }

    fn shortest_paths(&self, expansion_rate: u64) -> Vec<u64> {
        let mut results = vec![];
        for (i, v1) in self.galaxies.iter().enumerate() {
            for (j, v2) in self.galaxies.iter().enumerate() {
                if i < j {
                    let (steps_x, steps_y) = v1.steps(v2);
                    let mut distance = steps_x.iter().count() as u64 + steps_y.iter().count() as u64;
                    distance += self.expanding_x.intersection(&steps_x).count() as u64 * (expansion_rate - 1);
                    distance += self.expanding_y.intersection(&steps_y).count() as u64 * (expansion_rate - 1);
                    results.push(distance);
                }
            }
        }
        results
    }
}

fn part2(sky: &Sky) -> u64 {
    sky.shortest_paths(1000000)
        .iter()
        .sum()
}

fn part1(sky: &Sky) -> u64 {
    sky.shortest_paths(2)
        .iter()
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("11.txt").unwrap();
    let sky = Sky::new(&contents);
    println!("{}", part1(&sky));
    println!("{}", part2(&sky));
}