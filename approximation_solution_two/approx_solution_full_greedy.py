"""
    name:  Patrick Jabalee-Farrell

    Honor Code and Acknowledgments:

            This work complies with the JMU Honor Code.

           Comments here on your code and submission.
"""
import random
import sys

def approx_solution(graph):
    final_path = []
    total_weight = 0

    for vertex in graph:
        current_weight = 0
        current_path = [vertex]
        visited_vertices = set()
        current_vertex = vertex
        for neighbor, weight in graph[current_vertex].items():

            if (neighbor in visited_vertices):
                continue
            if weight > current_weight:
                current_path.append(neighbor)
                current_weight += weight
                visited_vertices.add(neighbor)
                current_vertex = neighbor

        if current_weight > total_weight:
            total_weight = current_weight
            final_path = current_path

    return total_weight, final_path


# All modules for CS 412 must include a main method that allows it
# to imported and invoked from other python scripts
def main():
    line = sys.stdin.readline().strip().split()
    n, m = int(line[0]), int(line[1])

    graph = {}

    for _ in range(m):
        u, v, w = sys.stdin.readline().strip().split()
        w = float(w)

        if u not in graph:
            graph[u] = {}
        if v not in graph:
            graph[v] = {}

        graph[u][v] = w
        graph[v][u] = w

    total_weight, final_path = approx_solution(graph)
    print(f"{total_weight}")
    
    print(" ".join(final_path))

if __name__ == "__main__":
    main()