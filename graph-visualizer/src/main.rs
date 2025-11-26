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
    #[arg(global = true, long)]
    directed: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    LongestPath(LongestPathArgs),
    Render(RenderArgs),
}

#[derive(Args, Debug)]
struct LongestPathArgs {
    graph: PathBuf,

    #[arg(long, default_value_t = false)]
    multithread: bool,
}

#[derive(Args, Debug)]
struct RenderArgs {
    graph: PathBuf,

    #[arg(long, default_value_t = false)]
    repeat: bool,
}

fn run_longest_path<T>(LongestPathArgs { graph, multithread }: LongestPathArgs)
where
    T: EdgeType + Send + Sync + 'static,
{
    let content = fs::read_to_string(graph).unwrap();
    let mut lines = content.lines().map(|s| s.to_string());

    let graph = graph_io::read_graph::<String, f64, T>(&mut lines);

    let (path, weight) = longest_path::longest_path(&graph, multithread);

    let path_str = graph_io::edge_path_to_nodes(&path, &graph)
        .iter()
        .map(|n| graph[*n].clone())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{weight}");
    println!("{path_str}");
}

fn run_render<T: EdgeType>(RenderArgs { graph, repeat }: RenderArgs) -> Result<()> {
    let content = fs::read_to_string(graph)?;
    let mut lines = content.lines().map(|s| s.to_string());

    let graph = graph_io::read_graph::<String, f64, T>(&mut lines);

    let mut stdin = io::stdin().lock().lines();

    let mut read_and_render = |filename: &str| -> Result<()> {
        let _ = stdin.next().ok_or(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Failed to read weight line",
        ))??;
        // let weight = weight_line.trim().parse::<usize>()?;

        let vertices_line = stdin.next().ok_or(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Failed to read path line",
        ))??;
        let vertices: Vec<String> = vertices_line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let path = graph_io::parse_path(&vertices, &graph)?;

        graph_io::draw_graph(&path, &graph, filename);

        Ok(())
    };

    if !repeat {
        read_and_render("render.svg")?;
        return Ok(());
    }

    let mut index = 0u64;
    loop {
        let filename = format!("render_{:05}.svg", index);
        read_and_render(filename.as_str())?;
        index += 1;
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::LongestPath(args) => match cli.directed {
            true => run_longest_path::<Directed>(args),
            false => run_longest_path::<Undirected>(args),
        },
        Commands::Render(args) => match cli.directed {
            true => run_render::<Directed>(args)?,
            false => run_render::<Undirected>(args)?,
        },
    };

    Ok(())
}
