use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[derive(Debug)]
struct Node {
    color: String,
    in_edges: Vec<Edge>,
    out_edges: Vec<Edge>,
}

impl Node {
    fn new(color: &str) -> Self {
        Self {
            color: String::from(color),
            in_edges: Vec::new(),
            out_edges: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    weight: usize,
    other: usize,
}

impl Edge {
    fn new(weight: usize, other: usize) -> Self {
        Self { weight, other }
    }
}

#[derive(Debug)]
struct Graph {
    node_counter: usize,
    nodes: HashMap<usize, Node>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            node_counter: 0,
            nodes: HashMap::new(),
        }
    }

    fn insert_node(&mut self, color: &str) -> usize {
        self.node_counter += 1;
        self.nodes.insert(self.node_counter, Node::new(color));
        self.node_counter
    }

    fn get_node(&self, color: &str) -> Option<usize> {
        self.nodes
            .iter()
            .find(|(_, node)| node.color == color)
            .map(|(idx, _)| *idx)
    }

    fn num_parents(&self, node: usize) -> usize {
        let mut unique_nodes = HashSet::new();
        let mut count = 0;
        for in_edge in &self.nodes.get(&node).unwrap().in_edges {
            count += self.num_parents_internal(in_edge.other, &mut unique_nodes);
        }
        count
    }

    fn num_parents_internal(&self, node: usize, unique_nodes: &mut HashSet<usize>) -> usize {
        match unique_nodes.get(&node) {
            Some(_) => return 0,
            None => unique_nodes.insert(node),
        };
        let mut count = 1;
        for in_edge in &self.nodes.get(&node).unwrap().in_edges {
            count += self.num_parents_internal(in_edge.other, unique_nodes);
        }
        count
    }

    fn num_children(&self, node: usize) -> usize {
        let mut count = 0;
        for out_edge in &self.nodes.get(&node).unwrap().out_edges {
            count += out_edge.weight * self.num_children_internal(out_edge.other);
        }
        count
    }

    fn num_children_internal(&self, node: usize) -> usize {
        let mut count = 1;
        for out_edge in &self.nodes.get(&node).unwrap().out_edges {
            count += out_edge.weight * self.num_children_internal(out_edge.other);
        }
        count
    }

    fn get_node_or_insert(&mut self, color: &str) -> usize {
        match self.get_node(color) {
            Some(node) => node,
            None => self.insert_node(color),
        }
    }

    fn insert_edge(&mut self, from: &str, to: &str, weight: usize) {
        let from_node = self.get_node_or_insert(from);
        let to_node = self.get_node_or_insert(to);
        self.nodes
            .get_mut(&from_node)
            .unwrap()
            .out_edges
            .push(Edge::new(weight, to_node));
        self.nodes
            .get_mut(&to_node)
            .unwrap()
            .in_edges
            .push(Edge::new(weight, from_node));
    }
}

fn read_input() -> Result<Graph> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut graph = Graph::new();

    for line in reader.lines() {
        let parts = line?
            .split(" bags contain ")
            .map(|part| part.to_owned())
            .collect::<Vec<String>>();
        let outer_color = &parts[0];
        if parts[1].contains("no other") {
            continue;
        }
        let inner_colors = parts[1]
            .strip_suffix(".")
            .unwrap()
            .split(", ")
            .map(|inner| {
                if inner.ends_with(" bags") {
                    inner.strip_suffix(" bags").unwrap()
                } else {
                    inner
                }
            })
            .map(|inner| {
                if inner.ends_with(" bag") {
                    inner.strip_suffix(" bag").unwrap()
                } else {
                    inner
                }
            })
            .map(|inner| {
                let inner_parts = inner
                    .splitn(2, " ")
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>();
                (
                    inner_parts[0].parse::<usize>().unwrap(),
                    String::from(&inner_parts[1]),
                )
            })
            .collect::<Vec<(usize, String)>>();
        for (weight, color) in inner_colors {
            graph.insert_edge(outer_color, &color, weight);
        }
    }

    Ok(graph)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let graph = read_input()?;
    let shiny_gold = graph.get_node("shiny gold").unwrap();

    println!("result part 1: {}", graph.num_parents(shiny_gold));
    println!("result part 2: {}", graph.num_children(shiny_gold));

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph() {
        let g = read_input().unwrap();
        println!("{:#?}", g);
    }
}
