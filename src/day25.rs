use std::collections::{HashMap, HashSet};
use std::fs;
use itertools::Itertools;
use petgraph::graph::{UnGraph};
use rustworkx_core::centrality::edge_betweenness_centrality;
use rustworkx_core::connectivity::connected_components;

struct ParsedGraph {
    connections: HashSet<(String, String)>,
}

impl ParsedGraph {
    fn new(data: &str) -> Self {
        let mut connections = HashSet::new();
        for line in data.lines() {
            let (first, others) = line.split(": ").collect_tuple().unwrap();
            for other in others.split(" ") {
                connections.insert((first.to_string(), other.to_string()));
                connections.insert((other.to_string(), first.to_string()));
            }
        }
        Self {
            connections
        }
    }

    fn rustworkx_graph(&self) -> UnGraph<&str, ()> {
        let mut g = UnGraph::<&str, ()>::new_undirected();
        let nodes: HashMap<&String, _> = self.connections
            .iter()
            .unique()
            .map(|(k1, k2)| (k1, g.add_node(k1.as_str())))
            .collect();
        for (a, b) in &self.connections {
            let aa = nodes.get(a).unwrap();
            let bb = nodes.get(b).unwrap();
            if !g.contains_edge(*bb, *aa) {
                g.add_edge(*aa, *bb, ());
            }
        }
        g
    }
}

fn part1(graph: &ParsedGraph) -> usize {
    let mut g = graph.rustworkx_graph();
    let edge_betweenness = edge_betweenness_centrality(&g, false, 200);
    let edges_to_remove: Vec<_> = edge_betweenness.iter().zip(g.edge_indices())
        .sorted_by(|(v1, _), (v2, _)| v2.partial_cmp(v1).unwrap())
        .map(|(v, k)| k)
        .take(3)
        .collect();
    for e in edges_to_remove {
        g.remove_edge(e);
    }
    let components = connected_components(&g);
    components
        .iter()
        .map(|c| c.len())
        .product()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("25.txt").unwrap();
    let graph = ParsedGraph::new(&contents);
    println!("{}", part1(&graph));
}