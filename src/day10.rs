use std::collections::{HashMap};
use std::fs;
use std::ops::Add;
use itertools::{Itertools};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn deltas(&self) -> Coordinates {
        match self {
            Direction::N => Coordinates { x: 0, y: -1 },
            Direction::S => Coordinates { x: 0, y: 1 },
            Direction::E => Coordinates { x: 1, y: 0 },
            Direction::W => Coordinates { x: -1, y: 0 }
        }
    }

    fn next_position(&self, c: &Coordinates) -> Coordinates {
        *c + self.deltas()
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
        }
    }
}

struct Node {
    c: char,
}

impl Node {
    fn move_directions(&self) -> Vec<Direction> {
        match self.c {
            '.' => vec![],
            'S' => vec![],
            '|' => vec![Direction::N, Direction::S],
            '-' => vec![Direction::W, Direction::E],
            'L' => vec![Direction::N, Direction::E],
            'J' => vec![Direction::N, Direction::W],
            '7' => vec![Direction::S, Direction::W],
            'F' => vec![Direction::S, Direction::E],
            _ => panic!()
        }
    }
}

struct Map {
    nodes: HashMap<Coordinates, Node>,
    starting: Coordinates,
}

impl Map {
    fn new(data: &str) -> Self {
        let mut nodes = HashMap::new();
        let mut starting = Coordinates { x: 0, y: 0 };
        for (i, line) in data.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let coordinates = Coordinates { x: j as i64, y: i as i64 };
                if c == 'S' {
                    starting = coordinates
                }
                nodes.insert(coordinates, Node { c });
            }
        }
        Self {
            nodes,
            starting,
        }
    }

    fn reachable_nodes(&self, pos: &Coordinates) -> Vec<Coordinates> {
        self.nodes.get(pos)
            .map(|node| node.move_directions()
                .iter()
                .map(|d| *pos + d.deltas())
                .collect())
            .unwrap_or_default()
    }
    fn find_starting_directions(&self) -> Vec<Direction> {
        let mut starting_directions = vec![];
        for direction in [Direction::N, Direction::S, Direction::E, Direction::W] {
            let neighbour = direction.next_position(&self.starting);
            let x = self.reachable_nodes(&neighbour);
            if x.contains(&self.starting) {
                starting_directions.push(direction);
            }
        }
        starting_directions
    }

    fn get_node(&self, c: &Coordinates) -> &Node {
        self.nodes.get(&c)
            .unwrap()
    }

    fn find_cycle(&self) -> Vec<Coordinates> {
        let starting_directions = self.find_starting_directions();
        let mut next_direction = *starting_directions.get(0).unwrap();
        let mut current = self.starting;
        let mut cycle = vec![self.starting];
        loop {
            current = next_direction.next_position(&current);
            if self.nodes.get(&current).unwrap().c == 'S' {
                break;
            }
            cycle.push(current);
            next_direction = self.get_node(&current)
                .move_directions()
                .iter()
                .filter(|&d| *d != next_direction.opposite())
                .map(|d| *d)
                .find_or_first(|_| true)
                .unwrap();
        }
        cycle
    }

    fn find_enclosed(&self) -> i64 {
        let mut cycle = self.find_cycle();
        cycle.push(self.starting);
        let area = shoelace_area(&cycle);
        // cycle edges are "included" in the area
        area - (cycle.len() / 2) as i64 + 1
    }
}

fn shoelace_area(coords: &Vec<Coordinates>) -> i64 {
    let mut s = 0;
    for pair in coords.windows(2) {
        let (c1, c2) = pair.iter().collect_tuple().unwrap();
        let tmp = (c1.x * c2.y) - (c1.y * c2.x);
        s += tmp
    }
    s.abs() / 2
}

fn part2(map: &Map) -> i64 {
    map.find_enclosed()
}

fn part1(map: &Map) -> u64 {
    map.find_cycle().len() as u64 / 2
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("10.txt").unwrap();
    let map = Map::new(&contents);
    println!("{}", part1(&map));
    println!("{}", part2(&map));
}