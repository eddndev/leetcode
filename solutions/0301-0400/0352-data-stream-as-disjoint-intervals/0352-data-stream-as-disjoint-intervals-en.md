---
title: "0352 Data Stream as Disjoint Intervals - EN"
problemUrl: "https://leetcode.com/problems/data-stream-as-disjoint-intervals/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["binary-search-tree", "ordered-map", "design", "intervals"]
complexity:
  time: "O(log N) per addNum, O(N) per getIntervals, where N is the number of intervals"
  space: "O(N)"
---

# Weaving Intervals from Chaos

## The Problem
Design a data structure that receives a stream of non-negative integers and summarizes them as a list of disjoint intervals. Implement the `SummaryRanges` class with two operations: `addNum(value)` which adds the integer `value` to the stream, and `getIntervals()` which returns a summary of the integers in the stream as a list of disjoint intervals `[start, end]`, sorted by `start`.

## The Challenge of a Continuous Stream

At first glance, one might consider maintaining a set of all seen numbers and rebuilding the intervals every time `getIntervals` is called. But that wastes work: if we already have the intervals formed, adding a new number should only affect things locally, perhaps extending an existing interval, merging two neighboring intervals, or creating a new one. The question is how to perform those operations efficiently.

## The BTreeMap Intuition

The ideal structure for this problem is an ordered map where each key is the start of an interval and its value is the end. A `BTreeMap` in Rust gives us exactly that: lookups, insertions, and deletions in O(log N), along with the ability to search for the predecessor and successor of any key.

When a new value arrives, I need to answer three questions:

1. **Is it already contained?** If some existing interval already covers this value, there is nothing to do.
2. **Is it adjacent to the previous interval?** If an interval ending at exactly `value - 1` exists, I can extend it.
3. **Is it adjacent to the next interval?** If there is an interval starting at `value + 1`, I can absorb it.

If it is adjacent to both sides, I merge the two intervals into one. If it only touches one side, I extend that interval. If it touches neither, I create a new interval `[value, value]`.

## Walking Through an Example

Suppose we add the numbers `1, 3, 7, 2, 6`:

- **addNum(1)**: No intervals exist. Create `[1, 1]`. State: `{[1,1]}`.
- **addNum(3)**: Not adjacent to `[1,1]` (which ends at 1, and 1 != 3-1=2). Create `[3, 3]`. State: `{[1,1], [3,3]}`.
- **addNum(7)**: Not adjacent to anything. Create `[7, 7]`. State: `{[1,1], [3,3], [7,7]}`.
- **addNum(2)**: The predecessor is `[1,1]` ending at 1 = 2-1, so `merge_left = true`. The successor is `[3,3]` starting at 3 = 2+1, so `merge_right = true`. Merge both: remove `[3,3]` and update `[1,1]` to `[1,3]`. State: `{[1,3], [7,7]}`.
- **addNum(6)**: The predecessor is `[1,3]` ending at 3, and 3 != 6-1=5, so `merge_left = false`. The successor is `[7,7]` starting at 7 = 6+1, so `merge_right = true`. Remove `[7,7]` and insert `[6,7]`. State: `{[1,3], [6,7]}`.

Result of `getIntervals()`: `[[1,3], [6,7]]`.

## Rust Solution

```rust
use std::collections::BTreeMap;

struct SummaryRanges {
    intervals: BTreeMap<i32, i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        SummaryRanges {
            intervals: BTreeMap::new(),
        }
    }

    fn add_num(&mut self, value: i32) {
        let prev = self
            .intervals
            .range(..=value)
            .next_back()
            .map(|(&start, &end)| (start, end));

        if let Some((_, end)) = prev {
            if end >= value {
                return;
            }
        }

        let merge_left = prev.map_or(false, |(_, end)| end == value - 1);

        let next = self
            .intervals
            .range((value + 1)..)
            .next()
            .map(|(&start, &end)| (start, end));
        let merge_right = next.map_or(false, |(start, _)| start == value + 1);

        match (merge_left, merge_right) {
            (true, true) => {
                let (prev_start, _) = prev.unwrap();
                let (next_start, next_end) = next.unwrap();

                self.intervals.remove(&next_start);
                self.intervals.insert(prev_start, next_end);
            }
            (true, false) => {
                let (prev_start, _) = prev.unwrap();
                self.intervals.insert(prev_start, value);
            }
            (false, true) => {
                let (next_start, next_end) = next.unwrap();
                self.intervals.remove(&next_start);
                self.intervals.insert(value, next_end);
            }
            (false, false) => {
                self.intervals.insert(value, value);
            }
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals
            .iter()
            .map(|(&start, &end)| vec![start, end])
            .collect()
    }
}
```

The implementation uses a `BTreeMap<i32, i32>` where each entry `(start, end)` represents the interval `[start, end]`. When a new value arrives, `range(..=value).next_back()` finds the interval with the largest key not exceeding `value`, i.e., the predecessor candidate. If that interval already contains `value` (its `end >= value`), we return immediately. Then we determine whether we can merge left (the predecessor ends at `value - 1`) and whether we can merge right (the successor starts at `value + 1`). The `match` on the four possible combinations of `(merge_left, merge_right)` handles each case: double merge, left extension, right extension, or creation of a new interval. The `get_intervals` operation simply iterates the ordered map, which already maintains the intervals in the correct order.

## Conclusion

Data Stream as Disjoint Intervals is a design problem that rewards choosing the right data structure. The `BTreeMap` gives us the perfect balance: logarithmic lookups to locate neighboring intervals, and a natural ordering that makes querying the intervals a simple traversal. The four-case merge logic is the heart of the algorithm, and once you understand that each new number can only affect its two immediate neighbors in the map, the solution flows with elegance. It is a reminder that in streaming data problems, maintaining the structure incrementally is almost always better than rebuilding it from scratch.
