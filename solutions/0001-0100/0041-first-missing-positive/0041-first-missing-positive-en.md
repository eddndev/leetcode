---
title: "0041 First Missing Positive - EN"
problemUrl: "https://leetcode.com/problems/first-missing-positive/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["array", "cyclic-sort", "in-place"]
complexity:
  time: "O(N)"
  space: "O(1)"
---

# The Number Nobody Invited

## The Problem
Given an unsorted integer array `nums`, find the smallest missing positive integer. The algorithm must run in O(N) time and use O(1) auxiliary space.

## The Trap of Obvious Solutions

When you read "find the smallest missing positive," the first idea is to sort the array and scan for the first gap. It works, but sorting costs O(N log N), and the problem demands O(N). The second idea is to use a HashSet: walk through the array, insert everything into the set, then iterate from 1 until you find the missing one. It runs in O(N), but the set consumes O(N) extra space, and the problem requires constant space.

The O(1) space constraint is what makes this problem Hard. I can't create auxiliary structures. I have to work with what I already have: the array itself.

## The Key Observation

There's a fundamental observation that unlocks everything: if the array has `n` elements, the answer always lies in the range `[1, n+1]`. In the best case, the array contains exactly the numbers `1, 2, 3, ..., n`, and the answer is `n+1`. In every other case, some number in `[1, n]` is missing.

This means negative numbers, zeros, and numbers greater than `n` are irrelevant. I can ignore them entirely. And if I only care about numbers from 1 to `n`, I can use the array itself as a makeshift hash table: number `k` should live at position `k-1`.

## Cyclic Sort: Every Number to Its Home

The technique is called **cyclic sort**. The idea is to walk through the array and, for each element, send it to the position where it "belongs." Number 1 goes to `nums[0]`, 3 goes to `nums[2]`, 5 goes to `nums[4]`. If a number is negative, zero, or greater than `n`, I simply skip it and move on.

The trick is in the `while`: for each position `i`, I don't advance until the current element is either in its correct spot or is a value I don't care about. Each swap places at least one number in its final position, so the total number of swaps across the entire array is at most `n`. This guarantees O(N) overall, even though it looks like there's a nested loop.

After rearranging, I make a second linear pass: the first position where `nums[i] != i + 1` gives me the answer. If every position is correct, the answer is `n + 1`.

## Rust Solution

```rust
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[(nums[i] - 1) as usize] != nums[i] {
                let target_index = (nums[i] - 1) as usize;
                nums.swap(i, target_index);
            }
        }

        for i in 0..n {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }

        (n + 1) as i32
    }
}
```

The `while` condition has three parts that work together as a precise filter. First, `nums[i] > 0` discards negatives and zeros. Second, `nums[i] <= n as i32` discards numbers outside the useful range. Third, `nums[(nums[i] - 1) as usize] != nums[i]` checks that the destination doesn't already hold the correct value, which prevents infinite loops when there are duplicates.

What I like about this implementation is that `nums.swap(i, target_index)` performs the exchange safely and expressively. Rust forces us to compute `target_index` before the swap because we can't take two mutable references to the same slice simultaneously with direct indexing. The `swap` method handles that internally in a safe way.

The casting between `i32` and `usize` is unavoidable because LeetCode defines the signature with `Vec<i32>`, but indices in Rust are `usize`. It's a bit verbose, but there's no runtime cost.

## Conclusion

This problem is a brilliant example of how the harshest constraint (O(1) space) is the one that guides you toward the most elegant solution. Instead of building an auxiliary structure, we turn the array itself into a map where each position answers the question "does number `i+1` exist?" Cyclic sort achieves this rearrangement in linear time with a simple invariant: each swap places one number in its permanent home. In the end, the first absent resident is our answer.
