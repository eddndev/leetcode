---
title: "1200 Minimum Absolute Difference - EN"
problemUrl: "https://leetcode.com/problems/minimum-absolute-difference/"
difficulty: "Easy"
pubDate: "2026-03-08"
tags: ["array", "sorting"]
complexity:
  time: "O(n log n)"
  space: "O(n)"
---

# Minimum Absolute Difference

## Problem
Given an array of distinct integers `arr`, find all pairs of elements with the minimum absolute difference between any two elements in the array. Return a list of pairs sorted in ascending order, where each pair `[a, b]` satisfies `a < b`.

## Solution
We sort the array. The minimum absolute difference can only occur between consecutive elements in the sorted array. We make a first pass with windows of size 2 to find the minimum difference, then a second pass to collect all consecutive pairs whose difference equals that minimum.

### Implementation in Rust

```rust
impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();

        let min_diff = arr.windows(2).map(|w| w[1] - w[0]).min().unwrap();

        arr.windows(2)
            .filter(|w| (w[1] - w[0]) == min_diff)
            .map(|w| vec![w[0], w[1]])
            .collect()
    }
}
```
