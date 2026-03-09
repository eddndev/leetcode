---
title: "0135 Candy - EN"
problemUrl: "https://leetcode.com/problems/candy/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["array", "greedy"]
complexity:
  time: "O(N)"
  space: "O(N)"
---

# Distributing Candy with Fairness

## The Problem
There are `n` children standing in a line. Each child is assigned a rating value given in the integer array `ratings`. We want to distribute candies to these children following two rules: each child must receive at least one candy, and a child with a higher rating than an immediate neighbor must receive more candies than that neighbor. Return the minimum total number of candies needed.

## The Trap of Trying to See Everything at Once

When I first encountered this problem, my instinct was to try solving everything in a single pass: compare each child with both neighbors simultaneously and assign candies immediately. But that doesn't work. The conflict is that the correct number of candies for a child depends on what happens both to their left and to their right, and those two perspectives can be in tension. A child might need 3 candies because of their relationship with the left neighbor but 5 because of the right neighbor. You can't reconcile both constraints in a single pass.

The key insight was decomposing the problem into two independent subproblems: first satisfy all constraints looking only leftward, then satisfy all constraints looking only rightward. Finally, combine both results.

## Two Passes: Left and Right

The strategy works as follows:

1. **Initialize:** Create a `candies` array of the same size as `ratings`, with all values set to 1. This satisfies the first rule: every child gets at least one candy.

2. **Left-to-right pass:** Traverse from the second child to the last. If `ratings[i] > ratings[i - 1]`, then child `i` deserves more candies than child `i - 1`, so we set `candies[i] = candies[i - 1] + 1`. If the rating is not higher, we leave the value at 1. After this pass, all constraints of the form "a child with a better rating than their left neighbor has more candies" are satisfied.

3. **Right-to-left pass:** Traverse from the second-to-last child back to the first. If `ratings[i] > ratings[i + 1]`, child `i` needs more candies than child `i + 1`. But we can't simply assign `candies[i + 1] + 1`, because that might violate the left constraint we already satisfied. The solution is to take the maximum: `candies[i] = max(candies[i], candies[i + 1] + 1)`. This preserves what the first pass established while simultaneously fulfilling the new constraint.

4. **Sum:** The total number of candies is the sum of the array.

### A Concrete Example

For `ratings = [1, 0, 2]`:
- Initialize: `candies = [1, 1, 1]`
- Left pass: `ratings[1]=0` is not greater than `ratings[0]=1`, stays. `ratings[2]=2 > ratings[1]=0`, so `candies[2] = candies[1] + 1 = 2`. Result: `[1, 1, 2]`
- Right pass: `ratings[0]=1 > ratings[1]=0`, so `candies[0] = max(1, 1 + 1) = 2`. `ratings[1]=0` is not greater than `ratings[2]=2`, stays. Result: `[2, 1, 2]`
- Total: **5** candies.

The middle child with rating 0 receives only 1 candy, and both neighbors with higher ratings receive 2 each. Both rules are satisfied with the minimum possible amount.

### Why the Maximum is Correct

The critical moment is the `max` in the second pass. Without it, we could destroy a constraint already satisfied. Suppose after the left pass a child has 4 candies because there's a long increasing sequence to their left. If in the right pass we discover they need at least 2 due to their right neighbor, we must not drop to 2; we must stay at 4. The maximum guarantees that both constraints are satisfied simultaneously: it's the operation that turns two partial solutions into a global solution.

## Rust Solution

```rust
use std::cmp;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candies = vec![1; n];

        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = cmp::max(candies[i], candies[i + 1] + 1);
            }
        }

        candies.iter().sum()
    }
}
```

The Rust implementation is concise and direct. The `vec![1; n]` idiomatically initializes all candies to 1. The first pass traverses with `1..n`, and the second uses `(0..n - 1).rev()` to iterate in reverse, an elegant pattern that Rust allows at no extra cost thanks to lazy iterators. The use of `cmp::max` in the second pass is the sole point where both constraints meet and are reconciled. Finally, `candies.iter().sum()` collapses the array into the final result, leveraging Rust's iterator traits. There are no unnecessary allocations or hidden complexity: just two linear traversals and a sum.

## Conclusion

This problem seems complicated because each child depends on two neighbors, creating a bidirectional constraint system. But the fundamental observation is that the constraints are separable by direction. By processing all left constraints first and then all right constraints, we transform a seemingly global problem into two local problems that are solved with simple linear traversals. The `max` at the end acts as a union operator that merges both partial solutions without violating either. It's a classic lesson in greedy programming: when constraints are hard to satisfy together, sometimes it's enough to satisfy them separately and combine them.
