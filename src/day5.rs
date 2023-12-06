use std::fs;
use itertools::Itertools;

struct NumberRange {
    start: u64,
    length: u64,
}

impl NumberRange {
    fn is_in_range(&self, value: &u64) -> bool {
        self.start <= *value && self.start + self.length > *value
    }

    fn offset(&self, value: &u64) -> Option<u64> {
        return if self.is_in_range(value) {
            Some(value - self.start)
        } else {
            None
        };
    }
    fn overlap_size(&self, other: &NumberRange) -> u64 {
        let min = (self.start + self.length).min(other.start + other.length);
        let max = self.start.max(other.start);
        return if min >= max {
            min - max + 1
        } else {
            0
        };
    }

    fn overlap_range(&self, other: &NumberRange) -> Option<NumberRange> {
        let length = self.overlap_size(other);
        return if length > 0 {
            let start = self.start.max(other.start);
            Some(NumberRange { start, length })
        } else {
            None
        };
    }
}

struct MappingRange {
    range: NumberRange,
    destination_start: u64,
}

impl MappingRange {
    fn new(line: &str) -> Self {
        let (destination_start, start, length) = line.split(" ")
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        MappingRange {
            range: NumberRange { start, length },
            destination_start,
        }
    }

    fn map_value(&self, value: &u64) -> Option<u64> {
        self.range.offset(value)
            .map(|offset| self.destination_start + offset)
    }

    fn overlap_size(&self, other: &NumberRange) -> u64 {
        self.range.overlap_size(other)
    }

    fn map_range(&self, other: &NumberRange) -> Option<MappingRange> {
        self.range.overlap_range(other)
            .map(|range|
                MappingRange {
                    destination_start: self.map_value(&range.start).unwrap(),
                    range,
                })
    }
}

struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn new(data: &str) -> Self {
        Mapping {
            ranges: data.lines()
                .skip(1)
                .map(|line| MappingRange::new(line))
                .collect()
        }
    }

    fn map_value(&self, value: &u64) -> u64 {
        self.ranges
            .iter()
            .filter_map(|range| range.map_value(value))
            .find_or_first(|v| true)
            .unwrap_or(*value)
    }

    fn map_range(&self, range_to_map: &NumberRange) -> Vec<MappingRange> {
        // FIXME: fill the holes of x->x?
        self.ranges
            .iter()
            .filter_map(|range| range.map_range(range_to_map))
            .collect()
    }
}

struct Mappings {
    mappings: Vec<Mapping>,
}

impl Mappings {
    fn find_seed_location(&self, seed: &u64) -> u64 {
        let mut x = *seed;
        for mapping in &self.mappings {
            x = mapping.map_value(&x);
        }
        x
    }

    fn find_seed_range_best_location(&self, seed: &NumberRange) -> u64 {
        let mut x = vec![MappingRange {
            range: NumberRange {
                start: 0,
                length: seed.length,
            },
            destination_start: seed.start,
        }];
        for mapping in &self.mappings {
            x = x.iter()
                .map(|r| mapping.map_range(&r.range))
                .flatten()
                .collect()
        }
        x.iter()
            .map(|x| x.destination_start)
            .min()
            .unwrap_or(99999999999999)
    }
}

struct Planting {
    seeds: Vec<u64>,
    mappings: Mappings,
}

impl Planting {
    fn new(data: &str) -> Self {
        let blocks: Vec<&str> = data.split("\n\n")
            .collect();
        let (_, seed_numbers) = blocks.get(0).unwrap().split(": ").collect_tuple().unwrap();
        let seeds = seed_numbers.split(" ").map(|x| x.parse().unwrap()).collect();
        let mappings = Mappings {
            mappings: blocks.iter().skip(1)
                .map(|&block| Mapping::new(block))
                .collect()
        };
        Planting {
            seeds,
            mappings,
        }
    }

    fn locations(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|seed| self.mappings.find_seed_location(seed))
            .collect()
    }

    fn locations_range(&self, best_in_range: impl Fn(u64, u64) -> u64) -> Vec<u64> {
        self.seeds
            .chunks(2)
            .map(|c| c.iter().collect_tuple().unwrap())
            .map(|(&start, &length)| best_in_range(start, length))
            .collect()
    }

    fn best_in_range_slow(&self, seed: &NumberRange) -> u64 {
        println!("Testing range {} (number of elements {})", seed.start, seed.length);
        (seed.start..seed.start + seed.length).map(|seed| self.mappings.find_seed_location(&seed))
            .min()
            .unwrap()
    }

    fn best_in_range_fast(&self, seed: &NumberRange) -> u64 {
        self.mappings.find_seed_range_best_location(seed)
    }
}

fn part2(planting: &Planting) -> u64 {
    // planting.locations_range(|start, length| planting.best_in_range_slow(&NumberRange { start, length }))
    planting.locations_range(|start, length| planting.best_in_range_fast(&NumberRange { start, length }))
        .iter()
        .min()
        .unwrap()
        .clone()
}

fn part1(planting: &Planting) -> u64 {
    planting.locations()
        .iter()
        .min()
        .unwrap()
        .clone()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("5.txt").unwrap();
    let planting = Planting::new(&contents);
    println!("{}", part1(&planting));
    println!("{}", part2(&planting));
}