---
title: "Two Sum"
problemUrl: "https://leetcode.com/problems/two-sum/"
difficulty: "Easy"
pubDate: "2026-01-08"
tags: ["array", "hash-table"]
complexity:
  time: "O(n)"
  space: "O(n)"
---

# Two Sum

## Problema
Dado un array de enteros `nums` y un entero `target`, retorna los índices de los dos números tales que sumen `target`.

Puedes asumir que cada entrada tendría **exactamente una solución**, y no puedes usar el mismo elemento dos veces.
Puedes devolver la respuesta en cualquier orden.

## Solución
Utilizamos un Hash Map para almacenar el complemento de cada número (`target - num`) mientras iteramos. Si el complemento ya existe en el mapa, hemos encontrado la pareja.

### Implementación en C
En C, implementamos una tabla hash básica con encadenamiento (chaining) para manejar colisiones.

```c
/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
#include <stdlib.h>
#define HASH_SIZE 20003

int hash(int key) { return abs(key) % HASH_SIZE; }

typedef struct Node {
    int key;
    int index;
    struct Node *next;
} Node;

typedef struct {
    Node *buckets[HASH_SIZE];
} HashTable;

// Insert a tuple { value, index}
void insert(HashTable *table, int key, int index) {
    int h = hash(key);
    Node *newNode = (Node *)malloc(sizeof(Node));
    newNode->key = key;
    newNode->index = index;

    // Insert in the beginning of the linked list
    newNode->next = table->buckets[h];
    table->buckets[h] = newNode;
}

int search(HashTable *table, int key) {
    int h = hash(key);
    Node *c = table->buckets[h];
    while (c != NULL) {
        if (c->key == key) {
            return c->index;
        }
        c = c->next;
    }
    return -1;
}

void freeTable(HashTable *table) {
    for (int i = 0; i < HASH_SIZE; i++) {
        Node *current = table->buckets[i];
        while (current != NULL) {
            Node *temp = current;
            current = current->next;
            free(temp);
        }
        table->buckets[i] = NULL;
    }
}

int *twoSum(int *nums, int numsSize, int target, int *returnSize) {
    HashTable table;
    for (int i = 0; i < HASH_SIZE; i++)
        table.buckets[i] = NULL;

    for (int i = 0; i < numsSize; i++) {
        int complement = target - nums[i];
        int foundIndex = search(&table, complement);

        if (foundIndex != -1 && i != foundIndex) {
            int *sol = (int *)malloc(2 * sizeof(int));

            sol[0] = foundIndex;
            sol[1] = i;
            *returnSize = 2;

            freeTable(&table);
            return sol;
        }
        insert(&table, nums[i], i);
    }

    *returnSize = 0;
    freeTable(&table);
    return NULL;
}
```

### Implementación en Rust
En Rust, aprovechamos `std::collections::HashMap` para una solución segura y concisa.

```rust
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = map.get(&complement) {
                return vec![index as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}
```
