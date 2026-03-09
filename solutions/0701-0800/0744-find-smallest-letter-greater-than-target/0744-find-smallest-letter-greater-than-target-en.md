---
title: "0744 Find Smallest Letter Greater Than Target - EN"
problemUrl: "https://leetcode.com/problems/find-smallest-letter-greater-than-target/"
difficulty: "Easy"
pubDate: "2026-03-08"
tags: ["array", "binary-search"]
complexity:
  time: "O(n)"
  space: "O(1)"
---

# Find Smallest Letter Greater Than Target

## Problem
Given an array of characters `letters` sorted in ascending order and a character `target`, return the smallest character in `letters` that is strictly greater than `target`. If no such character exists, return the first element of the array (the array wraps around).

## Solution
We iterate through the array from left to right. The first character that is strictly greater than `target` is the answer, since the array is sorted. If no character satisfies the condition, we return the first element of the array.

### Implementation in Rust

```rust
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for &c in &letters {
            if c > target {
                return c;
            }
        }

        letters[0]
    }
}
```
