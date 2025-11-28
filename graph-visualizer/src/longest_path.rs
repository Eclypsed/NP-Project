use std::ops::Add;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use bitvec::prelude::bitvec;
use bitvec::vec::BitVec;
use num_traits::Num;
use petgraph::EdgeType;
use petgraph::{
    Graph,
    graph::{EdgeIndex, NodeIndex},
    visit::EdgeRef,
};
use std::marker::Copy;

trait LongestPath<E> {
    fn longest_from_start(&self, start: NodeIndex) -> (Vec<EdgeIndex>, E);
}

impl<N, E, T> LongestPath<E> for Graph<N, E, T>
where
    E: Num + PartialOrd + Copy,
    T: EdgeType,
{
    fn longest_from_start(&self, start: NodeIndex) -> (Vec<EdgeIndex>, E) {
        let n = self.node_count();
        let mut current_path: Vec<EdgeIndex> = Vec::with_capacity(n - 1);

        longest_from_start_recursive(
            start,
            E::zero(),
            &mut bitvec![0; n],
            &mut current_path,
            self,
        )
    }
}

fn longest_from_start_recursive<N, E, T>(
    start: NodeIndex,
    weight: E,
    visited: &mut BitVec,
    current_path: &mut Vec<EdgeIndex>,
    graph: &Graph<N, E, T>,
) -> (Vec<EdgeIndex>, E)
where
    E: Add<Output = E> + PartialOrd + Copy,
    T: EdgeType,
{
    visited.set(start.index(), true);

    let mut best_path: Vec<EdgeIndex> = vec![];
    let mut best_weight = weight;

    for edge in graph.edges(start) {
        let neighbor = edge.target();
        let n_index = neighbor.index();

        if visited[n_index] {
            continue;
        }

        current_path.push(edge.id());

        let weight = *edge.weight() + weight;
        let (mut new_path, new_weight) =
            longest_from_start_recursive(neighbor, weight, visited, current_path, graph);

        if new_weight > best_weight {
            best_weight = new_weight;
            new_path.insert(0, edge.id());
            best_path = new_path;
        }

        current_path.pop();
        visited.set(n_index, false);
    }

    (best_path, best_weight)
}

pub fn longest_path<N, E, T>(graph: &Graph<N, E, T>, multithread: bool) -> (Vec<EdgeIndex>, E)
where
    N: Clone + Send + Sync + 'static,
    E: Num + PartialOrd + Copy + Send + Sync + 'static,
    T: EdgeType + Send + Sync + 'static,
{
    if multithread {
        let n = graph.node_count();

        let graph_m = Arc::new(graph.clone());
        let mut threads: Vec<JoinHandle<(Vec<EdgeIndex>, E)>> = Vec::with_capacity(n);

        for start in graph.node_indices() {
            let graph_m = Arc::clone(&graph_m);
            let handle = thread::spawn(move || graph_m.longest_from_start(start));
            threads.push(handle);
        }

        threads
            .into_iter()
            .map(|h| h.join().unwrap())
            .max_by(|(_, w1), (_, w2)| w1.partial_cmp(w2).unwrap())
            .unwrap()
    } else {
        graph
            .node_indices()
            .map(|start| graph.longest_from_start(start))
            .max_by(|(_, w1), (_, w2)| w1.partial_cmp(w2).unwrap())
            .unwrap()
    }
}
