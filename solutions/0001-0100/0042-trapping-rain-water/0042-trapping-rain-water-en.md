---
title: "0042 Trapping Rain Water - EN"
problemUrl: "https://leetcode.com/problems/trapping-rain-water/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["array", "two-pointers"]
complexity:
  time: "O(N)"
  space: "O(1)"
---

# Trapping the Rain Between Walls

## The Problem
Given an array of non-negative integers `height` where each element represents the height of a bar with width 1, compute how much water can be trapped between the bars after raining.

## The Initial Intuition

When I first saw this problem, my mind went straight to brute force: for each position, look at the tallest bar to its left and the tallest bar to its right, and the water that fits there is the minimum of those two heights minus the height of the current bar. It's correct, but it requires scanning the entire array for each position, which gives O(N^2).

The natural next step is to precompute those maximums with two auxiliary arrays: one storing the maximum from the left and another from the right. That brings the time down to O(N), but uses O(N) extra space. I asked myself: can we do it without those auxiliary arrays? And the answer is yes, with **two pointers**.

## The Two-Pointer Strategy

The key observation is this: we don't need to know both maximums at the same time. We only need to know that the opposite side has a bar tall enough to "hold" the water.

We place a pointer `left` at the beginning and another `right` at the end of the array. We also maintain two variables: `left_max` and `right_max`, which track the tallest bar seen from each end.

At each step, we compare `height[left]` with `height[right]`:

1. **If `height[left] < height[right]`:** We know the right side has at least one bar as tall as `height[right]`, which is greater than `height[left]`. Therefore, the water at position `left` is determined solely by `left_max`. If `height[left] >= left_max`, we update `left_max`. Otherwise, the difference `left_max - height[left]` is trapped water. We advance `left`.

2. **If `height[left] >= height[right]`:** The reasoning is symmetric. The left side guarantees there's a bar tall enough, so the water at `right` depends only on `right_max`. If `height[right] >= right_max`, we update `right_max`. Otherwise, we add `right_max - height[right]`. We move `right` backward.

The elegance is that we never need to look back or precompute anything. The fact that one of the two sides always has a bar taller than the current position gives us the guarantee we need to accumulate water with confidence.

### A Step-by-step Example

For `height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]`:
- Start with `left = 0`, `right = 11`, `left_max = 0`, `right_max = 0`, `water = 0`
- `height[0]=0 < height[11]=1`: `left_max = 0`, no water. `left = 1`
- `height[1]=1 < height[11]=1`? No, they're equal, so we go to else: `right_max = 1`. `right = 10`
- `height[1]=1 < height[10]=2`: `left_max = 1`. `left = 2`
- `height[2]=0 < height[10]=2`: `0 < left_max(1)`, water += 1. `left = 3`
- `height[3]=2 >= height[10]=2`: `right_max = max(1, 2) = 2`. `right = 9`
- `height[3]=2 >= height[9]=1`: `1 < right_max(2)`, water += 1. `right = 8`
- And so it continues until `left` and `right` meet, accumulating a total of **6** units of water.

## Rust Solution

```rust
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let mut left = 0;
        let mut right = height.len() - 1;

        let mut left_max = 0;
        let mut right_max = 0;

        let mut water = 0;

        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    water += left_max - height[left];
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    water += right_max - height[right];
                }
                right -= 1;
            }
        }

        water
    }
}
```

The Rust implementation is straightforward and clean. The early guard `height.len() < 3` is a practical detail: with fewer than three bars it's impossible to trap water, and it also avoids a potential underflow on `height.len() - 1` when the vector is empty (since `len()` returns `usize`, an unsigned type). The two pointers are managed with simple indices, and all the logic fits in a single `while` loop with a clear branch. There are no allocations, no auxiliary structures: just four scalar variables and the input array.

## Conclusion

This problem is a classic that demonstrates the power of the two-pointer technique. The key insight is realizing that we don't need global information to make local decisions: it's enough to know that the opposite side has a wall tall enough. That observation is what allows us to go from O(N) space to O(1), eliminating the precomputed maximum arrays without losing any information. Sometimes the most efficient solution doesn't come from adding more structure, but from realizing we already have everything we need.
