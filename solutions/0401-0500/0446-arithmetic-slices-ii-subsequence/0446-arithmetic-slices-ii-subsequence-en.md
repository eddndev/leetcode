---
title: "0446 Arithmetic Slices II - Subsequence - EN"
problemUrl: "https://leetcode.com/problems/arithmetic-slices-ii-subsequence/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "hash-table", "subsequence"]
complexity:
  time: "O(N^2), where N is the length of the array"
  space: "O(N^2), for the hash maps stored at each index"
---

# Counting Ghosts in the Sequence

## The Problem
Given an integer array `nums`, return the number of all arithmetic subsequences of `nums`. A sequence is arithmetic if it consists of at least three elements and the difference between consecutive elements is the same. A subsequence is a sequence derived from the array by deleting some or no elements without changing the order of the remaining elements.

## The Trap of Brute Force

My first reaction was to enumerate every possible subsequence and check whether it forms an arithmetic progression. But an array of length N has 2^N subsequences, which is catastrophically slow. Even restricting to subsequences of length 3 or more, the sheer number of combinations makes brute force impossible for the given constraints (N up to 1000). I need a way to count without constructing.

## Thinking in Pairs, Not Triples

The key insight that unlocks this problem is to think about pairs rather than complete subsequences. Every arithmetic subsequence of length k can be decomposed into an arithmetic subsequence of length k-1 ending at the second-to-last element, plus the last element. This means that if I know, for each element `nums[j]`, how many arithmetic subsequences with a given common difference `d` end at `j`, I can extend each of them by appending a new element `nums[i]` where `nums[i] - nums[j] == d`.

This is the DP formulation: let `dp[i]` be a HashMap where `dp[i][d]` stores the number of "weak" arithmetic subsequences (length 2 or more) ending at index `i` with common difference `d`. I call them "weak" because a pair of two elements is not yet a valid arithmetic subsequence (we need at least 3), but it is a potential prefix of one.

## How the Count Accumulates

For each pair `(j, i)` where `j < i`, I compute `diff = nums[i] - nums[j]`. Then:

1. I look up `dp[j][diff]`, which tells me how many weak arithmetic subsequences end at `j` with this difference. Each of these has length at least 2, so extending them by `nums[i]` produces subsequences of length at least 3 -- valid ones. I add this count to the total.

2. I update `dp[i][diff]` by adding `dp[j][diff] + 1`. The `+1` accounts for the new pair `(nums[j], nums[i])` itself, which is a weak subsequence of length 2 that might be extended later.

The beauty is that I never explicitly track the length of any subsequence. The `+1` creates the seeds (pairs), and the propagation of `dp[j][diff]` counts all the valid extensions. Every arithmetic subsequence of length 3 or more is counted exactly once, at the moment its last element is added.

## Walking Through an Example

Consider `nums = [2, 4, 6, 8, 10]`.

- **i = 1 (nums[i] = 4)**: Pair with j=0: diff=2, dp[0][2]=0. Total += 0. dp[1][2] = 0 + 1 = 1.
- **i = 2 (nums[i] = 6)**: Pair with j=0: diff=4, dp[2][4] = 1. Pair with j=1: diff=2, dp[1][2]=1. Total += 1. dp[2][2] = 1 + 1 = 2.
- **i = 3 (nums[i] = 8)**: Pair with j=0: diff=6, dp[3][6] = 1. Pair with j=1: diff=4, dp[1][4]=0, dp[3][4] = 1. Pair with j=2: diff=2, dp[2][2]=2. Total += 2. dp[3][2] = 2 + 1 = 3.
- **i = 4 (nums[i] = 10)**: Pair with j=0: diff=8, dp[4][8]=1. Pair with j=1: diff=6, dp[4][6]=1. Pair with j=2: diff=4, dp[2][4]=1, total += 1, dp[4][4]=2. Pair with j=3: diff=2, dp[3][2]=3, total += 3, dp[4][2] = 3 + 1 = 4.

Final total = 0 + 1 + 2 + 1 + 3 = 7. The seven arithmetic subsequences are: [2,4,6], [4,6,8], [6,8,10], [2,4,6,8], [4,6,8,10], [2,6,10], [2,4,6,8,10].

## Why i64 for the Difference

One subtle detail: the problem allows values up to 2^31 - 1 and as low as -2^31. The difference between two such values can overflow a 32-bit integer. By casting to `i64` before subtracting, I avoid this trap entirely.

## Rust Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut total = 0;
        let mut dp: Vec<HashMap<i64, i32>> = vec![HashMap::new(); n];

        for i in 0..n {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;
                let count = *dp[j].get(&diff).unwrap_or(&0);

                total += count;

                *dp[i].entry(diff).or_insert(0) += count + 1;
            }
        }

        total
    }
}
```

The solution allocates a vector of HashMaps, one per index. For each pair `(j, i)`, it computes the difference as an `i64`, retrieves the count of existing weak subsequences at `j` with that difference, adds it to the running total, and then updates `dp[i]` to include both the extended subsequences and the new pair. The `entry` API makes the HashMap update clean: if the key does not exist, it inserts 0 before adding. The entire computation runs in O(N^2) time and space, which comfortably handles arrays up to the constraint limit of 1000 elements.

## Conclusion

Arithmetic Slices II - Subsequence is a problem that rewards a shift in perspective. Instead of trying to enumerate subsequences directly, I count them by building up from pairs. Each pair is a seed that, when extended, becomes a valid arithmetic subsequence. The HashMap-per-index DP tracks every possible common difference simultaneously, and the accumulation logic ensures that each valid subsequence of length 3 or more is counted exactly once at the moment it is completed. It is a textbook example of how the right DP formulation can turn an exponential problem into a quadratic one.
