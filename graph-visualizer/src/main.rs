mod graph_io;
mod longest_path;

use anyhow::Result;
use petgraph::{Directed, EdgeType, Undirected};
use std::{
    fs,
    io::{self, BufRead},
    path::PathBuf,
};

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    /// Optionally interpret the graph as directed instead of undirected
    #[arg(global = true, long)]
    directed: bool,

    /// The file path of the graph to use
    #[arg()]
    graph: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Calculate the longest path in the graph
    LongestPath(LongestPathArgs),
    /// Read a weight & path from stdin and render the graph with the path highlighted
    Render(RenderArgs),
}

#[derive(Args, Debug)]
struct LongestPathArgs {
    /// Parallelize the computation
    #[arg(long, default_value_t = false)]
    multithread: bool,
}

#[derive(Args, Debug)]
struct RenderArgs {
    /// Repeatedly read and render multiple paths from stdin
    #[arg(long, default_value_t = false)]
    repeat: bool,
}

fn command_runner<T>(graph_path: PathBuf, command: Commands) -> Result<()>
where
    T: EdgeType + Send + Sync + 'static,
{
    let content = fs::read_to_string(graph_path)?;
    let mut lines = content.lines().map(|s| s.to_string());

    let graph = graph_io::read_graph::<String, f64, T>(&mut lines);

    match command {
        Commands::LongestPath(LongestPathArgs { multithread }) => {
            let (path, weight) = longest_path::longest_path(&graph, multithread);

            let path_str = graph_io::edge_path_to_nodes(&path, &graph)
                .iter()
                .map(|n| graph[*n].clone())
                .collect::<Vec<String>>()
                .join(" ");

            println!("{weight}");
            println!("{path_str}");
        }
        Commands::Render(RenderArgs { repeat }) => {
            let mut stdin = io::stdin().lock().lines();

            let read_and_render = |path_line: &str, filename: &str| -> Result<()> {
                let vertices: Vec<String> = path_line
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();

                let path = graph_io::parse_path(&vertices, &graph)?;

                graph_io::draw_graph(&path, &graph, filename);

                Ok(())
            };

            if !repeat {
                let _ = stdin.next().ok_or(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Failed to read weight line",
                ))??;
                // let weight = weight_line.trim().parse::<usize>()?;

                let vertices_line = stdin.next().ok_or(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Failed to read path line",
                ))??;

                read_and_render(&vertices_line, "render.svg")?;
                return Ok(());
            }

            let mut index = 0u64;

            while let (Some(_), Some(vertices_line)) = (stdin.next(), stdin.next()) {
                let path_line = vertices_line?;
                let filename = format!("render_{:05}.svg", index);
                read_and_render(&path_line, filename.as_str())?;
                index += 1;
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.directed {
        true => command_runner::<Directed>(cli.graph, cli.command),
        false => command_runner::<Undirected>(cli.graph, cli.command),
    }
}
