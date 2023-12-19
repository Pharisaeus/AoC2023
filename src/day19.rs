use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

#[derive(Clone)]
enum Comparison {
    Greater,
    Lesser,
    GreaterOrEq,
    LesserOrEq,
}

impl Comparison {
    fn evaluate(&self, v1: &i64, v2: &i64) -> bool {
        match self {
            Comparison::Greater => v1 > v2,
            Comparison::Lesser => v1 < v2,
            Comparison::GreaterOrEq => v1 >= v2,
            Comparison::LesserOrEq => v1 <= v2
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Comparison::Greater => Comparison::LesserOrEq,
            Comparison::Lesser => Comparison::GreaterOrEq,
            Comparison::GreaterOrEq => Comparison::Lesser,
            Comparison::LesserOrEq => Comparison::Greater
        }
    }
}

#[derive(Clone)]
struct Condition {
    variable_name: String,
    comparison: Comparison,
    constant: i64,
}

impl Condition {
    fn new(comp: &str) -> Self {
        let mut comparison = Comparison::Greater;
        let mut variable_name = "";
        let mut constant = "0";
        if comp.contains("<") {
            comparison = Comparison::Lesser;
            (variable_name, constant) = comp.split("<").collect_tuple().unwrap();
        } else {
            comparison = Comparison::Greater;
            (variable_name, constant) = comp.split(">").collect_tuple().unwrap();
        }
        Self {
            variable_name: variable_name.to_string(),
            comparison,
            constant: constant.parse().unwrap(),
        }
    }

    fn true_condition() -> Self {
        Self {
            variable_name: "TRUE".to_string(),
            comparison: Comparison::Greater,
            constant: -1,
        }
    }

    fn negate(&self) -> Self {
        Self {
            variable_name: self.variable_name.to_string(),
            comparison: self.comparison.opposite(),
            constant: self.constant,
        }
    }

    fn evaluate(&self, query: &Query) -> bool {
        self.simple_evaluate(query.variables.get(&self.variable_name).unwrap_or(&0))
    }

    fn simple_evaluate(&self, value: &i64) -> bool {
        self.comparison.evaluate(value, &self.constant)
    }
}

struct ConditionsChain {
    conditions: Vec<Condition>,
}

impl ConditionsChain {
    fn combinations(&self) -> i64 {
        self.group_conditions_by_variable()
            .iter()
            .map(|c| c.solutions_count())
            .product::<i64>()
    }

    fn group_conditions_by_variable(&self) -> Vec<ConditionsChain> {
        let mut x = vec![Condition::true_condition()];
        let mut m = vec![Condition::true_condition()];
        let mut a = vec![Condition::true_condition()];
        let mut s = vec![Condition::true_condition()];
        for c in &self.conditions {
            match c.variable_name.as_str() {
                "x" => { x.push(c.clone()) }
                "m" => { m.push(c.clone()) }
                "a" => { a.push(c.clone()) }
                "s" => { s.push(c.clone()) }
                "TRUE" => {}
                _ => panic!()
            }
        }
        vec![
            ConditionsChain { conditions: x },
            ConditionsChain { conditions: m },
            ConditionsChain { conditions: a },
            ConditionsChain { conditions: s },
        ]
    }

    fn is_accepted(&self, value: &i64) -> bool {
        self.conditions.iter().all(|c| c.simple_evaluate(value))
    }

    fn solutions_count(&self) -> i64 {
        (1..=4000)
            .filter(|v| self.is_accepted(v))
            .count() as i64
    }
}

struct Transition {
    condition: Condition,
    target: String,
}

impl Transition {
    fn new(data: &str) -> Self {
        let (c, target) = data.split(":").collect_tuple().unwrap();
        Self {
            condition: Condition::new(c),
            target: target.to_string(),
        }
    }
    fn is_valid(&self, query: &Query) -> bool {
        self.condition.evaluate(query)
    }
}

struct Step {
    step_name: String,
    transitions: Vec<Transition>,
}

impl Step {
    fn new(step_name: &str, conditions: &str) -> Self {
        let condition_strings: Vec<String> = conditions.replace("}", "")
            .split(",")
            .map(|x| x.to_string())
            .collect();
        let mut transitions: Vec<Transition> = condition_strings.iter()
            .take(condition_strings.len() - 1)
            .map(|c| Transition::new(c))
            .collect();
        let fallback = condition_strings.last().unwrap().to_string();
        transitions.push(Transition { condition: Condition::true_condition(), target: fallback });
        Self {
            step_name: step_name.to_string(),
            transitions,
        }
    }

    fn next_step(&self, query: &Query) -> String {
        for c in &self.transitions {
            if c.is_valid(query) {
                return c.target.clone();
            }
        }
        panic!()
    }
}

struct Workflow {
    steps: HashMap<String, Step>,
}

impl Workflow {
    fn new(data: &str) -> Self {
        let mut steps = HashMap::new();
        for line in data.lines() {
            let (step_name, rest) = line.split("{").collect_tuple().unwrap();
            steps.insert(step_name.to_string(), Step::new(step_name, rest));
        }
        Self {
            steps
        }
    }

    fn evaluate(&self, query: &Query) -> bool {
        let mut current = "in".to_string();
        while current != "A" && current != "R" {
            let current_step = self.steps.get(&current).unwrap();
            current = current_step.next_step(query);
        }
        current == "A"
    }

    fn acceptance_chains(&self) -> Vec<ConditionsChain> {
        let step = self.steps.get("in").unwrap();
        let chains = self.expand_step(&vec![vec![]], &step);
        let mut result = vec![];
        for conditions in chains {
            result.push(ConditionsChain { conditions });
        }
        result
    }

    fn expand_step(&self, paths: &Vec<Vec<Condition>>, step: &Step) -> Vec<Vec<Condition>> {
        let mut result_paths = vec![];
        let mut negated_conditions: Vec<Condition> = vec![];
        for transition in &step.transitions {
            let mut paths_with_negations = paths.clone();
            for c in &negated_conditions {
                for p in &mut paths_with_negations {
                    p.push(c.clone())
                }
            }
            result_paths.extend(self.expand_condition(&paths_with_negations, &transition.condition, &transition.target));
            negated_conditions.push(transition.condition.negate())
        }
        result_paths
    }

    fn expand_condition(&self, paths: &Vec<Vec<Condition>>, c: &Condition, target: &String) -> Vec<Vec<Condition>> {
        let mut result_paths = paths.clone();
        for p in &mut result_paths {
            p.push(c.clone())
        }
        return if target == "A" {
            result_paths
        } else if target == "R" {
            vec![]
        } else {
            let step = self.steps.get(target).unwrap();
            self.expand_step(&result_paths, &step)
        };
    }
}

struct Query {
    variables: HashMap<String, i64>,
}

impl Query {
    fn new(line: &str) -> Self {
        Self {
            variables: line.replace("{", "").replace("}", "")
                .split(",")
                .map(|b| b.split("=").collect_tuple().unwrap())
                .map(|(name, v)| (name.to_string(), v.parse().unwrap()))
                .collect()
        }
    }

    fn query_value(&self) -> i64 {
        self.variables
            .values()
            .sum()
    }
}

fn part2(workflow: &Workflow) -> i64 {
    workflow.acceptance_chains()
        .iter()
        .map(|chain| chain.combinations())
        .sum()
}

fn part1(workflow: &Workflow, queries: &Vec<Query>) -> i64 {
    queries.iter()
        .filter(|query| workflow.evaluate(query))
        .map(|query| query.query_value())
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("19.txt").unwrap();
    let (wkf, queries) = contents.split("\n\n").collect_tuple().unwrap();
    let workflow = Workflow::new(wkf);
    let queries = queries.split("\n")
        .map(|query| Query::new(query))
        .collect();
    println!("{}", part1(&workflow, &queries));
    println!("{}", part2(&workflow));
}