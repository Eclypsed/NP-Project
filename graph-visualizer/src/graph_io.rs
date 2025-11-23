use bitvec::prelude::bitvec;
use petgraph::{
    EdgeType, Graph,
    graph::{EdgeIndex, NodeIndex},
};
use std::{self, collections::HashMap, hash::Hash, str::FromStr};
use thiserror::{self, Error};

#[derive(Error, Debug)]
pub enum ParseEdgeError {
    #[error("Missing vertex")]
    MissingVertex,
    #[error("Failed to parse vertex")]
    VertexParseError,
    #[error("Missing weight")]
    MissingWeight,
    #[error("Failed to parse weight")]
    WeightParseError,
}

type IOedge<N, E> = (N, N, E);

fn read_edge<N: FromStr, E: FromStr>(input: &str) -> Result<IOedge<N, E>, ParseEdgeError> {
    let mut values = input.split_whitespace();

    fn parse<T: FromStr, Err>(
        raw: Option<&str>,
        err_missing: Err,
        err_parse: Err,
    ) -> Result<T, Err> {
        raw.ok_or(err_missing)
            .and_then(|v| v.parse::<T>().map_err(|_| err_parse))
    }

    let v1 = parse::<N, _>(
        values.next(),
        ParseEdgeError::MissingVertex,
        ParseEdgeError::VertexParseError,
    )?;

    let v2 = parse::<N, _>(
        values.next(),
        ParseEdgeError::MissingVertex,
        ParseEdgeError::VertexParseError,
    )?;

    let weight = parse::<E, _>(
        values.next(),
        ParseEdgeError::MissingWeight,
        ParseEdgeError::WeightParseError,
    )?;

    Ok((v1, v2, weight))
}

pub fn read_graph<N, E, T>(
    reader: impl Iterator<Item = std::io::Result<String>>,
    n: usize,
    m: usize,
) -> Graph<N, E, T>
where
    N: FromStr + Clone + Hash + Eq,
    E: FromStr,
    T: EdgeType,
{
    let mut graph: Graph<N, E, T> = Graph::with_capacity(n, m);
    let mut nodes: HashMap<N, NodeIndex> = HashMap::with_capacity(n);

    let edges = reader
        .take(m)
        .map(|l| read_edge::<N, E>(&l.unwrap()).unwrap());

    for (v1, v2, weight) in edges {
        let node1 = *nodes
            .entry(v1.clone())
            .or_insert_with(|| graph.add_node(v1));

        let node2 = *nodes
            .entry(v2.clone())
            .or_insert_with(|| graph.add_node(v2));

        graph.add_edge(node1, node2, weight);
    }

    graph
}

pub fn path_highlighter<N, E, T: EdgeType>(
    path: &Vec<EdgeIndex>,
    graph: &Graph<N, E, T>,
) -> (impl Fn(NodeIndex) -> String, impl Fn(EdgeIndex) -> String) {
    let n = graph.node_count();
    let m = graph.edge_count();

    let mut vertices = bitvec![0; n];
    let mut edges = bitvec![0; m];

    for edge in path {
        edges.set(edge.index(), true);

        let Some((source, target)) = graph.edge_endpoints(*edge) else {
            continue;
        };

        vertices.set(source.index(), true);
        vertices.set(target.index(), true);
    }

    let node_coloring = move |n: NodeIndex| {
        if vertices[n.index()] {
            return "#0000ff".to_string();
        }

        "#000000".to_string()
    };

    let edge_coloring = move |e: EdgeIndex| {
        if edges[e.index()] {
            return "#0000ff".to_string();
        }

        "#000000".to_string()
    };

    (node_coloring, edge_coloring)
}
