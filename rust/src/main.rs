use bitvec::{prelude::bitvec, vec::BitVec};
use core::f64;
use std::{
    cmp,
    collections::HashMap,
    io::{self, BufRead},
    sync::Arc,
    thread::{self, JoinHandle},
};

type Graph = HashMap<usize, HashMap<usize, f64>>;

fn longest_path_no_end(
    start: usize,
    weight: f64,
    visited: &mut BitVec,
    graph: &Graph,
) -> (Vec<usize>, f64) {
    visited.set(start, true);

    let Some(adjacent) = graph.get(&start) else {
        return (vec![start], weight);
    };

    let mut best_path: Vec<usize> = vec![start];
    let mut best_weight: f64 = weight;

    for (v, w) in adjacent {
        if visited[*v] {
            continue;
        }

        let (mut new_path, new_weight) = longest_path_no_end(*v, weight + *w, visited, graph);

        if new_weight > best_weight {
            best_weight = new_weight;
            new_path.insert(0, start);
            best_path = new_path;
        }

        visited.set(*v, false);
    }

    (best_path, best_weight)
}

fn read_edge(line: &str) -> (String, String, f64) {
    let mut line_vals = line.split_whitespace();

    let u = line_vals
        .next()
        .map(|s| s.to_string())
        .expect("Missing outgoing vertex");
    let v = line_vals
        .next()
        .map(|s| s.to_string())
        .expect("Missing incomming vertex");
    let w = line_vals
        .next()
        .and_then(|w| w.parse::<f64>().ok())
        .expect("Missing or invalid weight");

    (u, v, w)
}

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

    let mut graph: Graph = HashMap::with_capacity(n);

    let mut vertices: Vec<String> = Vec::with_capacity(n);
    let mut vertex_map: HashMap<String, usize> = HashMap::with_capacity(n);

    let edges = lines
        .take(m)
        .map(|l| read_edge(&l.expect("Failed to read line")));

    let mut vertex_index: usize = 0;

    let mut add_vertex = |v: String| {
        *vertex_map.entry(v.clone()).or_insert_with(|| {
            let id = vertex_index;
            vertex_index += 1;
            vertices.insert(id, v);
            id
        })
    };

    for (u, v, w) in edges {
        let u_id = add_vertex(u);
        let v_id = add_vertex(v);

        graph.entry(u_id).or_default().insert(v_id, w);
        graph.entry(v_id).or_default().insert(u_id, w);
    }

    let graph: Arc<Graph> = Arc::new(graph);
    let mut threads: Vec<JoinHandle<(Vec<usize>, f64)>> = Vec::new();

    for start in 0..n {
        let graph = Arc::clone(&graph);
        let handle =
            thread::spawn(move || longest_path_no_end(start, 0.0, &mut bitvec![0; n], &graph));
        threads.push(handle);
    }

    let (path, weight) = threads
        .into_iter()
        .map(|h| h.join().unwrap())
        .reduce(|v1, v2| cmp::max_by(v1, v2, |(_, w1), (_, w2)| w1.total_cmp(w2)))
        .expect("No paths!");

    let path_str = path
        .iter()
        .map(|v| vertices[*v].clone())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{weight}");
    println!("{path_str}");
}
