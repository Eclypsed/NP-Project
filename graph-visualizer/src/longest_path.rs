use std::ops::Add;

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

pub fn longest_from_start<N, E, T>(start: NodeIndex, graph: &Graph<N, E, T>) -> (Vec<EdgeIndex>, E)
where
    E: Num + PartialOrd + Copy,
    T: EdgeType,
{
    let n = graph.node_count();
    longest_from_start_recursive(start, E::zero(), &mut bitvec![0; n], graph)
}

fn longest_from_start_recursive<N, E: Add<Output = E> + PartialOrd + Copy, T: EdgeType>(
    start: NodeIndex,
    weight: E,
    visited: &mut BitVec,
    graph: &Graph<N, E, T>,
) -> (Vec<EdgeIndex>, E) {
    visited.set(start.index(), true);

    let mut best_path: Vec<EdgeIndex> = vec![];
    let mut best_weight = weight;

    for edge in graph.edges(start) {
        let neighbor = edge.target();
        let n_index = neighbor.index();

        if visited[n_index] {
            continue;
        }

        let weight = *edge.weight() + weight;
        let (mut new_path, new_weight) =
            longest_from_start_recursive(neighbor, weight, visited, graph);

        if new_weight > best_weight {
            best_weight = new_weight;
            new_path.insert(0, edge.id());
            best_path = new_path;
        }

        visited.set(n_index, false);
    }

    (best_path, best_weight)
}
