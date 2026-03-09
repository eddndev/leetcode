---
title: "1292 Maximum Side Length of a Square - EN"
problemUrl: "https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["array", "binary-search", "matrix", "prefix-sum"]
complexity:
  time: "O(m * n)"
  space: "O(m * n)"
---

# Maximum Side Length of a Square: Prefix Sums Meet Greedy Growth

## The Problem
Given an `m x n` matrix `mat` and an integer `threshold`, we need to find the maximum side length of a square submatrix whose sum of elements is less than or equal to `threshold`. If no such square exists, we return 0.

For example, given a 3x3 matrix and a threshold of 4, we need to check every possible square submatrix and find the largest one whose total sum does not exceed 4.

At first glance, this seems like it could involve a brute-force check of every possible square at every possible position, but that would be far too slow for large matrices.

## The Intuition: Fast Submatrix Sums
When I first looked at this problem, I immediately thought of **2D prefix sums**. This is the classic technique for computing the sum of any rectangular subregion of a matrix in O(1) time, once you have done O(m * n) preprocessing.

The prefix sum `dp[i][j]` stores the sum of all elements in the rectangle from `(0, 0)` to `(i-1, j-1)`. Using inclusion-exclusion, the sum of any rectangle can be computed as:

```
sum(r1, c1, r2, c2) = dp[r2][c2] - dp[r1][c2] - dp[r2][c1] + dp[r1][c1]
```

So checking whether a given square fits within the threshold becomes a constant-time operation. The question then becomes: how do we find the maximum side length efficiently?

## The Key: Incremental Growth Instead of Binary Search
The typical approach would be to build the prefix sum table and then binary search on the side length. But there is a more elegant observation: as we scan the matrix from top-left to bottom-right, we only need to check whether the current best answer can be incremented by 1.

Think about it this way: if we have already found a valid square of side `k`, the next interesting question is whether a square of side `k + 1` exists. We do not need to re-check smaller sizes. As we visit each cell `(i, j)`, we compute the prefix sum and then check if a square of side `max_len + 1` ending at `(i, j)` has a sum within the threshold. If it does, we increment `max_len`.

This works because if a valid square of side `k + 1` exists anywhere in the matrix, we will encounter its bottom-right corner during our scan, and at that point `max_len` will be at least `k` (since the square of side `k` was already found), so we will test and accept `k + 1`.

## The Algorithm
1. Create a prefix sum table `dp` of size `(m+1) x (n+1)`, initialized to zero.
2. Initialize `max_len = 0`.
3. For each cell `(i, j)` from `(1, 1)` to `(m, n)`:
   - Compute `dp[i][j] = mat[i-1][j-1] + dp[i-1][j] + dp[i][j-1] - dp[i-1][j-1]`.
   - Let `current_len = max_len + 1`.
   - If `i >= current_len` and `j >= current_len` (the square fits), compute the sum of the square of side `current_len` ending at `(i, j)`.
   - If that sum is within the threshold, increment `max_len`.
4. Return `max_len`.

The beauty of this approach is that we never check more than one candidate side length per cell, so the entire algorithm runs in O(m * n) time.

### Implementation in Rust

```rust
impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut dp = vec![vec![0; n + 1]; m + 1];

        let mut max_len = 0;

        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = mat[i - 1][j - 1] + dp[i - 1][j] + dp[i][j - 1] - dp[i - 1][j - 1];

                let current_len = max_len + 1;

                if i >= current_len && j >= current_len {
                    let sum = dp[i][j] - dp[i - current_len][j] - dp[i][j - current_len]
                        + dp[i - current_len][j - current_len];

                    if sum <= threshold {
                        max_len += 1;
                    }
                }
            }
        }

        max_len as i32
    }
}
```

## Conclusion
This problem is a great example of how prefix sums can transform a seemingly expensive search into a linear scan. The clever insight is that we do not need to binary search over the side length at all. By only ever asking "can I do one better?", we reduce the problem to a single pass through the matrix. It is one of those solutions that feels almost too simple once you see it, but getting there requires recognizing that the answer can only grow by one at a time.
