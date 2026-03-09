---
title: "0327 Count of Range Sum - EN"
problemUrl: "https://leetcode.com/problems/count-of-range-sum/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["merge-sort", "divide-and-conquer", "prefix-sum"]
complexity:
  time: "O(N log N) where N is the length of the array"
  space: "O(N) for the prefix sum array and the merge buffer"
---

# Hunting Sums Between the Lines

## The Problem
Given an integer array `nums` and two integers `lower` and `upper`, return the number of range sums that lie in `[lower, upper]` inclusive. A range sum `S(i, j)` is defined as the sum of the elements in `nums` between indices `i` and `j` inclusive, where `i <= j`.

## The Brute Force Trap

The naive solution iterates over all pairs `(i, j)` with `i <= j`, computes the subarray sum, and checks whether it falls within `[lower, upper]`. This O(N^2) approach (or O(N^3) if sums are recomputed from scratch each time) crumbles quickly when `N` reaches tens of thousands. We can optimize sum computation using prefix sums -- where `S(i, j) = prefix[j+1] - prefix[i]` -- but the pair enumeration itself remains quadratic. We need a way to count valid pairs without examining each one individually.

## The Strategy: Merge Sort over Prefix Sums

### Building the Foundation with Prefix Sums

I first construct a prefix sum array `sums` of length `N + 1`, where `sums[0] = 0` and `sums[i+1] = sums[i] + nums[i]`. With this array, any range sum `S(i, j)` equals `sums[j+1] - sums[i]`. The problem then transforms into: count the number of pairs `(i, j)` with `i < j` such that `lower <= sums[j] - sums[i] <= upper`. This is a structured counting problem over all pairs of prefix sums.

### Why Merge Sort Fits Perfectly

I recognized that merge sort provides exactly the framework needed. During the merge step, when I have two sorted halves, I can efficiently count how many elements in the right half satisfy the range constraint relative to each element in the left half. Specifically, for each `sums[i]` in the left half, I want to count how many `sums[j]` in the right half satisfy `lower <= sums[j] - sums[i] <= upper`, which is equivalent to `sums[i] + lower <= sums[j] <= sums[i] + upper`.

Because the right half is sorted, I can use two pointers `k` and `m` to find the window of valid values. For each `sums[i]` in the left half, `k` advances to the first position where `sums[k] - sums[i] >= lower`, and `m` advances to the first position where `sums[m] - sums[i] > upper`. The count of valid pairs for this `sums[i]` is `m - k`. Crucially, as `sums[i]` increases (since the left half is sorted), both `k` and `m` can only move forward, so the total counting work across all elements in the left half is linear.

### The Counting and Merging in Tandem

After counting, I perform the standard merge to combine the two halves into a single sorted sequence. This is essential: future recursion levels need the array sorted so the two-pointer counting technique remains valid. The counting and merging are separate phases within each merge step -- first I count the valid pairs, then I merge the elements.

### A Concrete Example

With `nums = [-2, 5, -1]`, `lower = -2`, `upper = 2`:

```
Prefix sums: [0, -2, 3, 2]

Split: [0, -2] and [3, 2]

Left sub-merge: [0, -2]
  Split: [0] and [-2]
  Count: for sums[i]=0, find sums[j] in [-2+0, 2+0] = [-2, 2]
         sums[j]=-2, which is in [-2, 2], count += 1
  Merge: [-2, 0]     running count = 1

Right sub-merge: [3, 2]
  Split: [3] and [2]
  Count: for sums[i]=3, find sums[j] in [3+(-2), 3+2] = [1, 5]
         sums[j]=2, which is not in [1, 5]... wait, 2 >= 1 and 2 <= 5, so count += 1
  Merge: [2, 3]      running count = 2

Final merge: [-2, 0] and [2, 3]
  For sums[i]=-2: find sums[j] in [-2+(-2), -2+2] = [-4, 0]
    Neither 2 nor 3 is in [-4, 0], count += 0
  For sums[i]=0: find sums[j] in [0+(-2), 0+2] = [-2, 2]
    sums[j]=2 is in [-2, 2], count += 1
  Merge: [-2, 0, 2, 3]    running count = 3
```

The answer is 3, corresponding to range sums: `S(0,0) = -2`, `S(2,2) = -1`, and `S(0,2) = 2`, all within `[-2, 2]`.

### Why the Recursion Does Not Double-Count

Each level of recursion counts only pairs `(i, j)` where `i` belongs to the left half and `j` belongs to the right half of the current subarray. Pairs where both indices are in the left half were counted at a deeper recursion level, and similarly for the right half. The disjoint partitioning guarantees that every valid pair is counted exactly once across the entire recursion tree.

## Rust Solution

```rust
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n = nums.len();
        let mut sums = vec![0i64; n + 1];
        for i in 0..n {
            sums[i + 1] = sums[i] + nums[i] as i64;
        }

        let mut cache = vec![0i64; n + 1];

        Self::merge_sort_recursive(&mut sums, &mut cache, 0, n + 1, lower as i64, upper as i64)
    }

    fn merge_sort_recursive(
        sums: &mut [i64],
        cache: &mut [i64],
        left: usize,
        right: usize,
        lower: i64,
        upper: i64,
    ) -> i32 {
        if right - left <= 1 {
            return 0;
        }

        let mid = left + (right - left) / 2;

        let mut count = Self::merge_sort_recursive(sums, cache, left, mid, lower, upper)
            + Self::merge_sort_recursive(sums, cache, mid, right, lower, upper);

        let mut k = mid;
        let mut m = mid;

        for i in left..mid {
            while k < right && sums[k] - sums[i] < lower {
                k += 1;
            }
            while m < right && sums[m] - sums[i] <= upper {
                m += 1;
            }
            count += (m - k) as i32;
        }

        let mut i = left;
        let mut j = mid;
        let mut idx = 0;

        while i < mid && j < right {
            if sums[i] < sums[j] {
                cache[left + idx] = sums[i];
                i += 1;
            } else {
                cache[left + idx] = sums[j];
                j += 1;
            }
            idx += 1;
        }

        while i < mid {
            cache[left + idx] = sums[i];
            i += 1;
            idx += 1;
        }
        while j < right {
            cache[left + idx] = sums[j];
            j += 1;
            idx += 1;
        }

        sums[left..right].copy_from_slice(&cache[left..right]);

        count
    }
}
```

The implementation starts by building a prefix sum array using `i64` to avoid overflow from accumulated `i32` values. A `cache` buffer of the same size is allocated once and reused across all recursion levels, keeping space usage at O(N). The recursive function `merge_sort_recursive` operates on the index range `[left, right)`. The base case returns 0 when the range contains at most one element. For each left-half element, two pointers `k` and `m` scan the right half to find the window where `lower <= sums[j] - sums[i] <= upper`. Because both halves are sorted and left-half elements are iterated in order, both pointers only advance forward, making the counting step O(N) per merge level. After counting, a standard merge combines the two halves into sorted order via the cache buffer, then `copy_from_slice` writes the result back into `sums`. The total work is O(N log N) across all recursion levels.

## Conclusion

Count of Range Sum is a beautiful example of how prefix sums and merge sort can combine to solve what initially looks like an intractable quadratic problem. By reformulating range sums as differences of prefix sums and leveraging the sorted order that merge sort provides, the two-pointer counting technique reduces the work at each merge level to linear time. The recursive decomposition ensures every pair is counted exactly once, and the careful use of `i64` arithmetic prevents overflow pitfalls that the `i32` input values might otherwise cause. The result is an elegant O(N log N) solution that handles the full constraint space with ease.
