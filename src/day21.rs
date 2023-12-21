use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Add;
use std::sync::atomic::Ordering::SeqCst;
use itertools::Itertools;
use crate::day21::TileType::{Ground, Rock};

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
            Direction::N => Coord { x: 0, y: 1 },
            Direction::S => Coord { x: 0, y: -1 },
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
    Ground,
    Rock,
}

impl TileType {
    fn new(c: &char) -> Self {
        match c {
            '.' => Ground,
            '#' => Rock,
            'S' => Ground,
            &_ => panic!()
        }
    }
}

struct Garden {
    tiles: HashMap<Coord, TileType>,
    start: Coord,
    bound_x: i64,
    bound_y: i64,
}

impl Garden {
    fn new(data: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut start = Coord { x: 0, y: 0 };
        let bound_y = data.lines().count() as i64;
        let mut bound_x = 0;
        for (y, row) in data.lines().enumerate() {
            bound_x = row.len() as i64;
            for (x, c) in row.chars().enumerate() {
                tiles.insert(Coord { x: x as i64, y: y as i64 }, TileType::new(&c));
                if c == 'S' {
                    start = Coord { x: x as i64, y: y as i64 }
                }
            }
        }
        Self {
            tiles,
            start,
            bound_x,
            bound_y,
        }
    }

    fn is_legal_expanded(&self, position: &Coord) -> bool {
        let wrapped_position = Coord { x: position.x.rem_euclid(self.bound_x), y: position.y.rem_euclid(self.bound_y) };
        self.tiles.get(&wrapped_position).filter(|&t| t == &Ground).is_some()
    }

    fn count_positions(&self, steps: usize) -> i64 {
        let mut to_check = HashSet::new();
        to_check.insert(self.start);
        for _ in 0..steps {
            let mut new_to_check = HashSet::new();
            for c in to_check {
                for direction in [Direction::N, Direction::S, Direction::E, Direction::W] {
                    let next_position = c + &direction;
                    if self.is_legal_expanded(&next_position) {
                        new_to_check.insert(next_position);
                    }
                }
            }
            to_check = new_to_check;
        }
        to_check.len() as i64
    }
}

// stolen from day9
struct Sequence {
    history: Vec<i64>,
}

impl Sequence {
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
    fn predict_next(&self) -> i64 {
        return if self.all_equal() {
            *self.history.first().unwrap()
        } else {
            self.history.last().unwrap() + self.diffs().predict_next()
        };
    }
}

fn part2_slow(garden: &Garden) -> i64 {
    // we go into 2 orthogonal directions so the value has to grow with some ^2
    // it takes 65 steps to reach boundary
    // we need value at k*131 + 65
    // simulate first 3 values and then interpolate
    let mut polynomial_values = vec![];
    for i in 0..=2 {
        let x = 65 + i * 131;
        let y = garden.count_positions(x);
        polynomial_values.push(y);
    }
    let steps = (26501365 - 65) / 131;
    while polynomial_values.len() < steps {
        let next_y = Sequence { history: polynomial_values.clone() }.predict_next();
        polynomial_values.push(next_y);
    }
    Sequence { history: polynomial_values.clone() }.predict_next()
}

fn part2_fast(garden: &Garden) -> i64 {
    let mut polynomial_values = vec![];
    for i in 0..=2 {
        let x = 65 + i * 131;
        let y = garden.count_positions(x);
        polynomial_values.push(y);
    }
    let xs: Vec<_> = (0..=2).map(|i| (65 + i * 131) as f64).collect();
    let ys: Vec<_> = polynomial_values.iter().map(|&y| y as f64).collect();
    let poly = bacon_sci::interp::lagrange(&xs, &ys, 1e-1).unwrap();
    poly.evaluate(26501365.0).ceil() as i64
}

fn part1(garden: &Garden) -> i64 {
    garden.count_positions(64)
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("21.txt").unwrap();
    let garden = Garden::new(&contents);
    println!("{}", part1(&garden));
    // println!("{}", part2_slow(&garden));
    println!("{}", part2_fast(&garden));
}