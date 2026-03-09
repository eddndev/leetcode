---
title: "0407 Trapping Rain Water II - EN"
problemUrl: "https://leetcode.com/problems/trapping-rain-water-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["heap", "breadth-first-search", "matrix"]
complexity:
  time: "O(M * N * log(M * N)) where M and N are the dimensions of the matrix"
  space: "O(M * N)"
---

# Flooding the Terrain from the Edges

## The Problem
Given an `m x n` integer matrix `heightMap` representing the height of each cell in an elevation map, compute the volume of water it can trap after raining.

## The Initial Intuition

This problem is the three-dimensional extension of the classic Trapping Rain Water. In the 1D version, the idea was straightforward: for each bar, the water it holds depends on the tallest bar to its left and right. But in two dimensions, the water at a cell does not depend on just two directions -- it depends on the entire contour surrounding it. A cell can "leak" water through any path leading to the edge of the matrix.

My first reaction was to try generalizing two pointers, but I quickly realized that does not scale to 2D. The right question is not "what is the max to the left and right" but rather "what is the lowest path from this cell to the border." And that led me to think about the problem from the outside in.

## Thinking Like Water

Water always escapes through the lowest point on the boundary. If I imagine the terrain as an irregular pool, the water level is determined by the shortest wall. So instead of asking "how much water fits here?" for each cell, I should ask "how high can the water rise before it spills over?"

This suggests a greedy approach: start from the borders, which are cells that cannot hold water, and expand inward by always processing the lowest-height cell first. If an interior cell is lower than the current boundary level containing it, the difference is trapped water.

## The Min-Heap as the Frontier

The ideal data structure for this approach is a min-heap (minimum priority queue). I start by inserting all border cells into the heap, marking them as visited. These cells form the initial "wall" of the pool.

At each iteration, I extract the cell with the smallest height from the heap. This cell represents the weakest point of the current contour. Then I examine its four unvisited neighbors. For each neighbor:

1. If its height is **less than** the current cell's height, the difference is water trapped there. I insert the neighbor into the heap with the current cell's height (not its own), because the water raises it to that level and it now becomes part of the contour at that effective height.

2. If its height is **greater than or equal**, no water is trapped, and the neighbor enters the heap with its own height.

In both cases, the height I insert into the heap is `max(current_height, neighbor_height)`, which elegantly captures the logic: the effective contour level never decreases, it only rises.

## Why the Min-Heap Guarantees Correctness

The key insight is that by always processing the lowest-height cell first, I simulate how water would fill the terrain in reality: it overflows at the low points first. When I process a cell from the heap, I have the guarantee that its height represents the lowest possible water level that can reach that region. Any interior cell discovered later will have a contour at least as high, because the heap already consumed all the lower points.

This is essentially a priority-based BFS, similar to Dijkstra's algorithm but over terrain heights.

## The Base Case

If the matrix has fewer than 3 rows or fewer than 3 columns, it is impossible to trap water: all cells are part of the border or adjacent to it with no room for a "pool." The guard `m < 3 || n < 3` at the top handles this trivial case.

## Rust Solution

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();

        if m < 3 || n < 3 {
            return 0;
        }

        let mut heap = BinaryHeap::new();
        let mut visited = vec![vec![false; n]; m];

        for r in 0..m {
            for c in 0..n {
                if r == 0 || r == m - 1 || c == 0 || c == n - 1 {
                    heap.push(Reverse((height_map[r][c], r, c)));
                    visited[r][c] = true;
                }
            }
        }

        let mut total_water = 0;

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some(Reverse((h, r, c))) = heap.pop() {
            for (dr, dc) in directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < m as isize && nc >= 0 && nc < n as isize {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if !visited[nr][nc] {
                        visited[nr][nc] = true;

                        let neighbor_height = height_map[nr][nc];

                        if neighbor_height < h {
                            total_water += h - neighbor_height;
                        }

                        heap.push(Reverse((h.max(neighbor_height), nr, nc)));
                    }
                }
            }
        }

        total_water
    }
}
```

Rust's `BinaryHeap` is a max-heap by default, so I wrap tuples in `Reverse` to turn it into a min-heap. The tuple `(height, row, column)` is compared lexicographically, ensuring I always process the lowest cell first. The `visited` matrix prevents processing a cell more than once, which is essential for both correctness and efficiency. The expression `h.max(neighbor_height)` when pushing into the heap is the heart of the algorithm: it propagates the effective water level inward, guaranteeing that each new cell enters the contour at the correct height. The directions are defined as `isize` to cleanly handle signed arithmetic when checking bounds, and the casts to `usize` only happen after confirming the indices are valid.

## Conclusion

Trapping Rain Water II transforms an intuitively complex problem -- computing water volume over a 3D terrain -- into an elegant exploration from the borders inward. The min-heap acts as an ordered frontier that simulates the physical behavior of water: it always overflows at the lowest point first. The generalization from 1D to 2D was not trivial, but the core idea remains the same: the water trapped at any point is determined by the lowest obstacle between that point and freedom. In 1D, that obstacle is found with two pointers; in 2D, with a heap that erodes the contour layer by layer.
