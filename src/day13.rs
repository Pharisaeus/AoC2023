use std::fs;
use itertools::Itertools;
use crate::day13::Terrain::{ASH, ROCK};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Terrain {
    ASH,
    ROCK,
}

impl Terrain {
    fn new(c: &char) -> Self {
        match c {
            '.' => ASH,
            '#' => ROCK,
            &_ => panic!()
        }
    }
    fn opposite(&self) -> Self {
        match self {
            ASH => ROCK,
            ROCK => ASH
        }
    }
}

struct Pattern {
    rows: Vec<Vec<Terrain>>,
    columns: Vec<Vec<Terrain>>,
}

impl Pattern {
    fn new(data: &str) -> Self {
        let mut rows = vec![];
        let column_count = data.lines().find_or_first(|_| true).map(|line| line.len()).unwrap();
        let row_count = data.lines().count();
        let mut columns: Vec<Vec<Terrain>> = (0..column_count)
            .map(|_| vec![ASH; row_count])
            .collect();

        for (row_index, data_row) in data.lines().enumerate() {
            let mut row = vec![];
            for (column_index, c) in data_row.chars().enumerate() {
                let t = Terrain::new(&c);
                row.push(t);
                columns[column_index][row_index] = t;
            }
            rows.push(row);
        }

        Pattern {
            rows,
            columns,
        }
    }

    fn find_splits(&self, input: &Vec<Vec<Terrain>>) -> Vec<usize> {
        self.find_potential_split_starts(input)
            .iter()
            .filter(|&&index| self.verify_split(input, index))
            .map(|x| *x)
            .collect()
    }

    fn find_potential_split_starts(&self, input: &Vec<Vec<Terrain>>) -> Vec<usize> {
        input.iter()
            .tuple_windows()
            .enumerate()
            .filter(|(index, (a, b))| a.eq(b))
            .map(|(index, _)| index)
            .collect()
    }

    fn verify_split(&self, input: &Vec<Vec<Terrain>>, position: usize) -> bool {
        let left = input.as_slice()[0..position + 1].iter().rev();
        let right = &input.as_slice()[position + 1..input.len()];
        left.zip(right)
            .all(|(a, b)| a.eq(b))
    }

    fn reflections(&self) -> Vec<usize> {
        let v: Vec<usize> = self.find_splits(&self.columns).iter().map(|x| x + 1).collect();
        let h: Vec<usize> = self.find_splits(&self.rows).iter().map(|x| (x + 1) * 100).collect();
        [v, h].concat()
    }

    fn smudges(&self) -> usize {
        let original_reflection = *self.reflections().first().unwrap();
        for (row_index, row) in self.rows.iter().enumerate() {
            for (col_index, terrain) in row.iter().enumerate() {
                let mut new_rows = self.rows.clone();
                let mut new_cols = self.columns.clone();
                new_rows[row_index][col_index] = terrain.opposite();
                new_cols[col_index][row_index] = terrain.opposite();
                let smudge_reflections = Pattern {
                    rows: new_rows,
                    columns: new_cols,
                }.reflections();
                let new_reflection = smudge_reflections
                    .iter()
                    .find(|&&x| x != original_reflection);
                if new_reflection.is_some() {
                    return *new_reflection.unwrap();
                }
            }
        }
        panic!()
    }
}

fn part2(patterns: &Vec<Pattern>) -> usize {
    patterns
        .iter()
        .map(|p| p.smudges())
        .sum()
}

fn part1(patterns: &Vec<Pattern>) -> usize {
    patterns
        .iter()
        .map(|p| *p.reflections().first().unwrap())
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("13.txt").unwrap();
    let patterns = contents.split("\n\n")
        .map(|data| Pattern::new(data))
        .collect();
    println!("{}", part1(&patterns));
    println!("{}", part2(&patterns));
}