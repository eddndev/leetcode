---
title: "1653 Minimum Deletions to Make String Balanced - EN"
problemUrl: "https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["string", "dynamic-programming", "stack"]
complexity:
  time: "O(n)"
  space: "O(1)"
---

# Minimum Deletions to Make String Balanced: Tracking the Cost as You Go

## The Problem
Given a string `s` consisting only of characters `'a'` and `'b'`, you can delete any number of characters from `s` to make the remaining string balanced. A string is balanced if there is no pair of indices `(i, j)` such that `i < j`, `s[i] = 'b'`, and `s[j] = 'a'`. In other words, all `'a'`s must come before all `'b'`s. Return the minimum number of deletions needed.

The brute force approach would try every possible partition point and count how many `'b'`s appear before it and how many `'a'`s appear after it. That works but requires precomputing prefix sums. There is a cleaner way that solves it in a single pass.

## The Intuition: A Running Decision

The key insight is to think about what happens as we scan the string from left to right. We maintain two pieces of state: the count of `'b'`s we have seen so far, and the minimum deletions needed to keep everything balanced up to the current position.

When we encounter a `'b'`, it does not cause any problem by itself. A `'b'` appearing after previous characters is fine for balance. We just increment our `'b'` counter.

When we encounter an `'a'`, we face a decision. This `'a'` appears after all the `'b'`s we have seen so far, which violates the balance condition. We have two choices: delete this `'a'` (which costs us one more deletion on top of our current result), or delete all the `'b'`s we have seen so far (which costs exactly `b_count`). We pick whichever is cheaper.

This is the DP transition in disguise: `res = min(res + 1, b_count)`. The beauty is that `res + 1` represents "keep the previous optimal solution and just remove this new `'a'`", while `b_count` represents "start fresh by removing all `'b'`s seen so far, making this `'a'` valid." The minimum of the two always gives us the global optimum at each step.

## Rust Solution

Iterating over `s.bytes()` instead of `s.chars()` avoids UTF-8 decoding overhead since we only care about ASCII characters. The `std::cmp::min` call is the entire DP logic condensed into one line.

```rust
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut b_count = 0;
        let mut res = 0;

        for c in s.bytes() {
            if c == b'a' {
                res = std::cmp::min(res + 1, b_count);
            } else {
                b_count += 1;
            }
        }
        res
    }
}
```

## Conclusion

The time complexity is $O(n)$ since we do a single pass through the string, and the space complexity is $O(1)$ since we only track two integers. What makes this problem satisfying is how the greedy choice at each `'a'` -- delete it or wipe out all previous `'b'`s -- naturally produces the global minimum without backtracking. It is a nice example of how dynamic programming can sometimes collapse into a constant-space linear scan when the state is small enough.
