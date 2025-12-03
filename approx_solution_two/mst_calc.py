import sys

# Disjoint Set (Union-Find) with path compression + union by rank
class DSU:
    def __init__(self, n):
        self.parent = list(range(n))
        self.rank = [0] * n

    def find(self, x):
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, x, y):
        xr, yr = self.find(x), self.find(y)
        if xr == yr:
            return False
        if self.rank[xr] < self.rank[yr]:
            self.parent[xr] = yr
        elif self.rank[xr] > self.rank[yr]:
            self.parent[yr] = xr
        else:
            self.parent[yr] = xr
            self.rank[xr] += 1
        return True


def main():
    # Read n and m
    line = sys.stdin.readline().split()
    n, m = int(line[0]), int(line[1])

    # Map vertex labels (like "v21") → integers 0..n-1
    index = {}
    edges = []

    next_id = 0

    for _ in range(m):
        u, v, w = sys.stdin.readline().split()
        w = float(w)

        if u not in index:
            index[u] = next_id
            next_id += 1
        if v not in index:
            index[v] = next_id
            next_id += 1

        # store as (weight, u_index, v_index)
        edges.append((w, index[u], index[v]))

    # Sort edges by descending weight for maximum spanning tree
    edges.sort(reverse=True)

    dsu = DSU(n)
    mst_weight = 0.0
    edges_used = 0

    # Kruskal
    for w, u, v in edges:
        if dsu.union(u, v):
            mst_weight += w
            edges_used += 1
            if edges_used == n - 1:
                break

    print(mst_weight)


if __name__ == "__main__":
    main()
