---
title: "0410 Split Array Largest Sum - EN"
problemUrl: "https://leetcode.com/problems/split-array-largest-sum/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["binary-search", "greedy"]
complexity:
  time: "O(N * log(S)) where N is the length of the array and S is the sum of all elements"
  space: "O(1)"
---

# Splitting the Load Without Breaking the Scale

## The Problem
Given an integer array `nums` and an integer `k`, split `nums` into `k` non-empty contiguous subarrays such that the largest sum among these subarrays is minimized. Return the minimized largest sum.

## The Initial Intuition

When I first encountered this problem, my mind jumped to dynamic programming -- try every possible way to split the array into `k` parts and track the minimum of the maximum subarray sum. That approach works, but its O(N^2 * k) complexity feels heavy. Then a different perspective clicked: I am not really searching for how to split. I am searching for a threshold -- the smallest value such that the array can be split into at most `k` contiguous subarrays where no subarray exceeds that threshold.

Once I frame the question as "can I split the array into at most `k` parts, each summing to at most `mid`?", the problem transforms into a binary search over the answer space. The answer must lie between the maximum single element (because every subarray contains at least one element) and the total sum of the array (which is the sum when `k = 1`). Between those two bounds, I can binary search for the tightest threshold that still allows a valid split.

## The Binary Search Setup

I set `low` to the maximum element of `nums` and `high` to the total sum. Any valid answer must fall in this range. If I try a candidate `mid`, I ask: "Is it possible to split `nums` into at most `k` contiguous subarrays, each with sum at most `mid`?" If yes, I try a smaller threshold by setting `high = mid`. If no, the threshold is too tight and I raise it with `low = mid + 1`.

This is a classic binary search on the answer pattern. The monotonic property is clear: if I can split successfully with threshold `T`, then I can certainly split with any threshold greater than `T`. Conversely, if I cannot split with threshold `T`, I cannot split with anything smaller either.

## The Greedy Feasibility Check

The `can_split` function is the heart of the solution. Given the array, the number of allowed subarrays `k`, and a candidate `limit`, I greedily build subarrays from left to right. I maintain a running sum and keep adding elements to the current subarray. The moment adding the next element would push the sum past the limit, I close the current subarray and start a new one with that element. If at any point I have opened more than `k` subarrays, the limit is infeasible and I return `false`.

This greedy strategy works because I always want to pack as many elements as possible into each subarray before starting a new one. There is no benefit to closing a subarray early -- doing so would only force more subarrays, never fewer.

## Why It All Fits Together

The binary search makes O(log(S)) calls to `can_split`, where S is the sum of the array. Each call to `can_split` runs in O(N), performing a single linear pass through the array. This gives an overall complexity of O(N * log(S)), which is remarkably efficient for a problem that might initially seem to require exponential search. The space complexity is O(1) since I only use a handful of variables -- no auxiliary data structures, no memoization tables.

## Rust Solution

```rust
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut low = 0;
        let mut high = 0;

        for &num in nums.iter() {
            low = low.max(num);
            high += num;
        }

        while low < high {
            let mid = low + (high - low) / 2;

            if Self::can_split(&nums, k, mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        low
    }

    fn can_split(nums: &[i32], k: i32, limit: i32) -> bool {
        let mut count = 1;
        let mut current_sum = 0;

        for &num in nums {
            if current_sum + num > limit {
                count += 1;
                current_sum = num;

                if count > k {
                    return false;
                }
            } else {
                current_sum += num;
            }
        }

        true
    }
}
```

The initial loop computes both bounds simultaneously: `low` accumulates the maximum element and `high` accumulates the total sum. The binary search uses `low + (high - low) / 2` instead of `(low + high) / 2` to avoid integer overflow. In `can_split`, the counter starts at `1` because the first subarray is always open from the start. When `current_sum + num` exceeds the limit, a new subarray begins with `num` as its first element, and the counter increments. The early return when `count > k` is an optimization that avoids scanning the rest of the array when the answer is already determined. When the binary search converges, `low` equals `high` and holds the minimized largest sum.

## Conclusion

Split Array Largest Sum is a textbook example of binary search on the answer space. The insight that transforms this from a partitioning combinatorics problem into an elegant binary search is recognizing that the question "what is the minimum possible largest sum?" can be rephrased as "what is the smallest threshold that permits a valid greedy split?" The greedy feasibility check provides the monotonic predicate that binary search requires, and together they deliver an O(N * log(S)) solution that handles the problem's constraints with ease. What appears at first to demand exhaustive enumeration of partitions yields gracefully to a simple search over a single number line.
