---
title: "0295 Find Median from Data Stream - EN"
problemUrl: "https://leetcode.com/problems/find-median-from-data-stream/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["heap", "two-heaps", "design", "data-stream"]
complexity:
  time: "O(log N) per insertion, O(1) per median query"
  space: "O(N) where N is the total number of inserted elements"
---

# The Balancing Act between Two Mountains

## The Problem
Design a data structure that supports two operations on a stream of integers: adding a number to the stream (`addNum`) and finding the median of all elements seen so far (`findMedian`). The median is the middle value when the list is sorted; if the count of elements is even, it is the average of the two middle values.

## The Brute Force Trap

The most straightforward solution would be to maintain a sorted list and insert each new element in its correct position. With binary search, finding the correct position takes O(log N), but shifting elements to make room takes O(N). Querying the median would be O(1) -- just access the middle index -- but with thousands or millions of insertions, the accumulated O(N) cost per insertion becomes prohibitive.

Another idea would be to skip maintaining order entirely and sort at query time. But if `findMedian` is called frequently, we would be repeatedly sorting a growing list, which is even worse.

What we really need is a structure that gives us immediate access to the "center" of the stream without requiring all elements to be fully sorted.

## The Strategy: Two Heaps in Balance

### The Core Insight

I realized that finding the median does not require knowing the complete order of all elements. I only need to know two things: what is the largest of the lower half, and what is the smallest of the upper half. If I can keep those two values accessible in O(1), the median is computed trivially.

This leads me to use **two heaps**:
- `small`: a max-heap holding the lower half of the elements. Its top is the largest of the "small" ones.
- `large`: a min-heap holding the upper half of the elements. Its top is the smallest of the "large" ones.

### The Insertion Protocol

For each new number, I follow these steps:

1. **Insert into `small`**: I push the new number onto the max-heap. This ensures the number is considered as a lower-half candidate.

2. **Transfer the maximum of `small` to `large`**: I pop the top of `small` (the largest of the lower half) and push it onto `large`. This guarantees that every element in `large` is greater than or equal to every element in `small`.

3. **Rebalance if needed**: if `large` has more elements than `small`, I pop the minimum of `large` and move it back to `small`. This maintains the invariant that `small` has equal count or one more than `large`.

After these three operations, the invariant is restored: `small.len() >= large.len()` and `small.len() - large.len() <= 1`.

### Querying the Median

- If `small` has more elements than `large`, the median is simply the top of `small`.
- If both have the same count, the median is the average of the top of `small` and the top of `large`.

### A Concrete Example

Inserting the numbers `[6, 10, 2, 6, 5, 0]`:

```
addNum(6):  small=[6], large=[]          -> median = 6.0
addNum(10): small=[6], large=[10]        -> median = (6+10)/2 = 8.0
addNum(2):  small=[6,2], large=[10]      -> median = 6.0
addNum(6):  small=[6,2], large=[6,10]    -> median = (6+6)/2 = 6.0
addNum(5):  small=[6,5,2], large=[6,10]  -> median = 6.0
addNum(0):  small=[5,2,0], large=[6,6,10] -> median = (5+6)/2 = 5.5
```

At each step, the heaps rebalance automatically, and the median is always available by peeking at one or two tops.

### Why Each Operation Is Logarithmic

Each insertion involves at most three heap operations: a push into `small`, a pop from `small` followed by a push into `large`, and possibly a pop from `large` followed by a push into `small`. Each heap operation is O(log N), so the total insertion cost is O(log N). The median query only accesses the tops of the heaps, which is O(1).

## Rust Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    small: BinaryHeap<i32>,
    large: BinaryHeap<Reverse<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.small.push(num);

        if let Some(max_of_small) = self.small.pop() {
            self.large.push(Reverse(max_of_small));
        }


        if self.large.len() > self.small.len() {
            if let Some(Reverse(min_of_large)) = self.large.pop() {
                self.small.push(min_of_large);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.small.len() > self.large.len() {
            return *self.small.peek().unwrap() as f64;
        }

        let s = *self.small.peek().unwrap();
        let l = self.large.peek().unwrap().0;

        (s as f64 + l as f64) / 2.0
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
```

The Rust implementation leverages the standard library's `BinaryHeap`, which is a max-heap by default. To simulate a min-heap, I use the `Reverse` wrapper from `std::cmp`, which inverts the comparison order. The `small` field is a straightforward max-heap -- its `peek()` returns the largest element of the lower half. The `large` field uses `BinaryHeap<Reverse<i32>>`, turning the native max-heap into an effective min-heap whose `peek()` returns the smallest of the upper half. The insertion protocol always routes the element through `small` first, then transfers the maximum to `large`, and finally rebalances if `large` has grown too big. The `unwrap()` calls in `find_median` are safe because this function is only called after at least one insertion, guaranteeing that `small` is never empty.

## Conclusion

Find Median from Data Stream is a classic data structure design problem. The idea of splitting elements into two halves using a pair of complementary heaps -- a max-heap for the lower half and a min-heap for the upper half -- transforms what seems to require a fully sorted list into an elegant O(log N) insertion and O(1) query operation. The heaps act as two opposing mountains whose peaks always point toward the center of the stream, and keeping them in balance is all it takes for the median to always be within arm's reach.
