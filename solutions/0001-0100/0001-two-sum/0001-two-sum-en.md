---
title: "0001 Two Sum - EN"
problemUrl: "https://leetcode.com/problems/two-sum/"
difficulty: "Easy"
pubDate: "2026-01-08"
tags: ["array", "hash-table"]
complexity:
  time: "O(n)"
  space: "O(n)"
---

# 0001 Two Sum - EN

## Problem
Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.

You may assume that each input would have **exactly one solution**, and you may not use the same element twice.
You can return the answer in any order.

## Solution
We use a Hash Map to store the complement of each number (`target - num`) as we iterate. If the complement already exists in the map, we have found the pair.

### Implementation in C
In C, we implement a basic hash table with chaining to handle collisions.

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

### Implementation in Rust
In Rust, we leverage `std::collections::HashMap` for a safe and concise solution.

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
