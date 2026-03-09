---
title: "0115 Distinct Subsequences - EN"
problemUrl: "https://leetcode.com/problems/distinct-subsequences/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "dynamic-programming"]
complexity:
  time: "O(M * N)"
  space: "O(M)"
---

# Counting Ghosts Inside a String

## The Problem
Given two strings `s` and `t`, return the number of distinct subsequences of `s` which equals `t`. A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements. The answer is guaranteed to fit in a 32-bit signed integer.

## The First Impression

At first glance, this problem feels deceptively close to a simple string matching exercise. But it's not asking *whether* `t` exists inside `s` -- it's asking *how many different ways* you can carve `t` out of `s` by selectively deleting characters. Each distinct set of indices you choose to keep forms a separate subsequence, and two subsequences count as different even if they produce the same characters, as long as the index positions differ.

My first instinct was brute force: enumerate every possible subset of indices in `s` and check if the resulting string equals `t`. But with strings up to length 1000, that's a combinatorial explosion. The key realization is that this problem has **optimal substructure**: the number of ways to form `t[0..j]` from `s[0..i]` depends on smaller subproblems, and subproblems overlap heavily. That's the classic signal for dynamic programming.

## Flattening the Table into a Single Row

The classic 2D approach would define `dp[i][j]` as the number of ways to form the first `j` characters of `t` from the first `i` characters of `s`. The recurrence is straightforward: for each character in `s`, if it matches `t[j]`, we add `dp[i-1][j-1]` (use this character) to `dp[i-1][j]` (skip it); otherwise we just carry forward `dp[i-1][j]`.

But notice that each row only depends on the previous row. That means we can collapse the entire table into a single one-dimensional array of length `m + 1`, where `m = len(t)`. The trick is to iterate **backwards** through the array when updating, so that we don't overwrite values we still need for the current row's computation.

### The base case

`dp[0] = 1`: there is exactly one way to form an empty subsequence from any prefix of `s` -- by selecting nothing. All other positions start at zero.

### The transition

For each character `s[i]`, we walk `j` from `m-1` down to `0`. If `s[i] == t[j]`, we do `dp[j+1] += dp[j]`. This captures the choice of "using" `s[i]` to match `t[j]`, accumulating the count from the subproblem where `t[0..j]` was already formed. By going in reverse, each `dp[j]` still holds its value from the previous iteration of `s`, which is exactly the "skip this character" scenario -- we don't need to do anything extra for it.

### Why `u64`?

Intermediate counts can grow beyond what a 32-bit integer can hold, even though the final answer fits in `i32`. Using `u64` for the DP array avoids overflow during accumulation. At the very end, we cast `dp[m]` to `i32`.

## Rust Solution

```rust
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let m = t.len();
        let n = s.len();

        if n < m {
            return 0;
        }

        let mut dp = vec![0u64; m + 1];

        dp[0] = 1;

        for &s_char in s_bytes {
            for j in (0..m).rev() {
                if s_char == t_bytes[j] {
                    dp[j + 1] += dp[j];
                }
            }
        }

        dp[m] as i32
    }
}
```

The Rust implementation is remarkably concise thanks to the space-optimized approach. We convert both strings to `&[u8]` with `as_bytes()` to work at the byte level, which is both efficient and sufficient since the problem deals with ASCII characters. The early return when `n < m` is a small but important guard: if `s` is shorter than `t`, there can't possibly be any subsequence of `s` that equals `t`. The reverse iteration `(0..m).rev()` is the linchpin that makes the 1D optimization correct -- it ensures we read each `dp[j]` before it gets modified in the current pass.

## Conclusion

This problem is a beautiful example of how a counting question over subsequences maps naturally to dynamic programming. The 2D table makes the recurrence intuitive, and the observation that each row depends only on the previous one lets us flatten the space to `O(M)`. The reverse-iteration trick is a pattern worth memorizing -- it shows up in many DP problems where you optimize from two dimensions down to one, such as the 0/1 knapsack. In the end, what looks like an overwhelming combinatorial challenge reduces to a clean linear scan with an elegant inner loop.
