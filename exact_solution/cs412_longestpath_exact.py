from collections import defaultdict
from functools import reduce

type Graph = dict[int, dict[int, float]]


def longest_path(n: int, graph: Graph) -> tuple[list[int], float]:

    def recurse(
        start: int, weight: float, visited: list[bool]
    ) -> tuple[list[int], float]:
        visited[start] = True

        if start not in graph:
            return ([start], weight)

        best_path: list[int] = [start]
        best_weight: float = weight

        for adj in graph[start].keys():
            if visited[adj]:
                continue

            res = recurse(adj, weight + graph[start][adj], visited)
            visited[adj] = False

            new_path, new_weight = res

            if new_weight > best_weight:
                best_weight = new_weight
                best_path = [start, *new_path]

        return (best_path, best_weight)

    return reduce(
        lambda x, y: x if x[1] > y[1] else y,
        map(lambda start: recurse(start, 0, [False] * n), range(n)),
    )


def main():
    first_line = input().split()
    n = int(first_line[0])
    m = int(first_line[1])

    graph: Graph = defaultdict(dict)

    vertices: list[str] = [""] * n
    vertex_map: dict[str, int] = {}
    vertex_index = 0

    for _ in range(0, m):
        u, v, w = input().split()

        if u not in vertex_map:
            vertex_map[u] = vertex_index
            vertices[vertex_index] = u
            vertex_index += 1
        if v not in vertex_map:
            vertex_map[v] = vertex_index
            vertices[vertex_index] = v
            vertex_index += 1

        weight = float(w)

        graph[vertex_map[v]][vertex_map[u]] = weight
        graph[vertex_map[u]][vertex_map[v]] = weight

    path, weight = longest_path(n, graph)

    print(weight)
    print(" ".join(map(lambda id: vertices[id], path)))


if __name__ == "__main__":
    main()
