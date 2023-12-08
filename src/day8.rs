use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

enum Direction {
    L,
    R,
}

impl Direction {
    fn new(c: &char) -> Self {
        match c {
            'L' => Direction::L,
            'R' => Direction::R,
            &_ => panic!()
        }
    }
}

struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(line: &str) -> (String, Self) {
        let (label, moves) = line.split(" = ").collect_tuple().unwrap();
        let x = moves.replace("(", "").replace(")", "");
        let (left, right) = x.split(", ").collect_tuple().unwrap();
        (label.to_string(), Node {
            left: left.to_string(),
            right: right.to_string(),
        })
    }

    fn next_node(&self, direction: &Direction) -> &String {
        match direction {
            Direction::L => &self.left,
            Direction::R => &self.right
        }
    }
}

struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn new(data: &str) -> Self {
        let (directions, graph) = data.split("\n\n").collect_tuple().unwrap();
        let nodes = graph.lines()
            .map(|line| Node::new(line))
            .collect();
        Map {
            directions: directions.chars().map(|c| Direction::new(&c)).collect(),
            nodes,
        }
    }

    fn next_node(&self, from: &String, position: usize) -> &String {
        let direction = self.directions.get(position % self.directions.len()).unwrap();
        self.nodes.get(from).unwrap().next_node(direction)
    }
}

fn lcm(nums: &Vec<usize>) -> usize {
    nums.iter()
        .map(|x| *x)
        .reduce(|x, y| (x * y) / (gcd(x, y)))
        .unwrap()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn part2(map: &Map) -> usize {
    let starting_positions: Vec<&String> = map.nodes.keys()
        .filter(|&x| x.ends_with("A"))
        .collect();
    let cycles = starting_positions.iter()
        .map(|start| step_count(map, start, &|current| current.ends_with("Z")))
        .collect();
    lcm(&cycles)
}

fn step_count(map: &Map, start: &String, end_condition: &impl Fn(&String) -> bool) -> usize {
    let mut current = start;
    let mut step_counter = 0;
    while !end_condition(current) {
        current = map.next_node(current, step_counter);
        step_counter += 1;
    }
    step_counter
}

fn part1(map: &Map) -> usize {
    let start = "AAA".to_string();
    step_count(map, &start, &|current| current.eq("ZZZ"))
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("8.txt").unwrap();
    let map = Map::new(&contents);
    println!("{}", part1(&map));
    println!("{}", part2(&map));
}