use std::fs;
use itertools::Itertools;
use regex::Regex;

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn winning_options(&self) -> u64 {
        (0..self.time)
            .filter(|speed| self.breaks_record(speed))
            .count() as u64
    }

    fn distance(&self, speed: &u64) -> u64 {
        (self.time - speed) * speed
    }

    fn breaks_record(&self, speed: &u64) -> bool {
        self.distance(speed) > self.record
    }
}

fn parse(data: &str) -> Vec<Race> {
    let pattern = Regex::new(r"\d+").unwrap();
    let (times, records): (Vec<u64>, Vec<u64>) = data.lines()
        .map(|line| pattern.find_iter(line)
            .map(|v| v.as_str().parse().unwrap())
            .collect())
        .collect_tuple()
        .unwrap();
    times.iter().zip(records)
        .map(|(&time, record)| Race { time, record })
        .collect()
}

fn part1(data: &str) -> u64 {
    let races = parse(data);
    races.iter()
        .map(|r| r.winning_options())
        .product()
}

fn part2(data: &str) -> u64 {
    let races = parse(data.replace(" ", "").as_str());
    races.iter()
        .map(|r| r.winning_options())
        .product()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("6.txt").unwrap();
    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}
