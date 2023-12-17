use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn perpendicular(&self) -> (Direction, Direction) {
        match self {
            Direction::Up | Direction::Down => (Direction::Left, Direction::Right),
            Direction::Left | Direction::Right => (Direction::Up, Direction::Down),
        }
    }
}

fn simple_directions(direction: Direction, steps_done: u64) -> Vec<(Direction, u64)> {
    let (turn1, turn2) = direction.perpendicular();
    if steps_done < 3 {
        vec![(direction, steps_done + 1), (turn1, 1), (turn2, 1)]
    } else {
        vec![(turn1, 1), (turn2, 1)]
    }
}

fn ultra_directions(direction: Direction, steps_done: u64) -> Vec<(Direction, u64)> {
    let (turn1, turn2) = direction.perpendicular();
    if steps_done < 4 {
        vec![(direction, steps_done + 1)]
    } else if steps_done < 10 {
        vec![(direction, steps_done + 1), (turn1, 1), (turn2, 1)]
    } else {
        vec![(turn1, 1), (turn2, 1)]
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Key {
    x: i64,
    y: i64,
    last_direction: Direction,
    steps_done: u64,
}

struct Heatmap {
    heatmap: HashMap<(i64, i64), u64>,
}

impl Heatmap {
    fn new(data: &str) -> Self {
        Self {
            heatmap: data.lines()
                .enumerate()
                .map(|(y, line)| line.chars().enumerate()
                    .map(move |(x, c)| ((x as i64, y as i64), c.to_string().as_str().parse().unwrap())))
                .flatten()
                .collect()
        }
    }

    fn height(&self) -> &i64 {
        self.heatmap.keys().map(|(x, y)| y).max().unwrap()
    }
    fn width(&self) -> &i64 {
        self.heatmap.keys().map(|(x, y)| x).max().unwrap()
    }

    fn find_heat_losses(&self, directions: impl Fn(Direction, u64) -> Vec<(Direction, u64)>) -> HashMap<Key, u64> {
        let mut visited = HashMap::new();
        let k1 = Key {
            x: 0,
            y: 0,
            last_direction: Direction::Right,
            steps_done: 0,
        };
        let k2 = Key {
            x: 0,
            y: 0,
            last_direction: Direction::Up,
            steps_done: 0,
        };
        visited.insert(k1, 0);
        visited.insert(k2, 0);
        let mut to_check = vec![k1, k2];
        while !to_check.is_empty() {
            let mut new_to_check = vec![];
            for key in to_check {
                let current_loss = *visited.get(&key).unwrap();
                for (next_direction, next_steps_done) in directions(key.last_direction, key.steps_done) {
                    let (dx, dy) = next_direction.delta();
                    let nx = key.x + dx;
                    let ny = key.y + dy;
                    if self.heatmap.contains_key(&(nx, ny)) {
                        let next_loss = current_loss + self.heatmap.get(&(nx, ny)).unwrap();
                        let next_key = Key {
                            x: nx,
                            y: ny,
                            last_direction: next_direction,
                            steps_done: next_steps_done,
                        };
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
        .filter(|(k, v)| k.x == *heatmap.width() && k.y == *heatmap.height() && k.steps_done >= 4)
        .map(|(k, v)| v)
        .min()
        .unwrap()
}

fn part1(heatmap: &Heatmap) -> u64 {
    *heatmap.find_heat_losses(simple_directions)
        .iter()
        .filter(|(k, v)| k.x == *heatmap.width() && k.y == *heatmap.height())
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