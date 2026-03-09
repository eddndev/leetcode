---
title: "0084 Largest Rectangle in Histogram - EN"
problemUrl: "https://leetcode.com/problems/largest-rectangle-in-histogram/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["stack", "array", "monotonic-stack"]
complexity:
  time: "O(N)"
  space: "O(N)"
---

# The Skyscraper Hidden in the Histogram

## The Problem
Given an array of integers `heights` representing the heights of bars in a histogram where each bar has width 1, find the area of the largest rectangle that can be formed within the histogram.

## The Initial Intuition

The first time I faced this problem, I thought of the obvious approach: for each bar, expand left and right while neighboring bars are at least as tall, and compute the area of the resulting rectangle. It works, but in the worst case each bar scans the entire array, giving O(N^2). For a histogram with 100,000 bars, that's not good enough.

I needed a way to know, for each bar, how far it can extend without repeating work. And that's where the **monotonic stack** comes in.

## The Monotonic Stack: Tear Down to Build Up

The core idea is to maintain a stack that stores bar indices in increasing order of height. We sweep the histogram left to right, and every time we encounter a bar shorter than the one on top of the stack, we know the top bar can no longer extend to the right: the current bar "cuts it off." That's the moment to compute its area.

When we pop an index from the stack, the rectangle's height is that bar's height. The width is determined as follows: the right boundary is the current position `i` (the bar that triggered the pop), and the left boundary is the new top of the stack (the previous bar that was shorter). If the stack becomes empty, it means the popped bar was the shortest seen so far, so the rectangle extends from the very beginning of the histogram.

### Why It Works

The stack maintains a powerful invariant: every bar in the stack knows that all bars between it and the bar above it are at least as tall. When a bar gets popped, we already have all the information needed to compute its maximum rectangle without ever having explicitly looked back. The stack acts as a compressed memory of each bar's "left boundary."

### A Step-by-step Example

For `heights = [2, 1, 5, 6, 2, 3]`:
- `i=0`: Stack empty, push 0. Stack: `[0]`
- `i=1`: `heights[1]=1 < heights[0]=2`. Pop 0: height=2, width=1 (stack empty, width=i=1), area=2. Push 1. Stack: `[1]`
- `i=2`: `heights[2]=5 >= heights[1]=1`. Push 2. Stack: `[1, 2]`
- `i=3`: `heights[3]=6 >= heights[2]=5`. Push 3. Stack: `[1, 2, 3]`
- `i=4`: `heights[4]=2 < heights[3]=6`. Pop 3: height=6, width=4-2-1=1, area=6. `heights[4]=2 < heights[2]=5`. Pop 2: height=5, width=4-1-1=2, area=10. `heights[4]=2 >= heights[1]=1`. Push 4. Stack: `[1, 4]`
- `i=5`: `heights[5]=3 >= heights[4]=2`. Push 5. Stack: `[1, 4, 5]`
- Cleanup phase (virtual height 0): Pop 5: height=3, width=6-4-1=1, area=3. Pop 4: height=2, width=6-1-1=4, area=8. Pop 1: height=1, width=6 (stack empty), area=6.
- Maximum: **10**

The final trick is crucial: after traversing the array, we introduce a virtual bar of height 0 to force everything remaining in the stack to be popped. This guarantees that no bar goes unevaluated.

## Rust Solution

```rust
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut stack: Vec<usize> = Vec::with_capacity(n);
        let mut max_area = 0;

        for i in 0..=n {
            let current_h = if i == n { 0 } else { heights[i] };

            while let Some(&top_index) = stack.last() {
                if current_h < heights[top_index] {
                    stack.pop();
                    let height = heights[top_index];
                    let width = if let Some(&prev_index) = stack.last() {
                        i - prev_index - 1
                    } else {
                        i
                    };

                    max_area = max_area.max(height * width as i32);
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        max_area
    }
}
```

The Rust implementation takes advantage of pattern matching expressiveness. The `while let Some(&top_index) = stack.last()` is idiomatic and elegant: it peeks at the top of the stack without popping, and only calls `pop` when it confirms the current bar is shorter. The range `0..=n` includes the extra iteration with the virtual bar of height 0, handled cleanly by `if i == n { 0 }`. The preallocation with `Vec::with_capacity(n)` is a subtle touch: in the worst case (a strictly increasing histogram), the stack will hold all indices before the cleanup phase. The width calculation using `i - prev_index - 1` when there's a previous element in the stack, or simply `i` when there isn't, captures exactly both cases of the logic without needing additional sentinels.

## Conclusion

This problem is a gem of monotonic stacks. The intuition that "when something can no longer grow, it's time to measure it" appears in many competitive programming problems, and mastering this pattern opens the door to an entire family of similar challenges. What makes this solution special is that each bar enters and leaves the stack exactly once, giving an amortized time of O(N) despite the nested loop. Sometimes the right data structure isn't the one that stores the most information, but the one that discards information at precisely the right moment.
