use std::collections::{HashMap, VecDeque};
use std::{fs, vec};
use std::hash::Hash;
use itertools::Itertools;

#[derive(PartialEq, Eq, Copy, Clone)]
enum PulseType {
    High,
    Low,
}

#[derive(Clone)]
struct Signal {
    pulse: PulseType,
    source: String,
}

trait Module {
    fn handle_signal(&mut self, signal: &Signal) -> Option<PulseType> {
        None
    }

    fn reset(&mut self) {}
}

enum FlipFlopStatus {
    On,
    Off,
}

struct FlipFlop {
    status: FlipFlopStatus,
}

impl Module for FlipFlop {
    fn handle_signal(&mut self, signal: &Signal) -> Option<PulseType> {
        if signal.pulse == PulseType::Low {
            return match self.status {
                FlipFlopStatus::Off => {
                    self.status = FlipFlopStatus::On;
                    Some(PulseType::High)
                }
                FlipFlopStatus::On => {
                    self.status = FlipFlopStatus::Off;
                    Some(PulseType::Low)
                }
            };
        }
        return None;
    }

    fn reset(&mut self) {
        self.status = FlipFlopStatus::Off
    }
}

struct Conjunction {
    inputs: Vec<String>,
    history: HashMap<String, PulseType>,
    emitted_high: bool,
}

impl Module for Conjunction {
    fn handle_signal(&mut self, signal: &Signal) -> Option<PulseType> {
        self.history.insert(signal.source.clone(), signal.pulse.clone());
        return if self.inputs.iter()
            .map(|name| self.history.get(name).unwrap_or(&PulseType::Low))
            .all(|&x| x == PulseType::High) {
            Some(PulseType::Low)
        } else {
            self.emitted_high = true;
            Some(PulseType::High)
        };
    }

    fn reset(&mut self) {
        self.history = HashMap::new();
        self.emitted_high = false;
    }
}

struct Broadcaster {}

impl Module for Broadcaster {
    fn handle_signal(&mut self, signal: &Signal) -> Option<PulseType> {
        Some(signal.pulse.clone())
    }
}

struct Output {
    history: Vec<PulseType>,
}

impl Module for Output {}

struct OutgoingSignal {
    signal: Signal,
    destination: String,
}

struct GreatMachine {
    modules: HashMap<String, Box<dyn Module>>,
    connections: HashMap<String, Vec<String>>,
    output: Box<dyn Module>,
}

impl GreatMachine {
    fn new(data: &str) -> Self {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();
        for line in data.lines() {
            let (type_and_name, children) = line.split(" -> ").collect_tuple().unwrap();
            let mut name = type_and_name.to_string();
            if type_and_name.contains("%") {
                name = type_and_name.replace("%", "");
            } else if type_and_name.contains("&") {
                name = type_and_name.replace("&", "");
            }
            connections.insert(name.clone(), children.split(", ").map(|child| child.to_string()).collect());
        }
        for line in data.lines() {
            let (type_and_name, _) = line.split(" -> ").collect_tuple().unwrap();
            let mut name = type_and_name.to_string();
            let mut module: Box<dyn Module> = Box::new(Broadcaster {});
            if type_and_name.contains("%") {
                name = type_and_name.replace("%", "");
                module = Box::new(FlipFlop { status: FlipFlopStatus::Off });
            } else if type_and_name.contains("&") {
                name = type_and_name.replace("&", "");
                let inputs = connections.iter()
                    .filter(|(k, v)| v.contains(&name))
                    .map(|(k, v)| k.clone())
                    .collect();
                module = Box::new(Conjunction { inputs, history: Default::default(), emitted_high: false });
            }
            modules.insert(name.clone(), module);
        }
        Self {
            modules,
            connections,
            output: Box::new(Output { history: vec![] }),
        }
    }

    fn press_button(&mut self) -> (i64, i64) {
        let mut signals = VecDeque::new();
        let mut highs = 0;
        let mut lows = 1;
        signals.push_back(OutgoingSignal { signal: Signal { pulse: PulseType::Low, source: "".to_string() }, destination: "broadcaster".to_string() });
        while !signals.is_empty() {
            let outgoing = signals.pop_front().unwrap();
            let destination_module = self.modules.get_mut(&outgoing.destination).unwrap_or(&mut self.output);
            let new_pulse = destination_module.handle_signal(&outgoing.signal);
            if new_pulse.is_some() {
                let pulse = new_pulse.unwrap();
                for child in self.connections.get(&outgoing.destination).unwrap() {
                    match pulse {
                        PulseType::High => highs += 1,
                        PulseType::Low => lows += 1,
                    }
                    let signal_to_send = Signal { pulse: pulse.clone(), source: outgoing.destination.clone() };
                    let signal_to_child = OutgoingSignal { signal: signal_to_send, destination: child.clone() };
                    signals.push_back(signal_to_child)
                }
            }
        }
        (lows, highs)
    }

    fn extract_conjunction(&self, name: &str) -> &Conjunction {
        let y = self.modules.get(name).unwrap().as_ref();
        // reinterpret cast goes brrr
        unsafe { &*(y as *const dyn Module as *const Conjunction) }
    }

    fn reset(&mut self) {
        for mut m in self.modules.values_mut() {
            m.reset();
        }
    }
}


fn count_cycle(node: &str, machine: &mut GreatMachine) -> i64 {
    machine.reset();
    let mut presses = 0;
    loop {
        presses += 1;
        machine.press_button();
        let conjunction = machine.extract_conjunction(node);
        if conjunction.emitted_high {
            break;
        }
    }
    presses
}

fn lcm(nums: &Vec<i64>) -> i64 {
    nums.iter()
        .map(|x| *x)
        .reduce(|x, y| (x * y) / (gcd(x, y)))
        .unwrap()
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn part2(machine: &mut GreatMachine) -> i64 {
    let inputs = machine.extract_conjunction("vd").inputs.clone();
    let cycles = inputs
        .iter()
        .map(|node| count_cycle(node, machine))
        .collect();
    lcm(&cycles)
}

fn part1(machine: &mut GreatMachine) -> i64 {
    let mut lows = 0;
    let mut highs = 0;
    for _ in 0..1000 {
        let (l, h) = machine.press_button();
        lows += l;
        highs += h;
    }
    lows * highs
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("20.txt").unwrap();
    let machine = &mut GreatMachine::new(&contents);
    println!("{}", part1(machine));
    println!("{}", part2(machine));
}