---
title: "0321 Create Maximum Number - EN"
problemUrl: "https://leetcode.com/problems/create-maximum-number/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["greedy", "stack", "monotonic-stack", "merge"]
complexity:
  time: "O(K * (M + N)) where M and N are the lengths of the two arrays"
  space: "O(M + N)"
---

# Forging the Largest Number from Two Decks

## The Problem
Given two integer arrays `nums1` and `nums2` of lengths `m` and `n` respectively, and an integer `k` where `k <= m + n`, create the maximum number of length `k` from digits of the two arrays. The relative order of the digits from the same array must be preserved. Return the array of `k` digits representing the largest possible number.

## The Initial Intuition

My first thought was dynamic programming, but the state space explodes quickly when I need to track positions in both arrays and the number of digits selected. Instead, I noticed that the problem can be cleanly decomposed into three independent subproblems:

1. **How many digits should I take from each array?** If I take `i` digits from `nums1`, I take `k - i` from `nums2`.
2. **Which digits should I pick from each array?** For a fixed count, I want the *largest possible subsequence* of that length.
3. **How do I combine two subsequences into the largest merged sequence?** This is a greedy merge, not a simple interleave.

The beauty is that each of these subproblems has a clean, efficient solution, and composing them gives the overall answer.

## Splitting the Budget

I iterate over all valid splits: take `i` digits from `nums1` and `k - i` from `nums2`, where `i` ranges from `max(0, k - n)` to `min(k, m)`. For each split, I extract the best subsequence from each array, merge them, and keep the overall best candidate.

## Extracting the Maximum Subsequence

Given an array and a target length `k`, I want the lexicographically largest subsequence of length `k`. This is a classic monotonic stack problem. I scan the array left to right, maintaining a stack. For each new element, while the stack's top is smaller than the current element *and* I still have enough remaining elements to fill the stack to length `k`, I pop the top. Then I push the current element (if the stack isn't full yet).

The variable `drop` counts how many elements I'm allowed to discard. Initially it's `n - k` (the total number of elements I must skip). Every time I pop from the stack, I've effectively discarded one element, so I decrement `drop`. At the end, I truncate the stack to exactly `k` elements to handle the case where I never popped enough.

### A Quick Example

For `nums = [9, 1, 2, 5, 8, 3]` and `k = 3`:

- `drop = 3`. Stack: `[]`
- `9`: push. Stack: `[9]`
- `1`: `1 < 9`, just push. Stack: `[9, 1]`
- `2`: `2 > 1`, pop `1` (drop=2), `2 < 9`, push. Stack: `[9, 2]`
- `5`: `5 > 2`, pop `2` (drop=1), `5 < 9`, push. Stack: `[9, 5]`
- `8`: `8 > 5`, pop `5` (drop=0), `8 < 9`, push. Stack: `[9, 8]`
- `3`: drop=0 so no popping, push. Stack: `[9, 8, 3]`

Result: `[9, 8, 3]` -- the largest 3-digit subsequence.

## The Merge: Lexicographic Greedy

Merging two subsequences into the largest possible sequence is trickier than it looks. At each step, I compare the *remaining* portions of both subsequences lexicographically, not just their front elements. If `s1[i..] > s2[j..]`, I take from `s1`; otherwise from `s2`.

Why can't I just compare the front elements? Consider merging `[6, 7]` and `[6, 0, 4]`. Both start with `6`, but taking from `[6, 7]` first gives `[6, 6, 7, 0, 4]` while taking from `[6, 0, 4]` first gives `[6, 6, 0, 7, 4]`. The first is clearly better. Comparing the full suffixes `[6, 7] > [6, 0, 4]` correctly tells me to take from the first array.

In Rust, this comparison is beautifully simple: slice comparison (`s1[i..] > s2[j..]`) performs lexicographic comparison out of the box.

## Rust Solution

```rust
impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let k = k as usize;
        let mut best_result = vec![0; k];

        let start = if k > n2 { k - n2 } else { 0 };
        let end = if k < n1 { k } else { n1 };

        for i in start..=end {
            let len1 = i;
            let len2 = k - i;

            let sub1 = Self::get_max_subsequence(&nums1, len1);
            let sub2 = Self::get_max_subsequence(&nums2, len2);

            let candidate = Self::merge(&sub1, &sub2);

            if candidate > best_result {
                best_result = candidate;
            }
        }

        best_result
    }

    fn get_max_subsequence(nums: &Vec<i32>, k: usize) -> Vec<i32> {
        let mut stack = Vec::with_capacity(k);
        let n = nums.len();
        let mut drop = n - k;

        for &val in nums {
            while drop > 0 && !stack.is_empty() && val > *stack.last().unwrap() {
                stack.pop();
                drop -= 1;
            }
            stack.push(val);
        }

        stack.truncate(k);
        stack
    }

    fn merge(s1: &Vec<i32>, s2: &Vec<i32>) -> Vec<i32> {
        let len = s1.len() + s2.len();
        let mut res = Vec::with_capacity(len);
        let mut i = 0;
        let mut j = 0;

        while i < s1.len() || j < s2.len() {
            if i < s1.len() && (j == s2.len() || s1[i..] > s2[j..]) {
                res.push(s1[i]);
                i += 1;
            } else {
                res.push(s2[j]);
                j += 1;
            }
        }
        res
    }
}
```

The implementation decomposes neatly into three functions mirroring the three subproblems. `max_number` orchestrates the split enumeration, initializing `best_result` as a vector of zeros (the smallest possible candidate) and updating it whenever a better candidate is found via direct vector comparison. The bounds `start` and `end` ensure I never ask for more digits than an array can provide. `get_max_subsequence` implements the monotonic stack approach with `with_capacity(k)` for a single allocation and a `truncate` at the end for safety. The `merge` function leverages Rust's native slice comparison `s1[i..] > s2[j..]`, which performs lexicographic comparison element by element -- an elegant one-liner that replaces what would be a manual loop in most other languages.

## Conclusion

Create Maximum Number is a problem that rewards decomposition. What initially looks like a monolithic optimization problem splits into three well-understood pieces: enumerating the split, extracting maximum subsequences with a monotonic stack, and merging with lexicographic comparison. Each piece is individually straightforward, and their composition yields the optimal solution. Rust's native slice comparison is the cherry on top, making the merge logic both correct and concise without needing a custom comparator.
