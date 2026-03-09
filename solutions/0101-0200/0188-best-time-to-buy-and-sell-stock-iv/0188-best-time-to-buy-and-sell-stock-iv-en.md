---
title: "0188 Best Time to Buy and Sell Stock IV - EN"
problemUrl: "https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["array", "dynamic-programming"]
complexity:
  time: "O(N * K)"
  space: "O(K)"
---

# K Rounds in the Trading Ring: Squeezing Maximum Profit from Limited Transactions

## The Problem
You are given an integer `k` and an array `prices` where `prices[i]` is the price of a given stock on the `i`th day. Find the maximum profit you can achieve. You may complete at most `k` transactions (a transaction is one buy followed by one sell). Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).

## The First Impression

This is the generalized form of the buy-and-sell-stock family. When `k = 1` it is the classic single-trade problem; when `k = 2` it is the well-known two-transaction variant. But here `k` can be anything, so we need a framework that scales gracefully with the number of allowed trades.

A brute-force approach -- enumerating all possible combinations of up to `k` non-overlapping buy-sell pairs -- explodes combinatorially. The key observation is that on any given day, our optimal state depends only on how many transactions we have completed so far and whether we are currently holding a stock. This is a textbook setup for **dynamic programming over a state machine**: for each transaction slot `j` from `1` to `k`, we maintain two values -- the best position if we are currently holding a stock after buying into our `j`th trade (`buy[j]`), and the best profit after having sold in our `j`th trade (`sell[j]`).

There is one critical shortcut that prevents the solution from timing out on edge cases: if `k >= n / 2` (where `n` is the number of days), then we effectively have unlimited transactions. In that scenario, every upward price movement can be captured greedily, and we collapse the entire problem to a simple linear scan summing all positive day-to-day differences.

## The State Machine with K Layers

We maintain two arrays of length `k + 1`:

- **`buy[j]`**: the maximum value we can have if we are holding a stock and have entered our `j`th transaction. Initialized to a large negative number (representing the impossibility of holding a stock before any day has been observed).
- **`sell[j]`**: the maximum profit after completing the `j`th sell. Initialized to zero -- doing nothing is always valid.

### The transitions

For each price in the array, and for each transaction slot `j` from `1` to `k`:

1. `buy[j] = max(buy[j], sell[j - 1] - price)` -- should we start the `j`th trade by buying at today's price? The effective cost is the profit banked from the first `j - 1` trades minus today's price.
2. `sell[j] = max(sell[j], buy[j] + price)` -- should we close the `j`th trade by selling today? The profit is whatever our holding position was plus today's price.

Because `sell[j - 1]` feeds into `buy[j]`, the profit from earlier trades cascades naturally into later ones. And because every update is a `max`, we only ever improve each state -- the order of updates within a single day does not cause interference.

### The unlimited-transactions shortcut

When `k >= n / 2`, no schedule of trades could possibly use more than `n / 2` transactions (since each trade requires at least two distinct days). In this regime we switch to a greedy strategy: walk through the prices with a sliding window of size two, and whenever tomorrow's price exceeds today's, pocket the difference. This runs in O(N) time and sidesteps the O(N * K) loop entirely, which is critical because `k` can be as large as 10^9.

### Why this handles edge cases

If fewer than `k` profitable trades exist, the extra transaction slots simply remain at zero profit -- they never degrade the answer. If the price array has fewer than two elements, or `k` is zero, there is nothing to trade and we return zero immediately. The algorithm handles all these situations without special-case branches in the main loop.

## Rust Solution

```rust
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 || k == 0 {
            return 0;
        }
        let k = k as usize;

        if k >= n / 2 {
            return prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum();
        }

        let mut buy = vec![-1_000_000_000; k + 1];
        let mut sell = vec![0; k + 1];

        for price in prices {
            for j in 1..=k {
                buy[j] = buy[j].max(sell[j - 1] - price);

                sell[j] = sell[j].max(buy[j] + price);
            }
        }

        sell[k]
    }
}
```

The Rust implementation is compact and expressive. The early returns handle degenerate inputs -- fewer than two prices or zero allowed transactions. The `k >= n / 2` check then dispatches the unlimited-transactions case using `prices.windows(2)`, a beautifully idiomatic way to iterate over consecutive pairs, summing only the positive differences with `.max(0)`.

For the general case, `buy` and `sell` are heap-allocated vectors of size `k + 1`. The sentinel value `-1_000_000_000` serves as a practical negative infinity -- large enough to be dominated by any real price difference while avoiding overflow when we add a price (since prices are at most 1000 and `k` is bounded). The nested loop iterates over each price and each transaction slot, performing exactly two `max` operations per `(price, j)` pair. At the end, `sell[k]` holds the maximum profit achievable with at most `k` complete trades. The overall complexity is O(N * K) in time and O(K) in space -- the best we can do for the general case without resorting to more exotic data structures.

## Conclusion

Problem 188 is the capstone of the stock-trading series, asking us to generalize from fixed transaction counts to an arbitrary `k`. The solution is a natural extension of the state-machine approach: instead of four hardcoded variables, we use two arrays of length `k` and cascade profits from each completed trade into the next purchase. The critical optimization -- detecting when `k` is large enough to allow unlimited trades and switching to a greedy linear scan -- prevents the algorithm from choking on pathological inputs where `k` dwarfs the number of days. It is a satisfying demonstration that clean abstraction (the state machine) and pragmatic shortcuts (the greedy fallback) can coexist in the same solution, each covering the other's weakness.
