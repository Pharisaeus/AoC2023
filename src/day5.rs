use std::fs;
use itertools::Itertools;

struct MappedRange {
    source: u64,
    destination: u64,
    length: u64,
}

impl MappedRange {
    fn new(line: &str) -> Self {
        let (destination, source, length) = line.split(" ")
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        MappedRange {
            source,
            destination,
            length,
        }
    }

    fn is_in_range(&self, src: &u64) -> bool {
        self.source <= *src && self.source + self.length > *src
    }

    fn get_mapped_value(&self, src: &u64) -> Option<u64> {
        return if self.is_in_range(src) {
            let offset = src - self.source;
            Some(self.destination + offset)
        } else {
            None
        };
    }

    fn overlap_size(&self, src: &u64, length: &u64) -> u64 {
        let min = (self.source + self.length).min(src + length);
        let max = self.source.max(*src);
        return if min >= max {
            min - max + 1
        } else {
            0
        };
    }

    fn get_mapped_range(&self, src: &u64, length: &u64) -> Option<MappedRange> {
        let overlap = self.overlap_size(src, length);
        return if overlap > 0 {
            let start = self.source.max(*src);
            Some(MappedRange { source: start, destination: self.get_mapped_value(&start).unwrap(), length: overlap })
        } else {
            None
        };
    }
}

struct Mapping {
    ranges: Vec<MappedRange>,
}

impl Mapping {
    fn new(data: &str) -> Self {
        Mapping {
            ranges: data.lines()
                .skip(1)
                .map(|line| MappedRange::new(line))
                .collect()
        }
    }

    fn get_mapped_value(&self, src: &u64) -> u64 {
        self.ranges
            .iter()
            .filter_map(|range| range.get_mapped_value(src))
            .find_or_first(|v| true)
            .unwrap_or(*src)
    }

    fn get_mapped_range(&self, src: &u64, length: &u64) -> Vec<MappedRange> {
        // FIXME: fill the holes of x->x?
        self.ranges
            .iter()
            .filter_map(|range| range.get_mapped_range(src, length))
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
            x = mapping.get_mapped_value(&x);
        }
        x
    }

    fn find_seed_range_best_location(&self, seed: &u64, length: &u64) -> u64 {
        let mut x = vec![MappedRange {
            source: 0,
            destination: *seed,
            length: *length,
        }];
        for mapping in &self.mappings {
            x = x.iter()
                .map(|r| mapping.get_mapped_range(&r.destination, &r.length))
                .flatten()
                .collect()
        }
        x.iter()
            .map(|x| x.destination)
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

    fn get_locations(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|seed| self.mappings.find_seed_location(seed))
            .collect()
    }

    fn get_locations_range(&self, best_in_range: impl Fn(u64, u64) -> u64) -> Vec<u64> {
        self.seeds
            .chunks(2)
            .map(|c| c.iter().collect_tuple().unwrap())
            .map(|(&start, &length)| best_in_range(start, length))
            .collect()
    }

    fn best_in_range_slow(&self, s: u64, length: u64) -> u64 {
        println!("Testing range {} (number of elements {}", s, length);
        (s..s + length).map(|seed| self.mappings.find_seed_location(&seed))
            .min()
            .unwrap()
    }

    fn best_in_range_fast(&self, s: u64, length: u64) -> u64 {
        self.mappings.find_seed_range_best_location(&s, &length)
    }
}

fn part2(planting: &Planting) -> u64 {
    // planting.get_locations_range(|start, length| planting.best_in_range_slow(start, length))
    planting.get_locations_range(|start, length| planting.best_in_range_fast(start, length))
        .iter()
        .min()
        .unwrap()
        .clone()
}

fn part1(planting: &Planting) -> u64 {
    planting.get_locations()
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