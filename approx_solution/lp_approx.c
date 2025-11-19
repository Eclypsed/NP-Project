#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <float.h>
#include <time.h>

#include "graph.h"

#define MAX_TIME 2.5

#define NOW() ({ \
        struct timespec ts__; \
        clock_gettime(CLOCK_MONOTONIC, &ts__); \
        ts__.tv_sec + ts__.tv_nsec / 1e9; \
        })


static float longest_path_greedy(graph *g, size_t start,
        size_t *out_path, size_t *out_len)
{
    int *visited = calloc(g->n, sizeof(int));
    size_t *path = out_path;
    size_t len = 0;

    size_t current = start;
    visited[current] = 1;
    path[len++] = current;

    float total = 0.0f;

    while (1) {
        edge *best = NULL;

        for (edge *e = g->adj[current]; e; e = e->next) {
            if (!visited[e->to]) {
                if (!best || e->weight > best->weight) {
                    best = e;
                }
            }
        }

        if (!best)
            break;

        total += best->weight;
        current = best->to;
        visited[current] = 1;
        path[len++] = current;
    }

    *out_len = len;
    free(visited);
    return total;
}

static void run_sim(graph *g) {
    size_t *path = malloc(g->n * sizeof(size_t));
    size_t *best_path = malloc(g->n * sizeof(size_t));
    size_t best_len = 0;

    float best_weight = -FLT_MAX;
    int improved = 1;

    double start_time = NOW();

    while (NOW() - start_time < MAX_TIME && improved) {
        improved = 0;

        for (size_t i = 0; i < g->n; i++) {
            size_t len;
            float w = longest_path_greedy(g, i, path, &len);

            if (w > best_weight) {
                best_weight = w;
                best_len = len;

                for (size_t k = 0; k < len; k++)
                    best_path[k] = path[k];

                improved = 1;
            }
        }
    }

    printf("%.0f\n", best_weight);
    for (size_t i = 0; i < best_len; i++) {
        printf("%s%s", g->names[best_path[i]],
                (i + 1 < best_len) ? " " : "");
    }
    printf("\n");

    free(path);
    free(best_path);
}

int main(void) {
    graph *g = graph_read();
    if (!g) {
        fprintf(stderr, "Failed to read graph.\n");
        return 1;
    }

    run_sim(g);
    graph_free(g);
    return 0;
}

