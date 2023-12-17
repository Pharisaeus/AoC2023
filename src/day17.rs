use std::collections::HashMap;
use std::fs;
use std::ops::Add;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> Position {
        match self {
            Direction::Up => Position { x: 0, y: 1 },
            Direction::Down => Position { x: 0, y: -1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }

    fn perpendicular(&self) -> (Direction, Direction) {
        match self {
            Direction::Up | Direction::Down => (Direction::Left, Direction::Right),
            Direction::Left | Direction::Right => (Direction::Up, Direction::Down),
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

impl Add<&Direction> for Position {
    type Output = Self;

    fn add(self, rhs: &Direction) -> Self::Output {
        self + rhs.delta()
    }
}

fn simple_directions(state: &State) -> Vec<State> {
    let (turn1, turn2) = state.last_direction.perpendicular();
    if state.steps_done < 3 {
        vec![state.step(&state.last_direction), state.step(&turn1), state.step(&turn2)]
    } else {
        vec![state.step(&turn1), state.step(&turn2)]
    }
}

fn ultra_directions(state: &State) -> Vec<State> {
    let (turn1, turn2) = state.last_direction.perpendicular();
    if state.steps_done < 4 {
        vec![state.step(&state.last_direction)]
    } else if state.steps_done < 10 {
        vec![state.step(&state.last_direction), state.step(&turn1), state.step(&turn2)]
    } else {
        vec![state.step(&turn1), state.step(&turn2)]
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    position: Position,
    last_direction: Direction,
    steps_done: u64,
}

impl State {
    fn step(&self, direction: &Direction) -> Self {
        Self {
            position: self.position + direction,
            last_direction: *direction,
            steps_done: if self.last_direction.eq(direction) {
                self.steps_done + 1
            } else {
                1
            },
        }
    }
}

struct Heatmap {
    heatmap: HashMap<Position, u64>,
}

impl Heatmap {
    fn new(data: &str) -> Self {
        Self {
            heatmap: data.lines()
                .enumerate()
                .map(|(y, line)| line.chars().enumerate()
                    .map(move |(x, c)| (Position { x: x as i64, y: y as i64 }, c.to_string().as_str().parse().unwrap())))
                .flatten()
                .collect()
        }
    }

    fn height(&self) -> i64 {
        self.heatmap.keys().map(|c| c.y).max().unwrap()
    }
    fn width(&self) -> i64 {
        self.heatmap.keys().map(|(c)| c.x).max().unwrap()
    }

    fn find_heat_losses(&self, directions: impl Fn(&State) -> Vec<State>) -> HashMap<State, u64> {
        let mut visited = HashMap::new();
        let s1 = State { position: Position { x: 0, y: 0 }, last_direction: Direction::Right, steps_done: 0 };
        let s2 = State { position: Position { x: 0, y: 0 }, last_direction: Direction::Up, steps_done: 0 };
        visited.insert(s1, 0);
        visited.insert(s2, 0);
        let mut to_check = vec![s1, s2];
        while !to_check.is_empty() {
            let mut new_to_check = vec![];
            for state in to_check {
                let current_loss = *visited.get(&state).unwrap();
                for next_key in directions(&state) {
                    if self.heatmap.contains_key(&next_key.position) {
                        let next_loss = current_loss + self.heatmap.get(&next_key.position).unwrap();
                        if !visited.contains_key(&next_key) || *visited.get(&next_key).unwrap() > next_loss {
                            visited.insert(next_key, next_loss);
                            new_to_check.push(next_key);
                        }
                    }
                }
            }
            to_check = new_to_check;
        }
        visited
    }
}

fn part2(heatmap: &Heatmap) -> u64 {
    *heatmap.find_heat_losses(ultra_directions)
        .iter()
        .filter(|(k, v)| k.position.x == heatmap.width() && k.position.y == heatmap.height() && k.steps_done >= 4)
        .map(|(k, v)| v)
        .min()
        .unwrap()
}

fn part1(heatmap: &Heatmap) -> u64 {
    *heatmap.find_heat_losses(simple_directions)
        .iter()
        .filter(|(k, v)| k.position.x == heatmap.width() && k.position.y == heatmap.height())
        .map(|(k, v)| v)
        .min()
        .unwrap()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("17.txt").unwrap();
    let heatmap = Heatmap::new(&contents);
    println!("{}", part1(&heatmap));
    println!("{}", part2(&heatmap));
}