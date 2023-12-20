use std::fs;
use itertools::Itertools;

fn hash(data: &str) -> usize {
    let mut current = 0;
    for c in data.chars() {
        let ascii_ordinal = c as u8;
        current += ascii_ordinal as usize;
        current *= 17;
        current %= 256;
    }
    current
}

struct Entry {
    label: String,
    focal_length: usize,
}

struct CustomHashMapBox {
    entries: Vec<Entry>,
}

impl CustomHashMapBox {
    fn new() -> Self {
        Self {
            entries: vec![],
        }
    }

    fn remove(&mut self, label: &str) -> Option<Entry> {
        let position = self.entries
            .iter()
            .find_position(|x| x.label.eq(label));
        match position {
            None => None,
            Some((pos, _)) => Some(self.entries.remove(pos))
        }
    }

    fn insert(&mut self, entry: Entry) {
        let position = self.entries
            .iter()
            .find_position(|x| x.label.eq(entry.label.as_str()));
        match position {
            None => self.entries.push(entry),
            Some((pos, _)) => self.entries[pos] = entry
        }
    }

    fn focusing_power(&self) -> usize {
        self.entries.iter().enumerate()
            .map(|(lens_slot, lens)| (lens_slot + 1) * lens.focal_length)
            .sum()
    }
}

struct CustomHashMap {
    boxes: Vec<CustomHashMapBox>,
}

impl CustomHashMap {
    fn new() -> Self {
        Self {
            boxes: (0..256).map(|_| CustomHashMapBox::new()).collect()
        }
    }

    fn process_operation(&mut self, op: &str) {
        if op.contains("=") {
            let (label, focal) = op.split("=").collect_tuple().unwrap();
            let entry = Entry { label: label.to_string(), focal_length: focal.parse().unwrap() };
            let h = hash(label);
            self.boxes[h].insert(entry);
        } else {
            let label = op.replace("-", "");
            let h = hash(label.as_str());
            self.boxes[h].remove(label.as_str());
        }
    }

    fn focusing_power(&self) -> usize {
        self.boxes.iter().enumerate()
            .map(|(box_id, lens_box)| (box_id + 1) * lens_box.focusing_power())
            .sum()
    }
}

fn part2(data: &str) -> usize {
    let mut hashmap = CustomHashMap::new();
    for op in data.split(",") {
        hashmap.process_operation(op);
    }
    hashmap.focusing_power()
}

fn part1(data: &str) -> usize {
    data.split(",")
        .map(|step| hash(step))
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("15.txt").unwrap();
    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}