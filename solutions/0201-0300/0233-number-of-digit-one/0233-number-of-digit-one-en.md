---
title: "0233 Number of Digit One - EN"
problemUrl: "https://leetcode.com/problems/number-of-digit-one/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["math", "digit-dp", "combinatorics"]
complexity:
  time: "O(log N) where N is the input number"
  space: "O(1) constant extra space"
---

# Counting Every Hidden One

## The Problem
Given an integer `n`, count the total number of times the digit `1` appears in all non-negative integers less than or equal to `n`. For example, given `n = 13`, the integers from 1 to 13 are `1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13`, and the digit `1` appears a total of 6 times.

## The Trap of Brute Force

The naive approach is obvious: iterate from 1 to `n`, extract each digit of every number, and count the ones. But with `n` reaching up to 10^9, this means billions of numbers with up to 10 digits each. That is far too slow. The real question is whether we can compute the answer mathematically, without visiting every number.

## The Insight: Counting Position by Position

Instead of counting ones *per number*, I count ones *per digit position*. For each position -- ones, tens, hundreds, and so on -- I calculate how many times the digit `1` appears across all numbers from 1 to `n`. The total is simply the sum across all positions.

### Dissecting a Position

Consider a specific position with place value `i` (where `i` cycles through 1, 10, 100, ...). For any number `n`, I split it into three parts relative to this position:

- **prefix**: the digits above position `i`, computed as `n / (i * 10)`
- **digit**: the digit at position `i`, computed as `(n / i) % 10`
- **suffix**: the digits below position `i`, computed as `n % i`

The count of ones at this position depends entirely on what `digit` is:

1. **If digit is 0**: The ones at this position come only from complete cycles of the prefix. Each full cycle of the higher digits contributes exactly `i` ones (one for each combination of the suffix digits). So the count is `prefix * i`.

2. **If digit is 1**: We get all the ones from complete cycles (`prefix * i`), plus a partial cycle. The partial cycle contributes `suffix + 1` ones -- the current digit is 1 for suffix values 0 through `suffix`. So the count is `prefix * i + suffix + 1`.

3. **If digit is 2 or greater**: The current cycle is fully completed. The count is `(prefix + 1) * i` -- all complete cycles including the current one.

### A Walkthrough with n = 314

Let me trace through `n = 314`:

**Ones position (i=1):** prefix = 31, digit = 4, suffix = 0. Since digit > 1: count = (31 + 1) * 1 = 32.

**Tens position (i=10):** prefix = 3, digit = 1, suffix = 4. Since digit = 1: count = 3 * 10 + (4 + 1) = 35.

**Hundreds position (i=100):** prefix = 0, digit = 3, suffix = 14. Since digit > 1: count = (0 + 1) * 100 = 100.

**Total:** 32 + 35 + 100 = 167 ones in all integers from 1 to 314.

### Why This Works

The mathematical structure comes from how decimal numbers cycle. In any contiguous range of 10 numbers (say 0-9, 10-19, etc.), the ones digit is `1` exactly once. In any range of 100 numbers, the tens digit is `1` exactly 10 times. In general, in any range of `10 * i` numbers, position `i` holds a `1` exactly `i` times. The prefix tells us how many complete cycles have passed, and the digit and suffix tell us how far into the current partial cycle we are.

## Rust Solution

```rust
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }

        let n = n as i64;
        let mut count = 0;
        let mut i = 1;

        while i <= n {
            let prefix = n / (i * 10);
            let digit = (n / i) % 10;
            let suffix = n % i;

            if digit == 0 {
                count += prefix * i;
            } else if digit == 1 {
                count += prefix * i + (suffix + 1);
            } else {
                count += (prefix + 1) * i;
            }

            if i > n / 10 {
                break;
            }
            i *= 10;
        }

        count as i32
    }
}
```

The implementation casts `n` to `i64` early on to avoid overflow when computing `i * 10` near the upper boundary of `i32`. The variable `i` represents the current place value and multiplies by 10 each iteration, so the loop runs at most 10 times for `n` up to 10^9. The guard `if i > n / 10 { break; }` prevents `i` from overflowing on the next multiplication -- once `i` exceeds `n / 10`, the next `i *= 10` would surpass `n` and the loop condition `i <= n` would terminate it anyway, but this early exit avoids the multiplication entirely. The three-way branch on `digit` directly encodes the mathematical formula: zero means no partial cycle contribution, one means a partial contribution of `suffix + 1`, and anything higher means the full cycle is complete. The final cast back to `i32` is safe because the answer is always bounded by `n * log10(n)`, well within `i32` range for valid inputs.

## Conclusion

The Number of Digit One problem rewards mathematical thinking over algorithmic complexity. By shifting perspective from "how many ones does each number contain" to "how many ones does each position contribute," we transform a problem that seems to require O(N) enumeration into one solvable in O(log N) time with constant space. The three cases -- digit below, at, or above 1 -- capture the complete cycle structure of decimal numbers with surgical precision. No data structures, no recursion, no memoization -- just arithmetic and a clean loop through the digit positions.
