use std::fmt::format;
use std::fs;
use std::ops::Add;
use itertools::Itertools;
use z3::{ast, Config, Context, SatResult, Solver};

#[derive(Copy, Clone)]
struct Coord {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

struct Hailstone {
    start: Coord,
    velocity: Coord,
}

impl Hailstone {
    fn is_in_future(&self, intersection_point: &Coord) -> bool {
        if self.velocity.x > 0f64 {
            self.start.x <= intersection_point.x
        } else {
            self.start.x >= intersection_point.x
        }
    }
}

impl Hailstone {
    fn new(line: &str) -> Hailstone {
        let (start, velocity) = line.split(" @ ")
            .map(|c| c.split(", ").map(|x| x.trim().parse().unwrap()).collect_tuple().unwrap())
            .map(|(x, y, z)| Coord { x, y, z })
            .collect_tuple()
            .unwrap();
        Self {
            start,
            velocity,
        }
    }
}

struct Line2d {
    a: f64,
    b: f64,
}

impl Line2d {
    fn from_hailstone(blizzard: &Hailstone) -> Self {
        let next = blizzard.start + blizzard.velocity;
        let a = (next.y - blizzard.start.y) / (next.x - blizzard.start.x);
        let b = blizzard.start.y - a * blizzard.start.x;
        Self {
            a,
            b,
        }
    }

    fn intersection(&self, other: &Line2d) -> Option<Coord> {
        return if self.a == other.a {
            None
        } else {
            let x = (other.b - self.b) / (self.a - other.a);
            let y = self.a * x + self.b;
            Some(Coord {
                x,
                y,
                z: 0.0,
            })
        };
    }
}

struct Blizzard {
    hailstones: Vec<Hailstone>,
}

impl Blizzard {
    fn new(data: &str) -> Self {
        Self {
            hailstones: data.lines()
                .map(|line| Hailstone::new(line))
                .collect()
        }
    }

    fn intersection2d_points(&self) -> Vec<Coord> {
        let mut res = vec![];
        let mut skip = 0;
        for first in &self.hailstones {
            let first_line = Line2d::from_hailstone(first);
            skip += 1;
            for second in self.hailstones.iter().skip(skip) {
                let second_line = Line2d::from_hailstone(second);
                match first_line.intersection(&second_line) {
                    None => {}
                    Some(intersection) => {
                        if first.is_in_future(&intersection) && second.is_in_future(&intersection) {
                            res.push(intersection)
                        }
                    }
                }
            }
        }
        res
    }
}

fn part1(blizzard: &Blizzard) -> usize {
    let small = 200000000000000f64;
    let big = 400000000000000f64;
    blizzard.intersection2d_points()
        .iter()
        .filter(|point| point.x >= small && point.x <= big && point.y >= small && point.y <= big)
        .count()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("24.txt").unwrap();
    let blizzard = Blizzard::new(&contents);
    println!("{}", part1(&blizzard));
}