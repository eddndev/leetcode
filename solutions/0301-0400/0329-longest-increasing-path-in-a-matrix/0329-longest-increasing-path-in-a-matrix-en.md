---
title: "0329 Longest Increasing Path in a Matrix - EN"
problemUrl: "https://leetcode.com/problems/longest-increasing-path-in-a-matrix/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "dfs", "memoization", "matrix", "graph"]
complexity:
  time: "O(M * N) where M and N are the dimensions of the matrix"
  space: "O(M * N)"
---

# Rivers Carving Through the Grid

## The Problem
Given an `m x n` integers matrix, return the length of the longest increasing path. From each cell, you can move in four directions: left, right, up, or down. You may not move diagonally or move outside the boundary. Each step must go to a cell with a strictly greater value.

## The Initial Intuition

At first glance, this looks like a graph problem where every cell is a node and edges connect to neighbors with strictly larger values. I need the longest path in this directed acyclic graph. The "acyclic" part is crucial -- since every step must go to a strictly greater value, I can never revisit a cell, which means cycles are impossible.

This DAG structure immediately suggests DFS with memoization. If I already know the longest increasing path starting from some cell `(i, j)`, I should never recompute it. Each cell's answer depends only on its neighbors with larger values, and those dependencies form a clean tree-like structure with no circular dependencies.

## Why Memoization Works Perfectly

The key observation is that the "strictly increasing" constraint gives us a natural topological ordering. If `matrix[a][b] < matrix[i][j]`, then the answer for `(a, b)` can never depend on the answer for `(i, j)`, because you can't go from a larger value to a smaller one. This means when I compute `dfs(i, j)`, all the recursive calls I make are for cells with strictly larger values, and their results are either already cached or will be computed without ever circling back to `(i, j)`.

Without memoization, the brute force DFS would revisit cells exponentially many times. Consider a grid like:

```
1  2  3
6  5  4
7  8  9
```

The path from cell `1` passes through cell `2`, which also gets explored independently. Cell `4` gets reached from both `3` and `5`. The overlap cascades. But with memoization, each cell is fully computed exactly once, giving us `O(M * N)` total work.

## The DFS Structure

For each cell `(i, j)`, I explore all four neighbors. If a neighbor `(ni, nj)` is within bounds and has a strictly larger value, I recursively find the longest path starting from that neighbor and add 1. The answer for `(i, j)` is the maximum across all valid neighbors, with a base case of 1 (just the cell itself, when no neighbor is larger).

I store results in a `cache` matrix initialized to zeros. A non-zero entry means the cell has already been computed. This dual-purpose trick avoids needing a separate "visited" array -- since every cell's answer is at least 1, a zero unambiguously means "not yet computed."

## The Outer Loop

The longest increasing path could start from any cell, so I launch DFS from every cell in the matrix, keeping track of the global maximum. Thanks to memoization, most of these calls return instantly from the cache. The total work across all calls is still `O(M * N)` because each cell's DFS body executes exactly once.

## Rust Solution

```rust
use std::cmp;

impl Solution {
    const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let m = matrix.len();
        let n = matrix[0].len();

        let mut cache = vec![vec![0; n]; m];
        let mut max_len = 0;

        for i in 0..m {
            for j in 0..n {
                max_len = cmp::max(max_len, Self::dfs(&matrix, &mut cache, i, j, m, n));
            }
        }

        max_len
    }

    fn dfs(
        matrix: &Vec<Vec<i32>>,
        cache: &mut Vec<Vec<i32>>,
        i: usize,
        j: usize,
        m: usize,
        n: usize,
    ) -> i32 {
        if cache[i][j] != 0 {
            return cache[i][j];
        }

        let mut current_max = 1;
        let current_val = matrix[i][j];

        for &(di, dj) in &Self::DIRS {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if ni >= 0 && ni < m as isize && nj >= 0 && nj < n as isize {
                let ni = ni as usize;
                let nj = nj as usize;

                if matrix[ni][nj] > current_val {
                    current_max = cmp::max(current_max, 1 + Self::dfs(matrix, cache, ni, nj, m, n));
                }
            }
        }

        cache[i][j] = current_max;
        current_max
    }
}
```

The implementation splits cleanly into two functions. `longest_increasing_path` handles the outer loop, iterating over every cell and tracking the global maximum. It initializes the cache as a zero-filled matrix and passes it mutably to the DFS. The `dfs` function is the workhorse: it checks the cache first, then explores all four directions using the `DIRS` constant. The boundary check casts indices to `isize` for safe arithmetic with negative offsets, then casts back to `usize` only after confirming the neighbor is in bounds. The `current_max` variable starts at 1 (the cell itself) and grows as longer paths are found through valid neighbors. Once computed, the result is stored in the cache before returning.

## Conclusion

Longest Increasing Path in a Matrix is a textbook example of how a "strictly increasing" constraint transforms what could be an NP-hard longest path problem into an elegant DFS with memoization. The strictly increasing requirement guarantees a DAG, which guarantees that memoization is safe and complete. Every cell is computed exactly once, giving linear time in the size of the matrix. The Rust solution mirrors this simplicity: a direction array, a cache matrix, and a recursive function that reads like a direct translation of the recurrence relation.
