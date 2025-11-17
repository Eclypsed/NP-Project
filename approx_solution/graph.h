// Basic graph structure using adjacency lists.
// Each vertex stores a linked list of outgoing edges.

#ifndef GRAPH_H
#define GRAPH_H

#include <stdlib.h>

// One edge in the adjacency list: destination, weight, and next pointer.
typedef struct edge {
    size_t to;
    float weight;
    struct edge* next;
} edge;

// Graph structure: number of vertices,
// adjacency lists, and optional vertex names.
typedef struct graph {
    size_t n;
    edge **adj;
    char **names;
} graph;

// Read a graph from standard input.
graph *graph_read(void);

// Free all memory used by the graph.
void graph_free(graph *g);

#endif // GRAPH_H


