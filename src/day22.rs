use std::arch::x86_64::__cpuid_count;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::ops::Add;
use itertools::Itertools;

#[derive(Eq, PartialEq, Copy, Clone)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn new(entry: &str) -> Self {
        let (x, y, z) = entry.split(",")
            .map(|x| x.parse()
                .unwrap())
            .collect_tuple()
            .unwrap();
        Self {
            x,
            y,
            z,
        }
    }
}

impl Add for &Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Copy, Clone)]
struct Block {
    label: usize,
    start: Coord,
    end: Coord,
}

impl Block {
    fn new(line: &str, label: usize) -> Self {
        let (s, e) = line.split("~").collect_tuple().unwrap();
        Self {
            label,
            start: Coord::new(&s),
            end: Coord::new(&e),
        }
    }

    fn move_down(&self) -> Self {
        let down_vector = Coord { x: 0, y: 0, z: -1 };
        let start = &self.start + &down_vector;
        let end = &self.end + &down_vector;
        Self {
            label: self.label,
            start,
            end,
        }
    }

    fn min_x(&self) -> i64 {
        self.start.x.min(self.end.x)
    }

    fn max_x(&self) -> i64 {
        self.start.x.max(self.end.x)
    }
    fn min_y(&self) -> i64 {
        self.start.y.min(self.end.y)
    }

    fn max_y(&self) -> i64 {
        self.start.y.max(self.end.y)
    }

    fn min_z(&self) -> i64 {
        self.start.z.min(self.end.z)
    }

    fn max_z(&self) -> i64 {
        self.start.z.max(self.end.z)
    }

    fn is_colliding(&self, other: &Block) -> bool {
        self.min_x() <= other.max_x()
            && self.max_x() >= other.min_x()
            && self.min_y() <= other.max_y()
            && self.max_y() >= other.min_y()
            && self.min_z() <= other.max_z()
            && self.max_z() >= other.min_z()
    }

    fn out_of_bounds(&self) -> bool {
        self.min_z() < 1
    }

    fn has_moved(&self, other: &Block) -> bool {
        self.label == other.label && (self.start != other.start || self.end != other.end)
    }
}

struct Board {
    blocks: Vec<Block>,
}

impl Board {
    fn new(data: &str) -> Self {
        let mut blocks: Vec<Block> = data
            .lines()
            .enumerate()
            .map(|(i, line)| Block::new(line, i))
            .collect();
        blocks.sort_by_key(|b| b.min_z());
        Self {
            blocks
        }
    }

    fn find_collisions(&self, block: &Block, skip: usize) -> Vec<usize> {
        self.blocks
            .iter()
            .enumerate()
            .filter(|&(i, b)| i != skip && b.is_colliding(block))
            .map(|(i, b)| i)
            .collect()
    }

    fn settle_step(&mut self) -> bool {
        let mut new_blocks = vec![];
        let mut modified = false;
        for (i, b) in self.blocks.iter().enumerate() {
            let moved = b.move_down();
            if moved.out_of_bounds() || self.find_collisions(&moved, i).len() > 0 {
                new_blocks.push(b.clone())
            } else {
                new_blocks.push(moved);
                modified = true;
            }
        }
        if modified {
            self.blocks = new_blocks;
        }
        modified
    }

    fn settle_board(&mut self) {
        loop {
            let modified = self.settle_step();
            if !modified {
                break;
            }
        }
    }
}

fn part2_slow(board: &Board) -> usize {
    let mut counter = 0;
    for (i, b) in board.blocks.iter().enumerate() {
        let other_blocks = board.blocks.iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(i, b)| b.clone())
            .collect();
        let mut board_without_block = Board {
            blocks: other_blocks
        };
        board_without_block.settle_board();
        for x in &board_without_block.blocks {
            for y in &board.blocks {
                if x.has_moved(y) {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn part2_fast(board: &Board) -> usize {
    // a->[b,c] block a is directly supported by b and c
    let mut is_supported: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (i, b) in board.blocks.iter().enumerate() {
        let moved = b.move_down();
        let collisions = board.find_collisions(&moved, i);
        is_supported.insert(i, HashSet::from_iter(collisions));
    }
    // a->[b,c] block a is directly supporting b and c
    let mut is_supporting = HashMap::new();
    for (i, _b) in board.blocks.iter().enumerate() {
        let supports_of_current = is_supported.get(&i).unwrap();
        for parent in supports_of_current {
            if !is_supporting.contains_key(parent) {
                is_supporting.insert(parent, vec![]);
            }
            let children = is_supporting.get_mut(parent).unwrap();
            children.push(i);
        }
    }
    let mut counter = 0;
    for (i, _b) in board.blocks.iter().enumerate() {
        let mut moved = HashSet::new();
        moved.insert(i);
        let children = is_supporting.get(&i);
        if children.is_some() {
            let children = children.unwrap();
            let mut to_check: VecDeque<usize> = children.iter().cloned().collect();
            while !to_check.is_empty() {
                let child = to_check.pop_front().unwrap();
                let child_parents = is_supported.get(&child).unwrap();
                if !moved.contains(&child) && child_parents.difference(&moved).count() == 0 { // all parents moved
                    moved.insert(child);
                    let next_level = is_supporting.get(&child); // check children of moved block
                    if next_level.is_some() {
                        to_check.extend(next_level.unwrap())
                    }
                }
            }
            counter += moved.len() - 1
        }
    }
    counter
}

fn part1(board: &Board) -> usize {
    let mut supports: HashSet<usize> = (0..board.blocks.len()).collect();
    for (i, b) in board.blocks.iter().enumerate() {
        let moved = b.move_down();
        let collisions = board.find_collisions(&moved, i);
        if collisions.len() == 1 {
            supports.remove(collisions.get(0).unwrap());
        }
    }
    supports.len()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("22.txt").unwrap();
    let mut board = Board::new(&contents);
    board.settle_board();
    println!("{}", part1(&board));
    println!("{}", part2_fast(&board));
}