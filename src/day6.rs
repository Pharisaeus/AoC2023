use std::fs;
use itertools::Itertools;
use regex::Regex;

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn winning_options_fast(&self) -> u64 {
        // solver parable time*x -x^2 > record
        let time = self.time as f64;
        let delta = (time * time - 4.0 * (self.record as f64 + 0.0000000001));
        let x1 = ((time - delta.sqrt()) / 2.0).floor() as u64;
        let x2 = ((time + delta.sqrt()) / 2.0).floor() as u64;
        x2 - x1
    }

    fn winning_options_slow(&self) -> u64 {
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
        .map(|r| r.winning_options_fast())
        .product()
}

fn part2(data: &str) -> u64 {
    let races = parse(data.replace(" ", "").as_str());
    races.iter()
        .map(|r| r.winning_options_fast())
        .product()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("6.txt").unwrap();
    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}
