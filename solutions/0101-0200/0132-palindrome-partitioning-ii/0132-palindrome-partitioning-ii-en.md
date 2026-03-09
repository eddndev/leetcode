---
title: "0132 Palindrome Partitioning II - EN"
problemUrl: "https://leetcode.com/problems/palindrome-partitioning-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "dynamic-programming", "palindrome"]
complexity:
  time: "O(N^2)"
  space: "O(N)"
---

# Breaking Mirrors with the Fewest Strikes

## The Problem
Given a string `s`, return the minimum number of cuts needed so that every substring of the partition is a palindrome.

## The First Impression

This problem is the natural sequel to "Palindrome Partitioning" (where you enumerate all valid partitions), but here the goal shifts dramatically: we no longer want every possible way to slice the string into palindromes, we want the one that uses **the fewest cuts**. That difference transforms the problem from exhaustive backtracking into pure optimization.

My first thought was a classic 2D table: `is_palindrome[i][j]` to know whether `s[i..=j]` is a palindrome, combined with another table `dp[i]` storing the minimum number of cuts for the prefix `s[0..=i]`. But that requires `O(N^2)` space just for the palindrome table. The question becomes: can we do something more elegant?

## Expanding from the Center

The key insight is to flip the perspective. Instead of asking "for each ending position, what's the best cut?", we expand palindromes outward from every possible center. Each time we find a palindrome `s[l..=r]`, we know we can reach position `r` with a cut right before `l` (i.e., `dp[l-1] + 1`), or with zero cuts if `l == 0` (because the entire substring from the start is a palindrome). If this value is smaller than the current `dp[r]`, we update it.

### The DP Array

We initialize `dp[i] = i` for every position. This represents the worst case: cutting between every pair of adjacent characters, which always produces single-character palindromes. From there, every palindrome we discover can only improve these values.

### Odd-length and Even-length Palindromes

For each center, we perform two expansions:
- **Odd**: start with `l = r = center`, expanding symmetrically while `s[l] == s[r]`.
- **Even**: start with `l = center, r = center + 1`, catching even-length palindromes like `"aa"` or `"abba"`.

In both cases, inside the expansion loop, we compute the potential cut and update `dp[r]` if we find an improvement. If `l` reaches `0`, the palindrome covers the string from the very beginning, so the cost is `0` (no cuts needed for that segment).

### Why Does This Work?

Every palindrome in the string will be discovered by some center expansion. By processing all centers left to right, when we evaluate `dp[l-1]`, that value has already been optimized by all previous centers. Thus, the final value `dp[n-1]` holds the minimum number of cuts for the entire string.

## Rust Solution

```rust
use std::cmp;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }

        let s = s.as_bytes();

        let mut dp: Vec<i32> = (0..n as i32).collect();

        for center in 0..n {
            let (mut l, mut r) = (center, center);
            while r < n && s[l] == s[r] {
                let new_cut = if l == 0 { 0 } else { dp[l - 1] + 1 };
                if new_cut < dp[r] {
                    dp[r] = new_cut;
                }

                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }

            let (mut l, mut r) = (center, center + 1);
            while r < n && s[l] == s[r] {
                let new_cut = if l == 0 { 0 } else { dp[l - 1] + 1 };
                if new_cut < dp[r] {
                    dp[r] = new_cut;
                }

                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }
        }

        dp[n - 1]
    }
}
```

The Rust implementation is straightforward and efficient. We convert the string to `&[u8]` with `as_bytes()` for fast byte-level comparisons. The `dp` vector is initialized with `(0..n as i32).collect()`, which generates the sequence `[0, 1, 2, ..., n-1]` -- the worst-case scenario. The two `while` blocks are structurally identical except for the initialization of `(l, r)`: the first handles odd-length palindromes and the second even-length ones. The guard `if l == 0 { break; }` is necessary because `l` is `usize` (unsigned in Rust), and subtracting 1 from zero would panic due to underflow. Note that the `use std::cmp` at the top of the original file is unused in this version of the solution -- likely a leftover from an earlier iteration that used `cmp::min`.

## Conclusion

This problem demonstrates how the center-expansion technique, typically associated with finding palindromes, can be elegantly fused with dynamic programming to solve an optimization problem. Instead of building a full boolean palindrome table and then optimizing cuts separately, both operations happen simultaneously during the expansion. The result is an `O(N^2)` time algorithm with only `O(N)` space -- a significant leap over the naive approach that would need a full 2D table. The solution is a reminder that sometimes the best way to solve a problem is not to attack it head-on, but to look at it from a different angle.
