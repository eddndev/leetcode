---
title: "0174 Dungeon Game - EN"
problemUrl: "https://leetcode.com/problems/dungeon-game/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "matrix"]
complexity:
  time: "O(M * N)"
  space: "O(N)"
---

# The Knight Who Walked Backwards

## The Problem
Demons have captured the princess and imprisoned her in the bottom-right corner of a dungeon. The dungeon consists of an `M x N` grid of rooms. Our brave knight starts in the top-left room and must fight his way to the princess. The knight has an initial health point total, and in each room he either gains or loses health depending on the room's value (positive for magic orbs, negative for demons). The knight's health must remain at least 1 at all times. We need to determine the minimum initial health required for the knight to rescue the princess.

## The Trap of the Forward Path

My first instinct was to attack this as a classic forward DP problem: start at the top-left corner, and for each cell compute the minimum initial health needed to have reached it. But this leads to a subtle dead end. When advancing cell by cell, I need two pieces of information simultaneously: the *accumulated* health and the *minimum* health reached along the path. These two values interact in ways that cannot be optimized with a single DP table, because a path with lower accumulated health might have a higher local minimum, and vice versa. The principle of optimality breaks down.

The solution is an elegant mental reversal: walk backwards. If I start from the princess and work my way back to the knight, at each cell I only need to answer one question: what is the minimum health I need to have *upon entering* this cell in order to reach the princess from here? That question has a unique answer and decomposes cleanly.

## The Logic of Reversal

Define `dp[j]` as the minimum health needed upon entering cell `(i, j)` in order to reach cell `(m-1, n-1)`. We iterate from the bottom-right corner toward the top-left.

For each cell, the knight can only move right or down. Therefore, the minimum health needed upon entering depends on the minimum between what he would need if he goes right (`dp[j+1]`) and what he would need if he goes down (`dp[j]` from the previous row, which in our in-place iteration is still the current value):

```
min_hp_next = min(dp[j], dp[j+1])
need = min_hp_next - dungeon[i][j]
```

If the cell contains a large positive orb, `need` could drop to zero or below, which would mean the knight could enter the cell dead. But the rule states he must always have at least 1 health point, so we apply `max(need, 1)`.

### The Initialization Trick

I use an array of size `n+1` initialized with `i32::MAX`. This acts as a "wall" for border cells: a cell in the last row cannot go down, and a cell in the last column cannot go right. The `MAX` ensures those forbidden directions are never chosen by the `min`. The sole exception is `dp[n-1] = 1`, which establishes that upon reaching the princess (before entering her cell), we need at least 1 health point coming from the next step.

### A Concrete Example

Consider the dungeon:
```
[[-2, -3,  3],
 [-5, -10, 1],
 [10,  30, -5]]
```

Starting from `(2,2)`: I need to survive the cell `-5`, and there are no more cells after. So I need `max(1 - (-5), 1) = 6` upon entering.

Then `(2,1)`: the cell is worth `30`, and going right requires 6. Need: `6 - 30 = -24`. Since it's negative, the knight only needs 1 health point to enter here.

Following the complete backward pass, the answer turns out to be **7**: the knight needs to start with 7 health points to survive the optimal path.

## Rust Solution

```rust
use std::cmp;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();

        let mut dp = vec![i32::MAX; n + 1];

        dp[n - 1] = 1;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let min_hp_next = cmp::min(dp[j], dp[j + 1]);

                let need = min_hp_next - dungeon[i][j];

                dp[j] = if need <= 0 { 1 } else { need };
            }
        }

        dp[0]
    }
}
```

The Rust implementation is remarkably compact for a Hard problem. The `dp` vector of size `n + 1` acts as a sliding row that gets reused from bottom to top. When we iterate `j` from right to left, `dp[j]` still holds the value from the row below (the cost of going down), while `dp[j+1]` has already been updated for the current row (the cost of going right). This overlap is exactly what we need: `cmp::min(dp[j], dp[j+1])` gives us the minimum across both directions without maintaining two arrays. The condition `if need <= 0 { 1 } else { need }` is an idiomatic way to express `max(need, 1)`, ensuring the knight never "enters dead" into any cell.

## Conclusion

This problem is a reminder that the direction in which you traverse a state space matters profoundly. Going forward, the problem is intractable with standard DP because the future affects the interpretation of the past. Going backward, each cell has a clean, self-contained answer that depends only on its already-computed neighbors. It is the same dungeon, the same demons, the same orbs -- but viewed from the princess back toward the knight, the fog lifts and the path becomes clear.
