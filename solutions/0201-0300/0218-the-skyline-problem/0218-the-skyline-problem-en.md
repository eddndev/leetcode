---
title: "0218 The Skyline Problem - EN"
problemUrl: "https://leetcode.com/problems/the-skyline-problem/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["heap", "sweep-line", "sorting", "divide-and-conquer"]
complexity:
  time: "O(N log N) where N is the number of buildings"
  space: "O(N) for the heap and the critical points"
---

# Drawing Silhouettes Against the Horizon

## The Problem
Given a list of buildings represented as `[left, right, height]`, return the skyline formed by these buildings as a list of key points `[x, height]`. Each key point marks where the skyline changes height. The output should contain no consecutive entries with the same height.

## The Challenge of Overlapping Towers

At first glance, this problem seems like a simple matter of tracking which building is tallest at each position. But buildings overlap in complex ways -- a shorter building might be entirely hidden behind a taller one, or two buildings of different heights might share the same left edge. The difficulty lies in efficiently determining *when the maximum height changes* as we sweep across the x-axis.

A brute force approach would check the height at every integer x-coordinate, computing the maximum height among all active buildings at each point. But with coordinates potentially in the tens of thousands, this is wasteful. Most of those x-coordinates don't produce any change in the skyline. The skyline can only change at the edges of buildings -- at their left and right boundaries.

## The Strategy: Sweep Line with a Max-Heap

### Identifying Critical Points

My key insight was that the skyline can only change at x-coordinates where a building starts or ends. So I extract all unique x-coordinates from the building edges, sort them, and process them left to right. These are the only positions where a height transition could occur.

### The Max-Heap as a Height Tracker

As I sweep from left to right, I need to know the tallest active building at each critical point. A **max-heap** (priority queue) is the natural choice: it gives me instant access to the current maximum height.

For each critical x-coordinate, I do two things:

1. **Activate buildings**: any building whose left edge equals the current x gets pushed onto the heap as `(height, right_edge)`. Since Rust's `BinaryHeap` is a max-heap by default, storing height first means the tallest building will always be at the top.

2. **Expire buildings**: before reading the current height, I pop any building from the top of the heap whose right edge is at or before the current x-coordinate. These buildings have ended and should no longer contribute to the skyline.

After these two steps, the top of the heap (if non-empty) gives me the current skyline height. If it differs from the last recorded height, I have found a new key point.

### A Concrete Example

With `buildings = [[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]]`:

```
Critical points: [2, 3, 5, 7, 9, 12, 15, 19, 20, 24]

x=2: Activate (10,9). Heap: [(10,9)]. Height=10 -> record [2,10]
x=3: Activate (15,7). Heap: [(15,7),(10,9)]. Height=15 -> record [3,15]
x=5: Activate (12,12). Heap: [(15,7),(12,12),(10,9)]. Height=15 -> no change
x=7: Expire (15,7). Heap: [(12,12),(10,9)]. Height=12 -> record [7,12]
x=9: Expire (10,9). Heap: [(12,12)]. Height=12 -> no change
x=12: Expire (12,12). Heap: []. Height=0 -> record [12,0]
x=15: Activate (10,20). Heap: [(10,20)]. Height=10 -> record [15,10]
x=19: Activate (8,24). Heap: [(10,20),(8,24)]. Height=10 -> no change
x=20: Expire (10,20). Heap: [(8,24)]. Height=8 -> record [20,8]
x=24: Expire (8,24). Heap: []. Height=0 -> record [24,0]

Result: [[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]
```

### Why Lazy Deletion Works

A subtle but important detail: when I "expire" buildings, I only pop from the top of the heap. Buildings that have ended but are buried under taller ones remain in the heap. This is fine -- they'll never affect the result because they're hidden behind the taller building above them. When the taller building eventually expires, those stale entries will be at the top and get popped then. This lazy deletion avoids the need for a more complex data structure like a balanced BST with deletion support.

## Rust Solution

```rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn get_skyline(mut buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points = Vec::with_capacity(buildings.len() * 2);
        for b in &buildings {
            points.push(b[0]);
            points.push(b[1]);
        }
        points.sort_unstable();
        points.dedup();

        buildings.sort_unstable_by_key(|b| b[0]);

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut i = 0;
        let n = buildings.len();

        for &x in &points {
            while i < n && buildings[i][0] == x {
                heap.push((buildings[i][2], buildings[i][1]));
                i += 1;
            }

            while let Some(&(_, right)) = heap.peek() {
                if right <= x {
                    heap.pop();
                } else {
                    break;
                }
            }

            let curr_height = if let Some(&(h, _)) = heap.peek() {
                h
            } else {
                0
            };

            if result.is_empty() || result.last().unwrap()[1] != curr_height {
                result.push(vec![x, curr_height]);
            }
        }

        result
    }
}
```

The implementation starts by collecting all building edges into a deduplicated, sorted list of critical x-coordinates. Buildings are sorted by their left edge so they can be activated in order with a simple index `i` that only advances forward -- no binary search needed. The `BinaryHeap` stores `(height, right_edge)` tuples; since Rust's heap is max-ordered by default, the tallest active building is always accessible via `peek()`. The lazy deletion loop at each critical point peels off expired entries from the top only, leaving buried expired entries for later cleanup. The final comparison `result.last().unwrap()[1] != curr_height` ensures that consecutive key points with the same height are never emitted, satisfying the problem's output constraint. The `with_capacity` pre-allocation on the `points` vector avoids reallocations, and `sort_unstable` with `dedup` efficiently produces the sorted unique set of critical coordinates.

## Conclusion

The Skyline Problem is a classic application of the sweep line paradigm. By recognizing that the skyline only changes at building boundaries, we reduce an infinite-coordinate problem to a finite set of critical events. The max-heap provides O(log N) insertion and O(1) access to the current maximum height, while lazy deletion keeps the logic simple without sacrificing correctness. The result is a clean O(N log N) solution that processes each building edge exactly once and lets expired buildings fall away naturally.
