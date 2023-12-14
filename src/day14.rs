use std::collections::{HashMap};
use std::fs;
use std::ops::Add;
use itertools::Itertools;
use crate::day14::Rock::{Cube, Empty, Rounded};

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
enum Rock {
    Rounded,
    Cube,
    Empty,
}

impl Rock {
    fn new(c: &char) -> Self {
        match c {
            '.' => Empty,
            '#' => Cube,
            'O' => Rounded,
            &_ => panic!()
        }
    }
}

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn in_bounds(&self, max_x: usize, max_y: usize) -> bool {
        self.x >= 0 && self.x < max_x as i64 && self.y >= 0 && self.y < max_y as i64
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn vector(&self) -> Coord {
        match self {
            Direction::N => Coord { x: 0, y: -1 },
            Direction::S => Coord { x: 0, y: 1 },
            Direction::E => Coord { x: 1, y: 0 },
            Direction::W => Coord { x: -1, y: 0 },
        }
    }

    fn ranges(&self, max_x: usize, max_y: usize) -> Vec<Coord> {
        match self {
            Direction::N => (0..max_y).map(|y| (0..max_x).map(move |x| Coord { x: x as i64, y: y as i64 })).flatten().collect(),
            Direction::S => (0..max_y).rev().map(|y| (0..max_x).map(move |x| Coord { x: x as i64, y: y as i64 })).flatten().collect(),
            Direction::E => (0..max_x).map(|x| (0..max_y).map(move |y| Coord { x: x as i64, y: y as i64 })).flatten().collect(),
            Direction::W => (0..max_x).rev().map(|x| (0..max_y).map(move |y| Coord { x: x as i64, y: y as i64 })).flatten().collect(),
        }
    }
}

struct Platform {
    rocks: HashMap<Coord, Rock>,
    height: usize,
    width: usize,
}

impl Platform {
    fn new(data: &str) -> Self {
        let mut rocks = HashMap::new();
        let mut height = data.lines().count();
        let mut width = 0;
        for (y, line) in data.lines().enumerate() {
            width = line.len();
            for (x, c) in line.chars().enumerate() {
                rocks.insert(Coord { x: x as i64, y: y as i64 }, Rock::new(&c));
            }
        }
        Platform {
            rocks,
            width,
            height,
        }
    }

    fn tilt_step(&self, direction: &Direction) -> (bool, Self) {
        let mut new_rocks = self.rocks.clone();
        let mut modified = false;
        for pos in direction.ranges(self.width, self.height) {
            let &current_rock = new_rocks.get(&pos).unwrap();
            match current_rock {
                Rounded => {
                    let new_pos = pos + direction.vector();
                    if new_pos.in_bounds(self.width, self.height) {
                        let destination_rock = new_rocks.get(&new_pos).unwrap();
                        match destination_rock {
                            Rounded => {}
                            Cube => {}
                            Empty => {
                                new_rocks.insert(new_pos, current_rock.clone());
                                new_rocks.insert(pos, Empty);
                                modified = true;
                            }
                        }
                    }
                }
                Cube => {}
                Empty => {}
            }
        }
        (modified, Platform {
            rocks: new_rocks,
            width: self.width,
            height: self.height,
        })
    }

    fn tilt_far(&self, direction: &Direction) -> Self {
        let (_, mut current) = self.tilt_step(direction);
        loop {
            let (modified, new_one) = current.tilt_step(direction);
            current = new_one;
            if !modified {
                return current;
            }
        }
    }

    fn tilt_cycle(&self) -> Self {
        let mut current = self.tilt_far(&Direction::N);
        current = current.tilt_far(&Direction::W);
        current = current.tilt_far(&Direction::S);
        current = current.tilt_far(&Direction::E);
        current
    }

    fn hashable(&self) -> Vec<(Coord, Rock)> {
        self.rocks.iter().map(|(&c, &r)| (c, r)).sorted().collect()
    }

    fn tilt_cycles(&self, steps: usize) -> Self {
        let mut history = vec![];
        let mut known = HashMap::new();
        let mut current = self.tilt_cycle();
        history.push(current.hashable());
        known.insert(current.hashable(), 0);
        let mut cycle_counter = 1;
        loop {
            current = current.tilt_cycle();
            if known.contains_key(&current.hashable()) {
                break;
            }
            history.push(current.hashable());
            known.insert(current.hashable(), cycle_counter);
            cycle_counter += 1;
        }
        let cycle_start = known.get(&current.hashable()).unwrap();
        let cycle_length = cycle_counter - cycle_start;
        let offset = (steps - cycle_start) % cycle_length;
        Self {
            rocks: history[offset + cycle_start - 1].iter().map(|(c, r)| (*c, *r)).collect(),
            height: self.height,
            width: self.width,
        }
    }

    fn north_support(&self) -> usize {
        let mut sum = 0;
        for (coord, rock) in &self.rocks {
            match rock {
                Rounded => sum += self.height - coord.y as usize,
                Cube => {}
                Empty => {}
            }
        }
        sum
    }
}

fn part2(platform: &Platform) -> usize {
    platform.tilt_cycles(1000000000)
        .north_support()
}

fn part1(platform: &Platform) -> usize {
    platform.tilt_far(&Direction::N)
        .north_support()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("14.txt").unwrap();
    let platform = Platform::new(&contents);
    println!("{}", part1(&platform));
    println!("{}", part2(&platform));
}