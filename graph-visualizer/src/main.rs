mod graph_io;
mod longest_path;

use petgraph::{Graph, Undirected, graph::EdgeIndex};
use std::{
    io::{self, BufRead},
    sync::Arc,
    thread::{self, JoinHandle},
};
use visgraph::{Layout, graph_to_svg, settings::SettingsBuilder};

fn main() {
    let mut lines = io::stdin().lock().lines();

    let line1 = lines
        .next()
        .and_then(|l| l.ok())
        .expect("Failed to read line");

    let mut nums = line1
        .split_whitespace()
        .take(2)
        .map(|s| s.parse::<usize>().expect("Failed to parse number"));

    let n = nums.next().expect("Missing n");
    let m = nums.next().expect("Missing m");

    let graph: Graph<String, f64, Undirected> = graph_io::read_graph(lines, n, m);

    let graph_m = Arc::new(graph.clone());
    let mut threads: Vec<JoinHandle<(Vec<EdgeIndex>, f64)>> = Vec::with_capacity(n);

    for start in graph_m.node_indices() {
        let graph_m = Arc::clone(&graph_m);
        let handle = thread::spawn(move || longest_path::longest_from_start(start, &graph_m));
        threads.push(handle);
    }

    let (path, weight) = threads
        .into_iter()
        .map(|h| h.join().unwrap())
        .max_by(|(_, w1), (_, w2)| w1.partial_cmp(w2).unwrap())
        .unwrap();

    let (node_coloring, edge_coloring) = graph_io::path_highlighter(&path, &graph);

    // lp_030 gives a good visual
    let settings = SettingsBuilder::new()
        .layout(Layout::Circular)
        .edge_coloring_fn(edge_coloring)
        .node_coloring_fn(node_coloring)
        .build()
        .unwrap();

    let _ = graph_to_svg(&graph, &settings, "test.svg");

    println!("{weight}");
}
