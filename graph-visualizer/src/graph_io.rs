use bitvec::prelude::bitvec;
use petgraph::{
    EdgeType, Graph,
    graph::{EdgeIndex, NodeIndex},
};
use std::fmt::Debug;
use std::{self, collections::HashMap, hash::Hash, str::FromStr};
use thiserror::{self, Error};
use visgraph::{graph_to_svg, layout::Layout, settings::SettingsBuilder};

#[derive(Error, Debug)]
enum ParseEdgeError {
    #[error("Missing token while trying to read {0}")]
    MissingToken(&'static str),
    #[error("Failed to parse {value} as {field}")]
    ParseError { value: String, field: &'static str },
}

type IOedge<N, E> = (N, N, E);

fn read_edge<N: FromStr, E: FromStr>(input: &str) -> Result<IOedge<N, E>, ParseEdgeError> {
    let mut values = input.split_whitespace();

    fn parse_field<T: FromStr>(s: Option<&str>, name: &'static str) -> Result<T, ParseEdgeError> {
        let raw = s.ok_or(ParseEdgeError::MissingToken(name))?;
        raw.parse().map_err(|_| ParseEdgeError::ParseError {
            value: raw.to_string(),
            field: name,
        })
    }

    let v1 = parse_field::<N>(values.next(), "Vertex 1")?;
    let v2 = parse_field::<N>(values.next(), "Vertex 2")?;
    let weight = parse_field::<E>(values.next(), "Weight")?;

    Ok((v1, v2, weight))
}

const MIN_PATH_LEN: usize = 2;

#[derive(Error, Debug)]
pub enum ParsePathError<V: Debug> {
    #[error("Invalid vertex {0:?} not in graph")]
    InvalidVertex(V),
    #[error("Vertex {0:?} visited more than once")]
    DuplicateVertex(V),
    #[error("Edge {0:?}->{1:?} does not exist")]
    NonexistentEdge(V, V),
    #[error("Expect at least {min} vertices in path, received {0}", min = MIN_PATH_LEN)]
    EmptyPath(usize),
}

pub fn parse_path<N, E, T>(
    path: &[N],
    graph: &Graph<N, E, T>,
) -> Result<Vec<EdgeIndex>, ParsePathError<N>>
where
    N: Eq + Hash + Clone + Debug,
    T: EdgeType,
{
    let path_len = path.len();
    if path_len < MIN_PATH_LEN {
        return Err(ParsePathError::EmptyPath(path_len));
    }

    let mut visited = bitvec![0; graph.node_count()];
    let mut vertex_map: HashMap<N, NodeIndex> = HashMap::with_capacity(graph.node_count());

    for node in graph.node_indices() {
        vertex_map.insert(graph[node].clone(), node);
    }

    let mut validate_vertex = |v: &N| -> Result<&NodeIndex, ParsePathError<N>> {
        let node = vertex_map
            .get(v)
            .ok_or(ParsePathError::InvalidVertex(v.clone()))?;

        if visited[node.index()] {
            return Err(ParsePathError::DuplicateVertex(v.clone()));
        } else {
            visited.set(node.index(), true);
        }

        Ok(node)
    };

    let mut nodes: Vec<&NodeIndex> = Vec::with_capacity(path_len);
    for node in path {
        let index = validate_vertex(node)?;
        nodes.push(index);
    }

    let mut parsed_path: Vec<EdgeIndex> = Vec::with_capacity(path_len - 1);

    for pair in nodes.windows(2) {
        let (a, b) = (pair[0], pair[1]);

        let edge = graph
            .find_edge(*a, *b)
            .ok_or(ParsePathError::NonexistentEdge(
                graph[*a].clone(),
                graph[*b].clone(),
            ))?;

        parsed_path.push(edge);
    }

    Ok(parsed_path)
}

pub fn read_graph<N, E, T>(reader: &mut impl Iterator<Item = String>) -> Graph<N, E, T>
where
    N: FromStr + Clone + Hash + Eq,
    E: FromStr,
    T: EdgeType,
{
    let line1 = reader.next().unwrap();

    let mut nums = line1
        .split_whitespace()
        .take(2)
        .map(|s| s.parse::<usize>().unwrap());

    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    let mut graph: Graph<N, E, T> = Graph::with_capacity(n, m);
    let mut nodes: HashMap<N, NodeIndex> = HashMap::with_capacity(n);

    let edges = reader
        .by_ref()
        .take(m)
        .map(|l| read_edge::<N, E>(&l).unwrap());

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

pub fn edge_path_to_nodes<N, E, T: EdgeType>(
    path: &[EdgeIndex],
    graph: &Graph<N, E, T>,
) -> Vec<NodeIndex> {
    let mut nodes = Vec::with_capacity(path.len() + 1);

    let Some(first) = path.first() else {
        return nodes;
    };

    let (a, b) = graph.edge_endpoints(*first).unwrap();

    let start = if path.len() == 1 {
        a
    } else {
        let (c, d) = graph.edge_endpoints(path[1]).unwrap();

        if a == c || a == d { b } else { a }
    };

    nodes.push(start);

    let mut prev = start;

    for edge in path {
        let (x, y) = graph.edge_endpoints(*edge).unwrap();
        let next = if x == prev { y } else { x };
        nodes.push(next);
        prev = next;
    }

    nodes
}

fn path_highlighter<N, E, T: EdgeType>(
    path: &[EdgeIndex],
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

pub fn draw_graph<N, E, T: EdgeType>(path: &[EdgeIndex], graph: &Graph<N, E, T>, filename: &str) {
    let (node_coloring, edge_coloring) = path_highlighter(path, graph);

    // lp_030 gives a good visual
    let settings = SettingsBuilder::new()
        .layout(Layout::Circular)
        .edge_coloring_fn(edge_coloring)
        .node_coloring_fn(node_coloring)
        .build()
        .unwrap();

    let _ = graph_to_svg(&graph, &settings, filename);
}
