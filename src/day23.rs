use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Add;
use itertools::Itertools;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn delta(&self) -> Coord {
        match self {
            Direction::N => Coord { x: 0, y: -1 },
            Direction::S => Coord { x: 0, y: 1 },
            Direction::E => Coord { x: 1, y: 0 },
            Direction::W => Coord { x: -1, y: 0 },
        }
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Direction> for Coord {
    type Output = Self;

    fn add(self, rhs: &Direction) -> Self::Output {
        self + rhs.delta()
    }
}

#[derive(Eq, PartialEq)]
enum TileType {
    Path,
    Forest,
    NorthSlope,
    SouthSlope,
    EastSlope,
    WestSlope,
}

impl TileType {
    fn new(c: &char) -> Self {
        match c {
            '.' => TileType::Path,
            '#' => TileType::Forest,
            '^' => TileType::NorthSlope,
            'v' => TileType::SouthSlope,
            '>' => TileType::EastSlope,
            '<' => TileType::WestSlope,
            &_ => panic!()
        }
    }

    fn possible_directions(&self) -> Vec<Direction> {
        match self {
            TileType::Path => vec![Direction::N, Direction::S, Direction::E, Direction::W],
            TileType::Forest => vec![],
            TileType::NorthSlope => vec![Direction::N],
            TileType::SouthSlope => vec![Direction::S],
            TileType::EastSlope => vec![Direction::E],
            TileType::WestSlope => vec![Direction::W],
        }
    }
}

struct Map {
    tiles: HashMap<Coord, TileType>,
}

impl Map {
    fn new(data: &str) -> Self {
        Self {
            tiles: data.lines()
                .enumerate()
                .map(|(y, line)| line.chars()
                    .enumerate()
                    .map(move |(x, c)| (Coord { x: x as i64, y: y as i64 }, TileType::new(&c)))
                ).flatten()
                .collect()
        }
    }

    fn next_positions(&self, position: &Coord) -> Vec<Coord> {
        let tile = self.tiles.get(position).unwrap();
        tile.possible_directions()
            .iter()
            .map(|d| *position + d)
            .filter(|new_pos| self.tiles.get(new_pos)
                .filter(|&t| t != &TileType::Forest).is_some())
            .collect()
    }

    fn max_x(&self) -> i64 {
        self.tiles.keys().map(|c| c.x).max().unwrap()
    }

    fn max_y(&self) -> i64 {
        self.tiles.keys().map(|c| c.y).max().unwrap()
    }

    fn junctions(&self) -> HashSet<Coord> {
        self.tiles.iter()
            .filter(|(_, t)| t != &&TileType::Forest)
            .map(|(&pos, _)| pos)
            .filter(|pos| self.next_positions(pos).len() > 2)
            .collect()
    }

    fn compress_graph(&self) -> CompressedGraph {
        let mut junctions = self.junctions();
        let start = Coord { x: 1, y: 0 };
        junctions.insert(start);
        let end = Coord { x: self.max_x() - 1, y: self.max_y() };
        junctions.insert(end);
        let mut edges = HashMap::new();
        for junction in &junctions {
            let mut reachable = HashMap::new();
            for neighbour in self.next_positions(&junction) { // each of those is either a junction or has just 1 neighbour
                let mut steps = 1;
                let mut current = neighbour;
                let mut seen = HashSet::new();
                seen.insert(*junction);
                loop {
                    seen.insert(current);
                    if junctions.contains(&current) {
                        reachable.insert(current, steps);
                        break;
                    } else {
                        let next_positions = self.next_positions(&current);
                        let next = next_positions.iter().filter(|x| !seen.contains(x)).last();
                        if next.is_some() {
                            current = next.unwrap().clone();
                            steps += 1;
                        } else { // reached dead end
                            break;
                        }
                    }
                }
            }
            edges.insert(*junction, reachable);
        }
        CompressedGraph {
            edges,
            start,
            end,
        }
    }
}

struct CompressedGraph {
    edges: HashMap<Coord, HashMap<Coord, usize>>,
    start: Coord,
    end: Coord,
}

impl CompressedGraph {
    fn longest(&self) -> usize {
        let seen = HashSet::new();
        self.longest_rec(self.start, &seen, 0)
    }

    fn longest_rec(&self, current: Coord, seen: &HashSet<Coord>, current_distance: usize) -> usize {
        if current == self.end {
            return current_distance;
        }
        let mut current_seen = seen.clone();
        current_seen.insert(current);
        let neighbours = self.edges.get(&current).unwrap();
        neighbours
            .iter()
            .filter(|(next, dist)| !current_seen.contains(next))
            .map(|(next, dist)| self.longest_rec(*next, &current_seen, current_distance + dist))
            .max()
            .unwrap_or(0)
    }
}

fn part2(contents: &str) -> usize {
    let no_slope = contents.replace(">", ".")
        .replace("<", ".")
        .replace("v", ".")
        .replace("^", ".");
    let map = Map::new(&no_slope);
    let graph = map.compress_graph();
    graph.longest()
}

fn part1(contents: &str) -> usize {
    let map = Map::new(&contents);
    let graph = map.compress_graph();
    graph.longest()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("23.txt").unwrap();
    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}