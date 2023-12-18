use std::fs;
use std::ops::Add;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(d: &str) -> Self {
        match d {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            &_ => panic!()
        }
    }

    fn new_from_color(c: &char) -> Self {
        match c {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            &_ => panic!()
        }
    }

    fn delta(&self, multiplier: i64) -> Position {
        match self {
            Direction::Up => Position { x: 0, y: multiplier },
            Direction::Down => Position { x: 0, y: -multiplier },
            Direction::Left => Position { x: -multiplier, y: 0 },
            Direction::Right => Position { x: multiplier, y: 0 },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&DigStep> for Position {
    type Output = Self;

    fn add(self, rhs: &DigStep) -> Self::Output {
        self + rhs.direction.delta(rhs.steps)
    }
}

struct DigStep {
    direction: Direction,
    steps: i64,
    color: String,
}

impl DigStep {
    fn new(line: &str) -> Self {
        let (direction, steps, color) = line.split(" ").collect_tuple().unwrap();
        Self {
            direction: Direction::new(direction),
            steps: steps.parse().unwrap(),
            color: color.to_string(),
        }
    }

    fn color_step(&self) -> Self {
        let stripped = self.color.replace("(", "")
            .replace(")", "")
            .replace("#", "");
        Self {
            direction: Direction::new_from_color(&(stripped.as_str().as_bytes()[stripped.len() - 1] as char)),
            steps: i64::from_str_radix(&stripped.as_str()[..stripped.len() - 1], 16).unwrap(),
            color: "".to_string(),
        }
    }
}

struct DigPlan {
    plan: Vec<DigStep>,
}

impl DigPlan {
    fn new(data: &str) -> Self {
        Self {
            plan: data.lines().map(|line| DigStep::new(line)).collect()
        }
    }

    fn area(&self, steps: &Vec<DigStep>) -> i64 {
        let mut coords = vec![];
        let mut current = Position { x: 0, y: 0 };
        coords.push(current);
        for step in steps {
            current = current + step;
            coords.push(current);
        }
        let internal_area = shoelace_area(&coords);
        let edge_area = (steps.iter().map(|s| s.steps).sum::<i64>()) / 2 + 1;
        internal_area + edge_area
    }

    fn simple_area(&self) -> i64 {
        self.area(&self.plan)
    }
    fn color_area(&self) -> i64 {
        self.area(&self.plan.iter().map(|s| s.color_step()).collect())
    }
}

fn shoelace_area(coords: &Vec<Position>) -> i64 {
    let mut s = 0;
    for pair in coords.windows(2) {
        let (c1, c2) = pair.iter().collect_tuple().unwrap();
        let tmp = (c1.x * c2.y) - (c1.y * c2.x);
        s += tmp
    }
    s.abs() / 2
}

fn part1(plan: &DigPlan) -> i64 {
    plan.simple_area()
}

fn part2(plan: &DigPlan) -> i64 {
    plan.color_area()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("18.txt").unwrap();
    let dig_plan = DigPlan::new(&contents);
    println!("{}", part1(&dig_plan));
    println!("{}", part2(&dig_plan));
}