# Graph Visualizer

A command line tool for calculating the longest path in a weighted graph as well
as generating SVG visualizations of paths in a graph.

## Building the project

Run `cargo build --release` to build the binary in
`target/release/graph-visualizer`

## Usage

Run `./graph-visualizer --help` for usage.

### Examples

The following are some examples of inputs and outputs you might expect when
running _Command_ on graph [lp_001.txt](../test_cases/lp_001.txt).

<table>
    <tr>
        <td>Command</td>
        <td>Input</td>
        <td>Ouput</td>
    </tr>
    <tr>
        <td><pre>longest-path</pre></td>
        <td>N/A</td>
        <td><pre>115<br>v5 v4 v7 v8 v3 v1 v6 v2</pre></td>
    </tr>
    <tr>
        <td><pre>render</pre></td>
        <td><pre>115<br>v5 v4 v7 v8 v3 v1 v6 v2</pre></td>
        <td><img src="./assets/lp_001.svg" alt="lp_001 render"></td>
    </tr>
</table>

Because the `render` command expects the same input that `longest-path` outputs,
the two can easilly be piped together:

```shell
./graph-visualizer lp_001.txt longest-path | \
./graph-visualizer lp_001.txt render
```

Or with another path generating program:

```shell
python cs412_longestpath_exact.py < lp_001.txt | \
./graph-visualizer lp_001.txt render
```
