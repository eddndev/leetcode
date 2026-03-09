---
title: "0220 Contains Duplicate III - EN"
problemUrl: "https://leetcode.com/problems/contains-duplicate-iii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["sliding-window", "bucket-sort", "hash-map"]
complexity:
  time: "O(N) where N is the length of the array"
  space: "O(min(N, indexDiff)) for the bucket map"
---

# Catching Near-Duplicates With Invisible Buckets

## The Problem
Given an integer array `nums` and two integers `indexDiff` and `valueDiff`, return `true` if there exist two distinct indices `i` and `j` such that `abs(i - j) <= indexDiff` and `abs(nums[i] - nums[j]) <= valueDiff`.

## The Trap of Brute Force

The naive approach checks every pair of elements within the index window -- for each element, scan up to `indexDiff` positions ahead and compare values. This runs in O(N * indexDiff) time, which can degrade to O(N^2) when `indexDiff` is large. A sorted structure like a `BTreeSet` can improve this to O(N log(indexDiff)) by maintaining a sliding window of sorted elements and performing range queries. But can we do better?

The real challenge is that we need to simultaneously satisfy two constraints: indices must be close *and* values must be close. The index constraint naturally suggests a sliding window, but the value constraint requires something cleverer than brute-force comparison within that window.

## The Strategy: Bucket Sort in a Sliding Window

### The Bucket Insight

My key idea was to borrow from bucket sort. If I divide the number line into buckets of width `valueDiff + 1`, then two numbers that fall into the *same bucket* are guaranteed to differ by at most `valueDiff`. Two numbers in *adjacent buckets* might also satisfy the condition, but I only need to check those two neighbors -- not the entire window.

For example, with `valueDiff = 3`, I create buckets of width 4: bucket 0 holds values [0, 3], bucket 1 holds [4, 7], bucket 2 holds [8, 11], and so on. If two values land in the same bucket, their difference is at most 3. If they land in adjacent buckets, I compute the actual difference and check.

### Handling Negatives

Negative numbers complicate the bucketing. A naive division `val / w` doesn't produce evenly-spaced buckets for negatives because integer division rounds toward zero. My solution offsets negative values: for a negative `val`, the bucket id is `(val + 1) / w - 1`. This shifts the buckets so that [-4, -1] maps to bucket -1, [-8, -5] maps to bucket -2, and so on -- each bucket covers exactly `w` consecutive integers.

### The Sliding Window

To enforce the index constraint, I maintain the bucket map as a sliding window of size `indexDiff`. As I process each element at position `i`:

1. **Compute the bucket id** for the current value.
2. **Check the same bucket**: if it already contains an element, return `true` immediately. Since the window only holds elements within `indexDiff` positions, the index constraint is automatically satisfied.
3. **Check adjacent buckets**: if the neighboring bucket exists and its value differs by less than `w` from the current value, return `true`.
4. **Insert** the current value into its bucket.
5. **Evict** the element that falls out of the window: when `i >= indexDiff`, remove the element at position `i - indexDiff` from its bucket.

Because each bucket can hold at most one element at a time (if two elements mapped to the same bucket, we would have already returned `true`), the map never grows beyond `indexDiff + 1` entries.

### A Concrete Example

With `nums = [1, 5, 9, 1, 5, 9]`, `indexDiff = 2`, `valueDiff = 3`:

```
Bucket width w = 4

i=0, val=1:  bucket=0. No match. Insert {0: 1}.
i=1, val=5:  bucket=1. Check neighbor 0: |5-1|=4 >= 4, no. Insert {0: 1, 1: 5}.
i=2, val=9:  bucket=2. Check neighbor 1: |9-5|=4 >= 4, no. Insert {0: 1, 1: 5, 2: 9}.
             Evict i=0 (val=1, bucket=0). Map: {1: 5, 2: 9}.
i=3, val=1:  bucket=0. Check neighbor 1: |1-5|=4 >= 4, no. Insert {0: 1, 1: 5, 2: 9}.
             Evict i=1 (val=5, bucket=1). Map: {0: 1, 2: 9}.
i=4, val=5:  bucket=1. Check neighbor 0: |5-1|=4 >= 4, no.
                        Check neighbor 2: |5-9|=4 >= 4, no. Insert {0: 1, 1: 5, 2: 9}.
             Evict i=2 (val=9, bucket=2). Map: {0: 1, 1: 5}.
i=5, val=9:  bucket=2. Check neighbor 1: |9-5|=4 >= 4, no. Insert {0: 1, 1: 5, 2: 9}.
             Evict i=3 (val=1, bucket=0). Map: {1: 5, 2: 9}.

Result: false
```

No pair satisfies both constraints simultaneously.

### Why Only Three Buckets Matter

This is the elegant part. For any given value, the answer can only come from elements in three buckets: the same one (guaranteed match), or the two immediate neighbors (need a distance check). Elements in buckets two or more away are guaranteed to differ by more than `valueDiff`. This reduces each lookup to O(1) -- three hash map queries regardless of the window size or value range.

## Rust Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        if value_diff < 0 {
            return false;
        }

        let mut buckets: HashMap<i64, i64> = HashMap::new();
        let w = value_diff as i64 + 1;

        let get_bucket_id = |val: i64| -> i64 {
            if val >= 0 {
                val / w
            } else {
                (val + 1) / w - 1
            }
        };

        for (i, &num) in nums.iter().enumerate() {
            let val = num as i64;
            let bucket_id = get_bucket_id(val);

            if buckets.contains_key(&bucket_id) {
                return true;
            }

            if let Some(&neighbor) = buckets.get(&(bucket_id - 1)) {
                if (val - neighbor).abs() < w {
                    return true;
                }
            }

            if let Some(&neighbor) = buckets.get(&(bucket_id + 1)) {
                if (val - neighbor).abs() < w {
                    return true;
                }
            }

            buckets.insert(bucket_id, val);

            if i as i32 >= index_diff {
                let old_val = nums[i - index_diff as usize] as i64;
                let old_bucket = get_bucket_id(old_val);
                buckets.remove(&old_bucket);
            }
        }

        false
    }
}
```

The implementation uses `i64` throughout to avoid overflow when computing differences between `i32` values -- a pair like `(i32::MIN, i32::MAX)` would overflow an `i32` subtraction. The bucket width `w` is `value_diff + 1` because we want values differing by *exactly* `valueDiff` to land in the same bucket. The closure `get_bucket_id` handles negative values with the `(val + 1) / w - 1` formula, ensuring uniform bucket sizes across the positive-negative boundary. The early return `if value_diff < 0` guards against an impossible constraint. The eviction logic at the bottom removes the element that just slid out of the window, maintaining the invariant that the map only contains elements within `indexDiff` positions of the current index.

## Conclusion

Contains Duplicate III is a problem that rewards thinking about value proximity in structural terms rather than numerical ones. By partitioning the number line into buckets of width `valueDiff + 1`, we transform a range query into a constant-time hash map lookup. The sliding window maintains the index constraint with simple insertion and eviction. The result is a clean O(N) algorithm that processes each element exactly once with three hash lookups per step -- no trees, no sorting, just the right abstraction applied to the right constraint.
