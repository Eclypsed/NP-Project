#include <stdio.h>
#include <stdlib.h>
#include <float.h>
#include <time.h>
#include <stdint.h>
#include <string.h>

#include "graph.h"

static inline uint32_t prng(void) { return rand(); }
static inline float frand(void) { return (float)prng() / (float)RAND_MAX; }
static inline double now(void) { return (double)clock() / CLOCKS_PER_SEC; }
static inline char *shift(int *argc, char ***argv) {
    if (*argc == 0)
        return NULL;

    char *arg = **argv;
    (*argv)++;
    (*argc)--;
    return arg;
}

// DEFAULTS
static double MAX_TIME = 60;
static float JUMP_PROB = 0.15f;

// Forward declarations
static void collect_moves(graph *, size_t, int *, edge **, size_t *, edge **, size_t *);
static edge *choose_edge(edge **, size_t, edge **, size_t);
static float sample_path(graph *, size_t, size_t *, size_t *);
static void run_sim(graph *);
void handle_args(int *, char ***);

// Entry
int main(int argc, char** argv)
{
    srand(time(NULL));
    handle_args(&argc, &argv);

    graph *g = graph_read();
    if (!g) {
        fprintf(stderr, "Failed to read graph.\n");
        return 1;
    }

    run_sim(g);

    graph_free(g);
    return 0;
}

// Collect unvisited neighbors and identify the best-weight ones for greedy selection.
static void collect_moves(graph *g, size_t current, int *visited, edge **neighbors, size_t *neighbor_count, edge **best_edges, size_t *best_edge_count) {
    float best_w = -1.0f;
    *neighbor_count = 0;
    *best_edge_count = 0;

    for (edge *e = g->adj[current]; e; e = e->next) {
        if (!visited[e->to]) {
            neighbors[(*neighbor_count)++] = e;

            if (e->weight > best_w) {
                best_w = e->weight;
                *best_edge_count = 1;
                best_edges[0] = e;
            } else if (e->weight == best_w) {
                best_edges[(*best_edge_count)++] = e;
            }
        }
    }
}

// Choose the next move: random jump or a random pick among best-weight edges.
static edge *choose_edge(edge **neighbors, size_t neighbor_count, edge **best_edges, size_t best_edge_count) {
    if (best_edge_count == 0)
        return NULL;

    /* Random jump to any edge */
    if (neighbor_count > 1 && frand() < JUMP_PROB)
        return neighbors[prng() % neighbor_count];

    /* Otherwise choose randomly among best edges */
    return best_edges[prng() % best_edge_count];
}

// Build a single randomized greedy path with jumps
static float sample_path(graph *g, size_t start, size_t *path, size_t *out_len)
{
    int *visited = calloc(g->n, sizeof(int));
    if (!visited) {
        fprintf(stderr, "Error: Failed to allocate visited array\n");
        exit(1);
    }

    size_t len = 0;
    size_t current = start;
    visited[current] = 1;
    path[len++] = current;

    float total = 0.0f;

    edge **neighbors = (edge**) calloc(g->n, sizeof(edge*));
    edge **best_edges = (edge**) calloc(g->n, sizeof(edge*));
    if (!neighbors || !best_edges) {
        fprintf(stderr, "Error: Failed to allocate edge arrays\n");
        free(visited);
        free(neighbors);
        free(best_edges);
        exit(1);
    }
    size_t neighbor_count, best_edge_count;

    // Main greedy loop with random jumps
    while (1) {
        collect_moves(g, current, visited, neighbors, &neighbor_count, best_edges, &best_edge_count);

        if (best_edge_count == 0) break;

        edge *chosen = choose_edge(neighbors, neighbor_count, best_edges, best_edge_count);
        if (!chosen) break;

        total += chosen->weight;
        current = chosen->to;
        visited[current] = 1;
        path[len++] = current;
    }

    free(visited);
    free(neighbors);
    free(best_edges);
    *out_len = len;
    return total;
}

// Repeatedly sample random paths within the time limit and keep the highest-weight one found.
static void run_sim(graph *g)
{
    size_t *path = (size_t*) calloc(g->n, sizeof(size_t));
    size_t *best_path = (size_t*) calloc(g->n, sizeof(size_t));

    if (!path || !best_path) {
        fprintf(stderr, "Error: Failed to allocate path arrays\n");
        free(path);
        free(best_path);
        exit(1);
    }

    size_t best_len = 0;
    float best_weight = -1.0f;

    double last_improve_time = now();

    while (now() - last_improve_time < MAX_TIME) {
        size_t start_node = prng() % g->n;

        size_t len;
        float w = sample_path(g, start_node, path, &len);

        if (w > best_weight) {
            best_weight = w;
            best_len = len;

            for (size_t i = 0; i < len; i++)
                best_path[i] = path[i];

            last_improve_time = now();
        }
    }

    // Output result
    printf("%.0f\n", best_weight);
    for (size_t i = 0; i < best_len; i++)
        printf("%s%s", g->names[best_path[i]], (i + 1 < best_len) ? " " : "");
    printf("\n");

    free(path);
    free(best_path);
}


void handle_args(int* argc, char*** argv){
    char *__attribute__((unused))program = shift(argc, argv);
    while (*argc > 0) {
        char *arg = shift(argc, argv);

        if (strcmp(arg, "--time") == 0) {
            if (*argc > 0) {
                char *next = (*argv)[0];
                char *end;

                float val = strtof(next, &end);

                if (end != next) {
                    MAX_TIME = val;
                    shift(argc, argv);
                }
            }
            continue;
        }
    }
}
