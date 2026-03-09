---
title: "0123 Best Time to Buy and Sell Stock III - EN"
problemUrl: "https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["array", "dynamic-programming"]
complexity:
  time: "O(N)"
  space: "O(1)"
---

# Two Shots at the Market: Maximizing Profit with a Pair of Trades

## The Problem
You are given an array `prices` where `prices[i]` is the price of a given stock on the `i`th day. Find the maximum profit you can achieve. You may complete at most two transactions. Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).

## The First Impression

The single-transaction version of this problem is a classic warm-up: track the minimum price seen so far and greedily update the best profit. But the moment we allow *two* transactions, the landscape shifts dramatically. Now we're choosing two non-overlapping buy-sell intervals from the timeline, and the profit from the first trade can effectively subsidize the cost of the second. A naive approach would try every possible pair of intervals -- O(N^2) at best -- but that misses the elegant structure hiding beneath the surface.

The breakthrough insight is to think of the problem as a **state machine**. At any point in time, we're in one of four states: holding our first stock, having sold our first stock, holding our second stock, or having sold our second stock. Each state transition either costs money (buying) or earns money (selling), and we want to maximize the final state's value. This transforms a combinatorial nightmare into a single left-to-right scan.

## Four Variables, One Pass

Instead of building tables or splitting the array, we track four values as we walk through the prices:

- **`buy1`**: the best (least negative) cost of buying the first stock so far. We initialize it to negative infinity because before seeing any price, no purchase has been made.
- **`sell1`**: the best profit after completing the first sell. Starts at zero -- doing nothing is always an option.
- **`buy2`**: the best effective cost of buying the second stock, *after* pocketing the profit from the first trade. Also starts at negative infinity.
- **`sell2`**: the best profit after completing both trades. Starts at zero.

### The transitions

For each price, we update all four states in order:

1. `buy1 = max(buy1, -price)` -- should we buy here for the first time? The cost is `-price`.
2. `sell1 = max(sell1, buy1 + price)` -- should we sell our first stock here? Profit is what we paid (`buy1`, a negative number) plus the current price.
3. `buy2 = max(buy2, sell1 - price)` -- should we start the second trade here? The effective cost is the first trade's profit minus the current price.
4. `sell2 = max(sell2, buy2 + price)` -- should we complete the second trade here?

The beauty is that these four updates don't interfere with each other within the same iteration. Even though `buy1` might change before `sell1` is computed, the `max` operation ensures we only ever improve our best-known state. And because `buy2` builds on top of `sell1`, the profit from the first transaction flows naturally into the second.

### Why this handles edge cases

If only one profitable trade exists, `buy2` and `sell2` will effectively replicate the first trade or add zero profit, so `sell2` still yields the correct answer. If no profitable trade exists at all, everything stays at zero. The algorithm gracefully degrades without special-case branches.

## Rust Solution

```rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy1 = i32::MIN;
        let mut sell1 = 0;

        let mut buy2 = i32::MIN;
        let mut sell2 = 0;

        for price in prices {
            buy1 = buy1.max(-price);

            sell1 = sell1.max(buy1 + price);

            buy2 = buy2.max(sell1 - price);

            sell2 = sell2.max(buy2 + price);
        }

        sell2
    }
}
```

The Rust implementation mirrors the state machine with almost mathematical purity. We initialize both buy states to `i32::MIN` -- Rust's equivalent of negative infinity for 32-bit integers -- to represent the impossibility of having bought before seeing any price. The `max` method on `i32` keeps each update clean and branchless. Notice there's no need for `if` statements, temporary variables, or even indexing into the array -- the `for price in prices` iterator consumes the vector directly. The entire solution runs in a single pass with four integer variables, achieving O(N) time and O(1) space -- about as lean as an algorithm can get.

## Conclusion

This problem is a masterclass in state-machine thinking applied to dynamic programming. What initially looks like it demands splitting arrays or nesting loops reduces to four carefully ordered `max` operations per element. The key idea -- that the second purchase can absorb the first sale's profit as a discount -- is what makes the single-pass approach possible. It's a pattern that generalizes beautifully: for at most *k* transactions, you'd maintain *2k* variables and apply the same cascading logic. But for `k = 2`, the result is especially satisfying -- four variables, one loop, and an answer that's hard to beat.
