---
title: "0330 Patching Array - EN"
problemUrl: "https://leetcode.com/problems/patching-array/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["greedy", "array", "number-theory"]
complexity:
  time: "O(M + log N) where M is the length of nums and N is the target"
  space: "O(1)"
---

# Filling the Gaps in the Number Line

## The Problem
Given a sorted integer array `nums` and an integer `n`, add the minimum number of patches (elements) to the array such that any number in the range `[1, n]` can be formed by the sum of some elements in the array. Return the minimum number of patches required.

## The Initial Intuition

This problem hides a beautiful greedy insight behind what initially looks like a combinatorial nightmare. If I have to guarantee that every integer from 1 to `n` can be represented as a sum of elements, the brute force approach of checking subsets is clearly hopeless. Instead, I need to think about coverage -- what range of sums can I currently represent, and what happens when I encounter a gap?

The key concept is a variable I call `miss`: the smallest number that I cannot yet form as a sum of elements I have processed so far. If I can form every number in `[1, miss)`, then my current coverage extends up to `miss - 1`. The question becomes: how do I extend this coverage efficiently?

## Why the Greedy Strategy Works

Suppose I can currently form every sum in `[1, miss)`. If the next number in my sorted array is less than or equal to `miss`, I can absorb it: by adding it to every sum I could already form, I now cover `[1, miss + nums[i])`. This is because any sum in `[miss, miss + nums[i])` can be created by taking the new element plus some subset that sums to the remainder, and that remainder is guaranteed to be in `[1, miss)` which I already cover.

But if the next number exceeds `miss`, there is a gap. No existing element can help me reach `miss` itself. The optimal patch in this case is to add `miss` itself. Why? Because adding `miss` doubles my coverage to `[1, 2 * miss)`. Any smaller patch would extend coverage less. Any larger patch would still leave `miss` unreachable. Adding `miss` is provably optimal -- it is the single number that maximally extends the range of representable sums.

## Walking Through an Example

Consider `nums = [1, 5, 10]` and `n = 20`.

Starting with `miss = 1`. The first element is `1`, which is `<= miss`, so I absorb it: `miss` becomes `1 + 1 = 2`. Now I cover `[1, 2)`.

Next element is `5`, but `5 > miss = 2`. I cannot form `2` yet, so I patch by adding `2` itself. Now `miss = 2 + 2 = 4`, covering `[1, 4)`. Patch count: 1.

Still looking at `5`, and now `5 > miss = 4`. I patch again with `4`. Now `miss = 4 + 4 = 8`, covering `[1, 8)`. Patch count: 2.

Now `5 <= miss = 8`, so I absorb it: `miss = 8 + 5 = 13`, covering `[1, 13)`.

Next element is `10`, and `10 <= 13`, so I absorb it: `miss = 13 + 10 = 23`, covering `[1, 23)`.

Since `23 > 20 = n`, I am done. Answer: 2 patches.

## The Doubling Argument

The reason this algorithm is so efficient is the doubling behavior when patching. Each time I patch, `miss` doubles. Starting from 1, after `k` patches (in the worst case with no array elements to absorb), `miss` reaches `2^k`. To cover up to `n`, I need at most `log2(n)` patches. Combined with the single pass through the array, the total time is `O(M + log N)`.

This is remarkably elegant: a problem that seems to require exponential search over subsets reduces to a linear scan with logarithmically many patches.

## Rust Solution

```rust
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut patches = 0;
        let mut miss: i64 = 1;
        let mut i = 0;
        let limit = n as i64;

        while miss <= limit {
            if i < nums.len() && (nums[i] as i64) <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                miss += miss;
                patches += 1;
            }
        }

        patches
    }
}
```

The implementation is strikingly concise for a Hard problem. The variable `miss` is declared as `i64` to avoid overflow -- since `n` can be up to `2^31 - 1`, doubling `miss` could exceed `i32` range during intermediate computations. The main loop continues as long as `miss <= limit`, meaning there are still values in `[1, n]` that need to be coverable. Inside the loop, there are exactly two cases: either the current array element fits within the coverage frontier and extends it additively, or a gap exists and I patch by doubling `miss`. The index `i` only advances when an array element is absorbed, naturally handling the case where I run out of array elements -- at that point, every iteration patches, and the doubling ensures rapid convergence toward `n`.

## Conclusion

Patching Array is one of those problems where the solution is deceptively simple once you see the right invariant. The `miss` variable -- tracking the smallest unreachable sum -- transforms a seemingly intractable coverage problem into a clean greedy algorithm. The proof of optimality follows from the fact that patching with `miss` itself is always the best choice: it maximally extends coverage by doubling it. The result is an `O(M + log N)` algorithm with constant space, encoded in barely a dozen lines of Rust. It is a reminder that the hardest problems sometimes have the shortest solutions.
