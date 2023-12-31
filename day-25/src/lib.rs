use std::collections::{HashMap, HashSet};

use aoc_commons::*;
use petgraph::{graph::UnGraph, visit::EdgeRef};
//use petgraph::dot::Dot;
//use std::fs::File;
//use std::io::Write;

pub fn min_cut_phase(graph: &mut UnGraph<String, i32>) -> (String, i32) {
    let n_node = graph.node_count();
    let mut cur_node = graph.node_indices().next().unwrap();
    let mut set_a = HashSet::from([cur_node]);
    let mut weights_to = HashMap::new();
    let mut order_added = vec![(cur_node, 0)];
    while set_a.len() != n_node {
        graph
            .edges(cur_node)
            .filter(|e| !set_a.contains(&e.target()))
            .for_each(|e| *weights_to.entry(e.target()).or_insert(0) += e.weight());
        let (new_node, edge_weight) = weights_to
            .iter()
            .filter(|(n, _w)| !set_a.contains(n))
            .max_by_key(|(_, w)| *w)
            .unwrap();
        set_a.insert(*new_node);
        order_added.push((*new_node, *edge_weight));
        cur_node = *new_node;
    }
    let (last_node, last_edge_weight) = order_added.last().unwrap();
    let last_node_weight = graph.node_weight(*last_node).unwrap();
    let cut = (last_node_weight.clone(), *last_edge_weight);
    let prev_last_node = order_added[n_node - 2].0;
    let prev_last_node_weight = graph.node_weight(prev_last_node).unwrap();
    let new_node = graph.add_node(format!("{}/{}", last_node_weight, prev_last_node_weight));
    //println!(
    //    "NEW: {new_node:?} : {}",
    //    graph.node_weight(new_node).unwrap()
    //);
    let mut new_edges = HashMap::new();
    for edge in graph.edges(*last_node).chain(graph.edges(prev_last_node)) {
        *new_edges.entry(edge.target()).or_insert(0) += edge.weight();
    }
    for (target, weight) in new_edges {
        graph.add_edge(new_node, target, weight);
    }
    //println!(
    //    "Remove: {}, {}",
    //    graph.node_weight(*last_node).unwrap(),
    //    graph.node_weight(prev_last_node).unwrap()
    //);
    graph.remove_node(*last_node);
    graph.remove_node(prev_last_node);
    cut
}
pub fn min_cut(graph: &UnGraph<String, i32>) -> (usize, usize) {
    let mut cur_graph = graph.clone();
    let mut best_cut = None;
    let mut count = 0;
    while cur_graph.node_count() > 0 {
        //{
        //    let path = format!("depth_{}.dot", count);
        //    if let Ok(mut output) = File::create(path) {
        //        write!(output, "{}", Dot::with_config(&cur_graph, &[])).unwrap();
        //    }
        //}
        if cur_graph.node_count() == 1 {
            break;
        }
        count += 1;
        println!("### {count}");
        let new_cut = min_cut_phase(&mut cur_graph);
        if let Some((_, edge_weight)) = best_cut {
            if new_cut.1 < edge_weight {
                best_cut = Some(new_cut.clone())
            }
        } else {
            best_cut = Some(new_cut.clone())
        }
        println!("Cut: {new_cut:?} Best:{best_cut:?}");
    }
    let g1 = best_cut.unwrap().0.split('/').count() + 1;
    let g2 = graph.node_count() - g1;
    (g1, g2)
}
pub fn solver(part: Part, input: &str) -> String {
    assert_eq!(part, Part::Part1);
    let mut graph = UnGraph::new_undirected();
    let mut nodes = HashMap::new();
    input.lines().for_each(|line| {
        let (from, tos) = line.split_once(':').unwrap();
        let from_id = *nodes
            .entry(from)
            .or_insert_with(|| graph.add_node(from.to_string()));

        tos.trim().split_ascii_whitespace().for_each(|to| {
            let to_id = *nodes
                .entry(to)
                .or_insert_with(|| graph.add_node(to.to_string()));
            graph.add_edge(from_id, to_id, 1);
        });
    });
    let (g1, g2) = min_cut(&graph);
    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    println!("{g1} x {g2}");
    (g1 * g2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "54");
    }
    //#[test]
    //fn example_part2() {
    //    assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "7");
    //}
}
