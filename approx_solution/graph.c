#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "graph.h"
#define HASHMAP_IMPLEMENTATION
#include "hashmap.h"

#define BUF_SIZE 128

/* Simple strdup replacement for C99 */
static char *c99_strdup(const char *s) {
    size_t len = strlen(s) + 1;
    char *copy = malloc(len);
    if (!copy) return NULL;
    memcpy(copy, s, len);
    return copy;
}

/* Look up a vertex name; assign a new ID if not present */
static size_t get_vertex_id(hashmap *map, const char *name, size_t *next_id) {
    size_t *found = hashmap_get(map, name);
    if (found) return *found;

    size_t id = *next_id;
    hashmap_insert(map, name, &id);
    (*next_id)++;
    return id;
}

graph *graph_read(void) {
    int n, m;
    if (scanf("%d %d", &n, &m) != 2)
        return NULL;

    graph *g = calloc(1, sizeof(graph));

    /* Initial adjacency list capacity */
    size_t capacity = (size_t)n * 2 + 4;
    g->adj = calloc(capacity, sizeof(edge*));

    /* Map vertex names to integer IDs */
    hashmap map;
    hashmap_init(&map, capacity, sizeof(size_t));

    size_t next_id = 0;
    char u[BUF_SIZE], v[BUF_SIZE];
    float w;

    /* Read all edges */
    for (int i = 0; i < m; i++) {
        if (scanf("%127s %127s %f", u, v, &w) != 3)
            break;

        size_t from = get_vertex_id(&map, u, &next_id);
        size_t to   = get_vertex_id(&map, v, &next_id);

        /* Expand adjacency list array if needed */
        if (from >= capacity || to >= capacity) {
            size_t old_cap = capacity;
            capacity = capacity * 2 + 4;
            g->adj = realloc(g->adj, capacity * sizeof(edge*));
            for (size_t j = old_cap; j < capacity; j++)
                g->adj[j] = NULL;
        }

        /* Forward edge */
        edge *e1 = calloc(1, sizeof(edge));
        e1->to = to;
        e1->weight = w;
        e1->next = g->adj[from];
        g->adj[from] = e1;

        /* Reverse edge (undirected graph) */
        edge *e2 = calloc(1, sizeof(edge));
        e2->to = from;
        e2->weight = w;
        e2->next = g->adj[to];
        g->adj[to] = e2;
    }

    g->n = next_id;

    /* Store vertex names by ID */
    g->names = calloc(g->n, sizeof(char*));
    for (size_t i = 0; i < map.capacity; i++) {
        if (map.entries[i].in_use) {
            const char *key = map.entries[i].key;
            size_t id = *(size_t*)map.entries[i].value;
            g->names[id] = c99_strdup(key);
        }
    }

    hashmap_free(&map);

    /* Shrink adjacency list to exact size */
    g->adj = realloc(g->adj, g->n * sizeof(edge*));

    return g;
}

void graph_free(graph *g) {
    if (!g) return;

    /* Free all adjacency lists */
    for (size_t i = 0; i < g->n; i++) {
        edge *cur = g->adj[i];
        while (cur) {
            edge *next = cur->next;
            free(cur);
            cur = next;
        }
        free(g->names[i]);
    }

    free(g->names);
    free(g->adj);
    free(g);
}


