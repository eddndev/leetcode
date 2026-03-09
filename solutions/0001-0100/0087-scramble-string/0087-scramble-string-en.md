---
title: "0087 Scramble String - EN"
problemUrl: "https://leetcode.com/problems/scramble-string/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "dynamic-programming", "recursion", "memoization"]
complexity:
  time: "O(N^4)"
  space: "O(N^3)"
---

# Unscrambling the Recursion Tree

## The Problem
Given two strings `s1` and `s2` of the same length, determine if `s2` is a scrambled version of `s1`. A scrambled string is formed by recursively splitting the string into two non-empty parts at any position, and optionally swapping them, then continuing the process on the substrings.

## The First Impression

When I first encountered this problem, it felt deceptively simple: just check if one string can be rearranged into another. But "scramble" doesn't mean "anagram." The transformation is recursive and structural: you pick a split point in the string, divide it into two parts, optionally swap those parts, and then repeat the process on each part independently. This tree-like nature is what makes it tricky.

My immediate thought was recursion. For two strings of length `n`, I can try every possible split point `k` from `1` to `n-1`. At each split, there are two possibilities:

1. **No swap:** The left part of `s1` (length `k`) matches the left part of `s2`, and the right part of `s1` (length `n-k`) matches the right part of `s2`.
2. **Swap:** The left part of `s1` (length `k`) matches the **right** part of `s2`, and the right part of `s1` (length `n-k`) matches the **left** part of `s2`.

This gives us the recursive structure, but without memoization, the overlapping subproblems would cause an exponential blowup. That's where the memo table becomes essential.

## The Pruning That Makes It Work

Before diving into the recursion for a given pair of substrings, there's a critical optimization: check whether they are even anagrams of each other. If two substrings don't contain the same character frequencies, there's no point exploring further. This is done with a simple frequency count array of size 26. This pruning eliminates a massive number of branches early and is what makes the solution practical despite the theoretical O(N^4) complexity.

There's also the trivial base case: if the two substrings are already identical, we return `true` immediately without any further splitting.

## The Memo Table

The state of each subproblem is defined by three values: the starting index in `s1`, the starting index in `s2`, and the length of the substring being compared. This gives us a three-dimensional memo table of size `(n+1) x n x n`, where each cell stores an `Option<bool>`: `None` if not yet computed, `Some(true)` or `Some(false)` if already resolved.

By caching every subproblem result, we guarantee that no pair of substrings is ever evaluated more than once, turning the exponential recursion into polynomial time.

## Rust Solution

```rust
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        if n != s2.len() {
            return false;
        }
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        let mut memo = vec![vec![vec![None; n]; n]; n + 1];

        Self::solve(s1_bytes, s2_bytes, 0, 0, n, &mut memo)
    }

    fn solve(
        s1: &[u8],
        s2: &[u8],
        i1: usize,
        i2: usize,
        len: usize,
        memo: &mut Vec<Vec<Vec<Option<bool>>>>,
    ) -> bool {
        if let Some(res) = memo[len][i1][i2] {
            return res;
        }
        if s1[i1..i1 + len] == s2[i2..i2 + len] {
            memo[len][i1][i2] = Some(true);
            return true;
        }

        let mut counts = [0; 26];
        for k in 0..len {
            counts[(s1[i1 + k] - b'a') as usize] += 1;
            counts[(s2[i2 + k] - b'a') as usize] -= 1;
        }
        if counts.iter().any(|&c| c != 0) {
            memo[len][i1][i2] = Some(false);
            return false;
        }

        for k in 1..len {
            if Self::solve(s1, s2, i1, i2, k, memo)
                && Self::solve(s1, s2, i1 + k, i2 + k, len - k, memo)
            {
                memo[len][i1][i2] = Some(true);
                return true;
            }

            if Self::solve(s1, s2, i1, i2 + len - k, k, memo)
                && Self::solve(s1, s2, i1 + k, i2, len - k, memo)
            {
                memo[len][i1][i2] = Some(true);
                return true;
            }
        }

        memo[len][i1][i2] = Some(false);
        false
    }
}
```

The Rust implementation leans into the language's strengths. We convert both strings to `&[u8]` with `as_bytes()` so that substring comparisons and character arithmetic are simple byte operations. The memo table uses `Option<bool>` to cleanly distinguish between "not yet computed" and an actual result, which is more expressive than using a sentinel value. The `solve` function is a classic top-down DP: it checks the memo first, tries the trivial case, prunes via the anagram check, and then explores all split points with both swap and no-swap configurations. The moment any valid split is found, we short-circuit and return `true`.

## Conclusion

This problem is a beautiful example of how recursion with memoization can tame a combinatorial explosion. The recursive structure mirrors the problem definition perfectly: try every split, try both orderings, and cache everything. The anagram pruning is the practical ingredient that keeps the search space manageable. Without it, the solution would still be correct but painfully slow. With it, we get a clean O(N^4) solution that handles the constraints comfortably.
