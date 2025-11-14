from collections import defaultdict
from math import inf
from pprint import pprint

type Graph = dict[int, dict[int, float]]


def longest_path(n: int, graph: Graph):

    memo: list[list[float]] = [([-inf] * n) for _ in range(0, n)]

    pass


def main():
    first_line = input().split()
    n = int(first_line[0])
    m = int(first_line[1])

    graph: Graph = defaultdict(dict)

    vertex_map: dict[str, int] = {}
    vertex_index: int = 0

    for _ in range(0, m):
        u, v, w = input().split()

        if u not in vertex_map:
            vertex_map[u] = vertex_index
            vertex_index += 1
        if v not in vertex_map:
            vertex_map[v] = vertex_index
            vertex_index += 1

        weight = float(w)

        graph[vertex_map[u]][vertex_map[v]] = weight

    longest_path(n, graph)


if __name__ == "__main__":
    main()
