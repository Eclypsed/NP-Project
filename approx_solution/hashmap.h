// This header defines a simple hashmap data structure.
// Conceptually similar to a Python dictionary: it stores key–value pairs,
// supports insertion and lookup, and uses a hash function to find slots.

#ifndef HASHMAP_H
#define HASHMAP_H

#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

// One entry in the hashmap: a key, a value pointer, and a flag to mark usage.
typedef struct hashmap_entry {
    const char *key;
    void *value;
    int in_use;
} hashmap_entry;

// The hashmap structure itself: an array of entries plus metadata.
typedef struct hashmap {
    hashmap_entry *entries;
    size_t capacity;
    size_t count;
    size_t value_size;
} hashmap;

// FNV-1a hash function for hashing strings.
static uint64_t hashmap_hash_fnv1a(const char *str) {
    uint64_t hash = 1469598103934665603ULL;
    while (*str) {
        hash ^= (unsigned char)(*str++);
        hash *= 1099511628211ULL;
    }
    return hash;
}

// Function declarations
static void hashmap_init(hashmap *map, size_t capacity, size_t value_size);
static void hashmap_free(hashmap *map);
static void hashmap_insert(hashmap *map, const char *key, const void *value);
static void *hashmap_get(hashmap *map, const char *key);

#endif // HASHMAP_H


// The implementation below becomes active only if HASHMAP_IMPLEMENTATION
// is defined before including this header.
#ifdef HASHMAP_IMPLEMENTATION

static void hashmap_init(hashmap *map, size_t capacity, size_t value_size) {
    map->capacity = capacity;
    map->count = 0;
    map->value_size = value_size;
    map->entries = (hashmap_entry *)calloc(capacity, sizeof(hashmap_entry));
}

static void hashmap_free(hashmap *map) {
    if (!map->entries) return;
    for (size_t i = 0; i < map->capacity; i++) {
        if (map->entries[i].in_use) {
            free((void*)map->entries[i].key);
            free(map->entries[i].value);
        }
    }
    free(map->entries);
    map->entries = NULL;
    map->capacity = 0;
    map->count = 0;
}

static void hashmap_insert(hashmap *map, const char *key, const void *value) {
    uint64_t h = hashmap_hash_fnv1a(key);
    size_t i = h % map->capacity;

    // Linear probing to find an open slot or matching key.
    while (map->entries[i].in_use) {
        if (strcmp(map->entries[i].key, key) == 0) {
            memcpy(map->entries[i].value, value, map->value_size);
            return;
        }
        i = (i + 1) % map->capacity;
    }

    map->entries[i].in_use = 1;

    // Copy key
    size_t len = strlen(key) + 1;
    char *copy = (char*)calloc(len, 1);
    snprintf(copy, len, "%s", key);
    map->entries[i].key = copy;

    // Copy value
    void *val = calloc(map->value_size, 1);
    memcpy(val, value, map->value_size);
    map->entries[i].value = val;

    map->count++;
}

static void *hashmap_get(hashmap *map, const char *key) {
    uint64_t h = hashmap_hash_fnv1a(key);
    size_t i = h % map->capacity;

    // Linear probing lookup
    while (map->entries[i].in_use) {
        if (strcmp(map->entries[i].key, key) == 0)
            return map->entries[i].value;
        i = (i + 1) % map->capacity;
    }

    return NULL;
}

#endif // HASHMAP_IMPLEMENTATION

