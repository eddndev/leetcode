---
title: "0440 K-th Smallest in Lexicographical Order - EN"
problemUrl: "https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["trie", "math"]
complexity:
  time: "O(log(n)^2), where n is the upper bound of the range"
  space: "O(1)"
---

# The Cartographer of the Infinite Dictionary

## The Problem
Given two integers `n` and `k`, find the k-th smallest number in lexicographical order within the range `[1, n]`. For example, if `n = 13` and `k = 2`, the numbers in lexicographical order are `[1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]`, and the second one is `10`.

## The Brute Force Trap

My first impulse was to generate all numbers from 1 to n, sort them lexicographically, and return the k-th one. But with `n` reaching up to `10^9`, that is not feasible in either time or memory. I need a way to navigate the lexicographical order without building the complete list.

## Think in Trees, Not Lists

The key insight is to visualize the numbers from 1 to n as a trie (prefix tree) with 10 branches. The root has children 1 through 9, each of those has children 0 through 9, and so on. A preorder traversal of this trie produces exactly the lexicographical order. So my question transforms: instead of visiting node by node, can I skip entire subtrees if I know k does not fall within them?

## Counting the Nodes in a Subtree

To decide whether I should descend into the subtree rooted at prefix `cur` or skip to the next sibling `cur + 1`, I need to know how many numbers live in the subtree of `cur`. This is computed level by level: at the first level there is just `cur` itself, at the second level there are numbers from `cur * 10` to `cur * 10 + 9`, at the third from `cur * 100` to `cur * 100 + 99`, and so on until the numbers exceed `n`. At each level, the valid numbers range from `first` to `min(n + 1, last) - 1`, where `first` and `last` define the boundaries of that level's range.

## The Navigation Strategy

I start with `cur = 1` and `k = k - 1` (because I am already standing on the first lexicographical number). At each step:

1. I calculate `steps`, the number of nodes in the subtree of `cur`.
2. If `steps <= k`, the k-th number is not in this subtree. I skip to the next sibling: `cur += 1` and subtract `steps` from `k`.
3. If `steps > k`, the number I am looking for is inside this subtree. I descend one level: `cur *= 10` and subtract 1 from `k` (because I consume the current node).

I repeat until `k` reaches 0, at which point `cur` is the answer.

## Walking Through an Example

With `n = 13`, `k = 2`:

- **Start**: `cur = 1`, `k = 1` (after subtracting 1).
- **Step 1**: Calculate the steps in the subtree of `1`: level 1 has `1` (the number 1 itself), level 2 has `min(14, 20) - 10 = 4` (the numbers 10, 11, 12, 13). Total: `steps = 5`. Since `5 > 1`, I descend: `cur = 10`, `k = 0`.
- **k = 0**: The answer is `10`.

## Rust Solution

```rust
impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut cur = 1;
        let mut k = k - 1;

        while k > 0 {
            let mut steps: i64 = 0;
            let mut first = cur as i64;
            let mut last = first + 1;
            let target = n as i64;

            while first <= target {
                steps += std::cmp::min(target + 1, last) - first;
                first *= 10;
                last *= 10;
            }

            if steps <= k as i64 {
                cur += 1;
                k -= steps as i32;
            } else {
                cur *= 10;
                k -= 1;
            }
        }
        cur
    }
}
```

The function uses `i64` for intermediate calculations because multiplying the level boundaries by 10 repeatedly can overflow an `i32`. The variable `first` tracks the start of the range at each level of the subtree, and `last` tracks the start of the next sibling's range at that same level. The condition `min(target + 1, last) - first` ensures we do not count numbers greater than `n`. The outer loop moves either horizontally (to the sibling) or vertically (to the child) depending on whether the subtree fits within the remaining `k` steps, navigating the virtual trie without ever constructing it.

## Conclusion

K-th Smallest in Lexicographical Order is a problem that reveals the hidden structure behind something as everyday as dictionary order. What appears to be a sorting problem becomes an intelligent navigation over an implicit trie, where counting the nodes of a subtree replaces the need to visit them one by one. The elegance of the solution lies in achieving O(log(n)^2) time and O(1) space, leaping over entire subtrees like an explorer who knows the map before setting foot on the terrain.
