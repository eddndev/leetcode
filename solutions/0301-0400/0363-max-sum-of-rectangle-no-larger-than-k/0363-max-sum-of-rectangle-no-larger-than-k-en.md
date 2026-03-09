---
title: "0363 Max Sum of Rectangle No Larger Than K - EN"
problemUrl: "https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "prefix-sum", "ordered-set", "binary-search", "matrix"]
complexity:
  time: "O(M^2 * N * log N) where M is the number of rows and N is the number of columns"
  space: "O(N)"
---

# The Bounded Treasure in the Grid

## The Problem
Given an `m x n` matrix `matrix` and an integer `k`, return the maximum sum of a rectangle in the matrix such that its sum is no larger than `k`. It is guaranteed that there will be a rectangle with a sum no larger than `k`.

## The Initial Intuition

Finding the maximum rectangle sum in a matrix is already a classic problem, but adding the constraint "no larger than k" changes everything. Without the constraint, Kadane's algorithm applied over compressed columns solves it cleanly. With the constraint, I cannot simply track the maximum subarray -- I need to find the best subarray sum that does not exceed a given bound.

My first thought is to reduce the 2D problem to multiple 1D problems. If I fix two row boundaries, the rectangle sum between those rows for any column range becomes a one-dimensional subarray sum problem on the compressed column sums. This is the standard row-compression technique: for each pair of row boundaries `(r1, r2)`, I maintain a running column sum array where each entry accumulates the matrix values from row `r1` to row `r2`.

## From 2D to 1D with Column Compression

For a fixed top row `r1`, I iterate the bottom row `r2` downward. As `r2` advances, I incrementally add `matrix[r2][c]` to `col_sums[c]` for each column `c`. Now `col_sums` represents the vertical sums from `r1` to `r2` for each column, and any contiguous subarray of `col_sums` corresponds to a rectangle in the original matrix.

The problem now reduces to: given a one-dimensional array `col_sums`, find the maximum subarray sum that is at most `k`. This is where the classic prefix sum combined with an ordered set comes in.

## Prefix Sums Meet Ordered Sets

I want the maximum value of `prefix[j] - prefix[i]` where `i < j` and this value is at most `k`. Rearranging, for each `prefix[j]`, I need the smallest `prefix[i]` such that `prefix[i] >= prefix[j] - k`. This is a classic "lower bound" query.

As I compute the running prefix sum, I maintain a `BTreeSet` of all previously seen prefix sums. For the current prefix sum `current_prefix_sum`, I query the set for the smallest value greater than or equal to `current_prefix_sum - k`. If such a value `prev_sum` exists, then `current_prefix_sum - prev_sum` is a valid subarray sum that does not exceed `k`, and I update the global maximum.

Inserting a seed value of `0` into the set before processing handles the case where the entire prefix from the start constitutes a valid rectangle.

## The Early Exit Optimization

If at any point the running maximum equals exactly `k`, I can return immediately. Since `k` is the upper bound and I am maximizing, no future rectangle can improve upon this result. This small optimization can save significant computation on certain inputs.

## Rust Solution

```rust
use std::cmp::max;
use std::collections::BTreeSet;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = matrix.len();
        if rows == 0 {
            return 0;
        }
        let cols = matrix[0].len();

        let mut max_sum = i32::MIN;

        for r1 in 0..rows {
            let mut col_sums = vec![0; cols];

            for r2 in r1..rows {
                for c in 0..cols {
                    col_sums[c] += matrix[r2][c];
                }

                let mut current_prefix_sum = 0;
                let mut set = BTreeSet::new();

                set.insert(0);

                for &val in &col_sums {
                    current_prefix_sum += val;

                    let target = current_prefix_sum - k;

                    if let Some(&prev_sum) = set.range(target..).next() {
                        max_sum = max(max_sum, current_prefix_sum - prev_sum);
                    }

                    set.insert(current_prefix_sum);
                }

                if max_sum == k {
                    return k;
                }
            }
        }

        max_sum
    }
}
```

The outer loop fixes the top row `r1` and the inner loop extends the bottom row `r2`. For each pair, `col_sums` accumulates the vertical sums incrementally. The innermost loop computes the running prefix sum over the compressed column array. Rust's `BTreeSet::range(target..)` returns an iterator starting from the first element greater than or equal to `target`, which is exactly the lower bound query I need. If such an element exists, the difference `current_prefix_sum - prev_sum` is a candidate answer. After processing all columns for a given row pair, if `max_sum` has already reached `k`, the function returns early.

## Conclusion

Max Sum of Rectangle No Larger Than K elegantly combines two powerful techniques: row compression to reduce a 2D problem to 1D, and prefix sums with an ordered set to efficiently find bounded subarray sums. The `BTreeSet` in Rust provides O(log N) insertions and range queries, keeping the per-row-pair work at O(N log N). The overall complexity of O(M^2 * N * log N) is the best achievable for this problem without resorting to more exotic data structures. The early exit when the answer reaches exactly `k` is a practical optimization that pays off when the bound is tight. What starts as a daunting 2D optimization problem with a constraint becomes, through systematic decomposition, a sequence of well-understood operations on sorted sets.
