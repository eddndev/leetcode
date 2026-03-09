---
title: "0312 Burst Balloons - EN"
problemUrl: "https://leetcode.com/problems/burst-balloons/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "interval-dp"]
complexity:
  time: "O(N^3)"
  space: "O(N^2)"
---

# The Last Balloon Standing

## The Problem
Given an array of integers `nums` representing balloons, where each balloon has a number painted on it. We are asked to burst all the balloons. When bursting balloon `i`, we collect `nums[i-1] * nums[i] * nums[i+1]` coins. If `i-1` or `i+1` go out of bounds of the array, they are treated as having a `1`. Find the maximum coins we can collect by bursting all the balloons.

## The Initial Intuition

My first instinct was to think of this as a backtracking problem: try every possible order of bursting balloons and keep the one that yields the most coins. But with up to 300 balloons, that's N! permutations, something completely infeasible.

What makes this problem particularly tricky is that bursting a balloon changes the neighbors of the remaining balloons. If I burst the middle balloon in `[3, 1, 5]`, now 3 and 5 become neighbors. That forward dependency makes it hard to think in terms of subproblems.

And here is the mental twist that changes everything: **instead of thinking about which balloon to burst first, think about which balloon to burst last**.

## Inverting the Thinking

If I decide that balloon `k` will be the last one to burst within a range `(left, right)`, then I know that when I burst it, its neighbors will be exactly `left` and `right` (the range boundaries, which haven't been burst yet). The coins I collect are `nums[left] * nums[k] * nums[right]`.

And the best part: the balloons to the left of `k` and the balloons to the right of `k` form completely independent subproblems. Those on the left know nothing about those on the right, because `k` is still there separating them. This is exactly what we need for dynamic programming.

To simplify boundary handling, we wrap the array with two virtual `1`s: one at the beginning and one at the end. So an array like `[3, 1, 5, 8]` becomes `[1, 3, 1, 5, 8, 1]`. Now the problem is to find the maximum coins from bursting all balloons between index `0` and `len-1` (the two virtual `1`s are never burst).

We define `dp[left][right]` as the maximum coins obtainable by bursting all balloons strictly between `left` and `right`. We iterate by window size: starting with windows of size 2 (where there are no balloons in between, so `dp = 0`), and growing from there. For each window `(left, right)`, we try every balloon `k` between them as the last one to burst:

```
dp[left][right] = max(dp[left][k] + dp[k][right] + nums[left] * nums[k] * nums[right])
```

The final answer is in `dp[0][len-1]`.

### A Step-by-step Example

For `nums = [3, 1, 5, 8]`, the padded array is `[1, 3, 1, 5, 8, 1]`:

- Windows of size 2 (no balloons in between): all `dp = 0`
- Window `(0, 2)`: only `k=1` (balloon 3). Coins = `1*3*1 = 3`. `dp[0][2] = 3`
- Window `(1, 3)`: only `k=2` (balloon 1). Coins = `3*1*5 = 15`. `dp[1][3] = 15`
- Window `(2, 4)`: only `k=3` (balloon 5). Coins = `1*5*8 = 40`. `dp[2][4] = 40`
- Window `(3, 5)`: only `k=4` (balloon 8). Coins = `5*8*1 = 40`. `dp[3][5] = 40`
- Larger windows combine previous results, and so on...
- The final answer in `dp[0][5] = 167`.

## Rust Solution

```rust
impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut padded_nums = Vec::with_capacity(n + 2);
        padded_nums.push(1);
        padded_nums.extend(nums);
        padded_nums.push(1);

        let len = padded_nums.len();

        let mut dp = vec![vec![0; len]; len];

        for window in 2..len {
            for left in 0..len - window {
                let right = left + window;

                for k in (left + 1)..right {
                    let coins = dp[left][k]
                        + dp[k][right]
                        + (padded_nums[left] * padded_nums[k] * padded_nums[right]);

                    if coins > dp[left][right] {
                        dp[left][right] = coins;
                    }
                }
            }
        }

        dp[0][len - 1]
    }
}
```

The Rust implementation mirrors the idea directly. First we build `padded_nums` with the two sentinel `1`s using `with_capacity` for a single allocation. The `dp` table is a square matrix of size `len x len`, and the triple nested loop does all the work: the outer one controls the window size, the middle one the window position, and the inner one tries each candidate `k` as the last balloon to burst. The manual comparison `if coins > dp[left][right]` avoids depending on `std::cmp::max` and is equally readable. At the end, `dp[0][len - 1]` holds the answer: the maximum coins possible from bursting all the original balloons.

## Conclusion

Burst Balloons is a problem that looks like pure backtracking until you make the right mental inversion. Thinking about the last balloon to burst instead of the first transforms a problem with tangled dependencies into perfectly independent subproblems suited for interval DP. The technique of adding sentinels at the boundaries elegantly simplifies edge case handling. It's one of those problems where the difficulty isn't in the implementation, but in finding the right way to decompose it.
