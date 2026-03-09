---
title: "0239 Sliding Window Maximum - EN"
problemUrl: "https://leetcode.com/problems/sliding-window-maximum/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["deque", "sliding-window", "monotonic-queue"]
complexity:
  time: "O(N) where N is the length of the array"
  space: "O(K) where K is the window size"
---

# The Sentinel at the Front of the Line

## The Problem
Given an array of integers `nums` and a sliding window of size `k` that moves from the very left of the array to the very right, return an array containing the maximum value in each window position. You can only see the `k` numbers in the window, and the window moves one position to the right each time.

## The Naive Trap

The obvious approach -- scanning all `k` elements in each window position to find the maximum -- works but runs in O(N*K) time. For large arrays with large windows, this is too slow. Every time the window slides by one position, we throw away most of our previous work. We already knew the maximum among `k-1` of those elements; surely we can reuse that knowledge somehow.

But there is a subtle complication. If the maximum was the element that just left the window, we have no cheap way to know what the *next* maximum is. A regular max-tracking variable cannot handle eviction. We need a data structure that maintains order, supports efficient removal from both ends, and always gives us the current maximum in O(1).

## The Strategy: A Monotonic Deque

### The Core Insight

I realized that not every element in the window needs to be remembered. If I am looking at element `nums[i]` and there is an earlier element `nums[j]` (where `j < i`) that is *smaller* than `nums[i]`, then `nums[j]` can never be the maximum of any future window. Why? Because `nums[i]` entered the window later and is larger -- it will outlive `nums[j]` in the window and always beat it. So `nums[j]` is useless and can be discarded.

This observation leads to a **monotonic decreasing deque**: a double-ended queue where elements are always in decreasing order from front to back. The front of the deque is always the current window's maximum.

### Building the Deque

For each new element `nums[i]`, I perform three operations:

1. **Evict the expired**: if the element at the front of the deque has an index that is outside the current window (i.e., its index is `<= i - k`), I pop it from the front. It has aged out.

2. **Enforce monotonicity**: I pop elements from the back of the deque as long as they are less than or equal to `nums[i]`. These elements are now dominated by the newcomer and will never be a window maximum.

3. **Insert the newcomer**: I push `i` onto the back of the deque.

After the first `k-1` elements have been processed (i.e., when `i >= k - 1`), the front of the deque holds the index of the current window's maximum, which I record in the result.

### A Concrete Example

With `nums = [1, 3, -1, -3, 5, 3, 6, 7]` and `k = 3`:

```
i=0: nums[0]=1.  Deque: [0].          Window not full yet.
i=1: nums[1]=3.  Pop 0 (1<=3). Deque: [1].  Window not full yet.
i=2: nums[2]=-1. Deque: [1, 2].       Window [1,3,-1] -> max=nums[1]=3
i=3: nums[3]=-3. Deque: [1, 2, 3].    Window [3,-1,-3] -> max=nums[1]=3
i=4: nums[4]=5.  Pop 3,2,1. Deque: [4]. Window [-1,-3,5] -> max=nums[4]=5
i=5: nums[5]=3.  Deque: [4, 5].       Window [-3,5,3] -> max=nums[4]=5
i=6: nums[6]=6.  Pop 5. Deque: [4, 6]. Front 4 expired (4<=6-3). Pop 4. Deque: [6].
     Window [5,3,6] -> max=nums[6]=6
i=7: nums[7]=7.  Pop 6. Deque: [7].   Window [3,6,7] -> max=nums[7]=7

Result: [3, 3, 5, 5, 6, 7]
```

### Why Each Element is Touched at Most Twice

Every index enters the deque exactly once (pushed to the back) and leaves the deque at most once (popped from either end). So across all `N` iterations, the total number of deque operations is at most `2N`. This makes the overall algorithm O(N), regardless of the window size `k`.

## Rust Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut deque: VecDeque<usize> = VecDeque::with_capacity(k);
        let mut result = Vec::with_capacity(n - k + 1);

        for i in 0..n {
            if let Some(&front) = deque.front() {
                if i >= k && front <= i - k {
                    deque.pop_front();
                }
            }

            while let Some(&back) = deque.back() {
                if nums[back] <= nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(i);

            if i >= k - 1 {
                result.push(nums[*deque.front().unwrap()]);
            }
        }

        result
    }
}
```

The implementation stores indices rather than values in the deque, which serves a dual purpose: indices let me check whether the front element has expired (fallen outside the window), and I can always recover the value via `nums[index]`. The `VecDeque` is pre-allocated with capacity `k` since the deque never holds more than `k` elements. The result vector is pre-allocated with `n - k + 1` capacity -- the exact number of windows. The expiration check `front <= i - k` only fires when `i >= k`, preventing underflow on the unsigned subtraction. The monotonicity enforcement loop uses `<=` rather than `<`, meaning equal elements are also evicted; this is safe because the newer element at the same value will outlive the older one and produce the same maximum.

## Conclusion

The Sliding Window Maximum is a textbook application of the monotonic deque pattern. By maintaining a decreasing sequence of candidates, we ensure that the maximum is always at the front, expiration is a simple index comparison, and every element is processed in amortized O(1) time. The result is a linear-time algorithm that elegantly avoids redundant comparisons, turning what looks like an O(N*K) problem into an O(N) one with just a deque and a single pass through the array.
