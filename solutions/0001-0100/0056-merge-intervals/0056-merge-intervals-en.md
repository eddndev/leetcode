---
title: "0056 Merge Intervals - EN"
problemUrl: "https://leetcode.com/problems/merge-intervals/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["array", "sorting"]
complexity:
  time: "O(n log n)"
  space: "O(n)"
---

# Merge Intervals: Sort First, Ask Questions Later

## The Problem
Given an array of `intervals` where `intervals[i] = [start_i, end_i]`, merge all overlapping intervals and return an array of the non-overlapping intervals that cover all the intervals in the input.

At first glance it seems like a graph problem where you need to find connected components. But the trick is that sorting transforms it into something much simpler.

## The Intuition: Sorting Changes Everything

If the intervals are unsorted, detecting overlaps requires comparing every pair, which is $O(n^2)$. But once we sort them by their start value, overlapping intervals become adjacent. That single observation reduces the problem to a linear scan.

After sorting, we iterate through the intervals and maintain a running "current" interval. For each new interval, we check: does it overlap with the current one? If it does (the current interval's end is greater than or equal to the new interval's start), we extend the current interval's end to cover both. If it doesn't, we push the current interval into our result and start a new one.

The key detail is that when extending, we take the **maximum** of both ends, not just the new interval's end. This handles the case where one interval is completely contained within another, like `[1, 10]` and `[2, 5]`.

## Rust Solution

Rust's `sort_unstable_by` is perfect here since we don't need to preserve the relative order of equal elements, and it avoids the overhead of a stable sort. The `match` on `result.last_mut()` is idiomatic: it gives us a mutable reference to the last element without any bounds checking overhead in the happy path.

```rust
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut result: Vec<Vec<i32>> = Vec::new();

        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        for interval in intervals {
            match result.last_mut() {
                Some(last) if last[1] >= interval[0] => {
                    last[1] = last[1].max(interval[1]);
                }

                _ => {
                    result.push(interval);
                }
            }
        }

        result
    }
}
```

## Conclusion

The bottleneck is the sort at $O(n \log n)$; the merge pass itself is just $O(n)$. Space is $O(n)$ for the output. This is one of those problems where the right preprocessing step (sorting) makes the actual logic almost trivial. The pattern of "sort, then linear scan with a running state" comes up surprisingly often in interval problems.
