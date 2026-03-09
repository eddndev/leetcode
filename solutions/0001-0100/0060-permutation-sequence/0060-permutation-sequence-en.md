---
title: "0060 Permutation Sequence - EN"
problemUrl: "https://leetcode.com/problems/permutation-sequence/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["math", "recursion"]
complexity:
  time: "O(N^2)"
  space: "O(N)"
---

# Skipping Millions of Permutations with Factorials

## The Problem
Given two integers `n` and `k`, return the k-th permutation sequence of the numbers `[1, 2, ..., n]`. The set of permutations is ordered lexicographically, and `k` is 1-indexed.

## The Initial Intuition

The brute force approach would be to generate all `n!` permutations in order and pick the k-th one. For `n = 9`, that's 362,880 permutations. It works, but it's deeply wasteful: we're building hundreds of thousands of sequences only to throw them all away except one. There has to be a way to jump directly to the answer.

And there is. The key realization is that permutations have a very predictable structure when ordered lexicographically. If I have `n` numbers, the first `(n-1)!` permutations all start with the smallest number, the next `(n-1)!` start with the second smallest, and so on. This means I can determine each digit of the result by dividing and narrowing down, without ever generating a single permutation I don't need.

## The Factorial Number System Approach

Think of it like this: given `n = 4` and `k = 9`, the available digits are `[1, 2, 3, 4]` and there are `4! = 24` total permutations. The first digit partitions those 24 permutations into 4 groups of `3! = 6` each:

- Permutations 1-6 start with `1`
- Permutations 7-12 start with `2`
- Permutations 13-18 start with `3`
- Permutations 19-24 start with `4`

Since `k = 9` falls in the second group, the first digit is `2`. Now we've consumed 6 permutations (the entire first group), so we need the `(9 - 6) = 3`rd permutation of the remaining digits `[1, 3, 4]`.

We repeat the process. Among `[1, 3, 4]`, the `2! = 2` permutations per starting digit give us:

- Permutations 1-2 start with `1`
- Permutations 3-4 start with `3`
- Permutations 5-6 start with `4`

The 3rd falls in the second group, so the next digit is `3`. Now we need the 1st permutation of `[1, 4]`, which is simply `1, 4`.

Result: `"2314"`.

The implementation makes this even cleaner by converting `k` to 0-indexed at the start. With `k = k - 1`, the index into the available digits at each step is simply `k / factorial`, and the remainder `k % factorial` becomes the new `k` for the next step. No off-by-one gymnastics needed.

### Why O(N^2)?

Each step involves removing an element from a vector of remaining digits. That removal is O(N) because elements after the removed one must shift. We do this N times, giving O(N^2) overall. For the constraint `n <= 9`, this is completely negligible, but it's worth noting. If `n` were large, a balanced BST or a Fenwick tree could bring it down to O(N log N).

## Rust Solution

```rust
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n_usize = n as usize;
        let mut fact = vec![1; n_usize];
        for i in 1..n_usize {
            fact[i] = fact[i - 1] * i as i32;
        }

        let mut numbers: Vec<char> = (1..=n as u8).map(|digit| (b'0' + digit) as char).collect();

        let mut k = k - 1;
        let mut result = String::with_capacity(n_usize);

        for i in (0..n_usize).rev() {
            let factorial = fact[i];

            let index = (k / factorial) as usize;

            result.push(numbers.remove(index));

            k %= factorial;
        }

        result
    }
}
```

The Rust implementation is compact and expressive. The factorial table `fact` is built bottom-up with `fact[i]` holding `i!`. The `numbers` vector starts as `['1', '2', ..., 'n']` and shrinks by one element each iteration as digits are selected and removed. The 0-indexed `k` drives the entire selection process: `k / fact[i]` tells us which digit to pick, and `k %= fact[i]` narrows the search to the remaining positions. The use of `String::with_capacity` avoids reallocations, and `Vec::remove` handles both extraction and shifting in a single call.

## Conclusion

This problem is a beautiful example of how understanding the mathematical structure behind a combinatorial object can eliminate entire classes of computation. Instead of generating permutations, we decompose `k` in the factorial number system, essentially reading off the answer digit by digit. The approach is deterministic, allocation-light, and runs in time proportional to `n^2` in the worst case -- though for the given constraints, it's effectively instantaneous. Sometimes the best algorithm isn't about clever data structures or intricate recursion; it's about realizing that the answer was always encoded in the arithmetic.
