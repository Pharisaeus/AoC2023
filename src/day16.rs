use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use itertools::{Itertools};
use crate::day16::Tile::{AngleLeft, AngleRight, Empty, HorizontalSplit, VerticalSplit};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_vector(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        }
    }
}

enum Tile {
    Empty,
    HorizontalSplit,
    VerticalSplit,
    AngleRight,
    AngleLeft,
}

impl Tile {
    fn new(c: &char) -> Self {
        match c {
            '.' => Empty,
            '-' => HorizontalSplit,
            '|' => VerticalSplit,
            '/' => AngleRight,
            '\\' => AngleLeft,
            &_ => panic!()
        }
    }

    fn handle_beam(&self, direction: Direction) -> Vec<Direction> {
        match self {
            Empty => vec![direction],
            HorizontalSplit => match direction {
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left | Direction::Right => vec![direction],
            }
            VerticalSplit => match direction {
                Direction::Up | Direction::Down => vec![direction],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            }
            AngleRight => match direction {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            }
            AngleLeft => match direction {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            }
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Beam {
    pos: (i64, i64),
    direction: Direction,
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn new(data: &str) -> Self {
        Self {
            tiles: data.lines()
                .map(|line| line.chars()
                    .map(|c| Tile::new(&c))
                    .collect())
                .collect()
        }
    }

    fn height(&self) -> i64 {
        self.tiles.len() as i64
    }

    fn width(&self) -> i64 {
        self.tiles.get(0).unwrap().len() as i64
    }

    fn is_in_grid(&self, x: i64, y: i64) -> bool {
        x >= 0 && y >= 0 && x < self.width() && y < self.height()
    }

    fn progress_beams(&self, beams: &HashSet<Beam>) -> HashSet<Beam> {
        let mut progressed_beams = HashSet::new();
        for beam in beams {
            let (dx, dy) = beam.direction.move_vector();
            let (x, y) = beam.pos;
            let nx = x + dx;
            let ny = y + dy;
            if self.is_in_grid(nx, ny) {
                let (dest_x, dest_y) = (nx as usize, ny as usize);
                let destination_tile = self.tiles.get(dest_y).unwrap().get(dest_x).unwrap();
                let resulting_beams = destination_tile.handle_beam(beam.direction);
                for d in resulting_beams {
                    progressed_beams.insert(Beam { pos: (nx, ny), direction: d });
                }
            }
        }
        progressed_beams
    }

    fn energize(&self, starting_beam: Beam) -> HashSet<Beam> {
        let mut all_beams = HashSet::new();
        let mut beams = HashSet::from([starting_beam]);
        loop {
            beams = self.progress_beams(&beams);
            if beams.difference(&all_beams).count() == 0 {
                break;
            } else {
                all_beams.extend(&beams);
            }
        }
        all_beams
    }
}

fn count_energized(grid: &Grid, starting_beam: Beam) -> usize {
    grid.energize(starting_beam)
        .iter()
        .map(|beam| beam.pos)
        .unique()
        .count()
}

fn part1(grid: &Grid) -> usize {
    count_energized(grid, Beam { pos: (-1, 0), direction: Direction::Right })
}

fn part2(grid: &Grid) -> usize {
    let mut possible_solutions = vec![];
    for x in 0..grid.width() {
        possible_solutions.push(count_energized(grid, Beam { pos: (x, -1), direction: Direction::Down }));
        possible_solutions.push(count_energized(grid, Beam { pos: (x, grid.height()), direction: Direction::Up }));
    }
    for y in 0..grid.height() {
        possible_solutions.push(count_energized(grid, Beam { pos: (-1, y), direction: Direction::Right }));
        possible_solutions.push(count_energized(grid, Beam { pos: (grid.width(), y), direction: Direction::Left }));
    }
    *possible_solutions.iter().max().unwrap()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("16.txt").unwrap();
    let grid = Grid::new(&contents);
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}