---
title: "0154 Find Minimum in Rotated Sorted Array II - EN"
problemUrl: "https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["array", "binary-search"]
complexity:
  time: "O(N) worst case, O(log N) average"
  space: "O(1)"
---

# Hunting the Needle in a Twisted Haystack

## The Problem
Given a sorted array of integers `nums` that has been rotated between 1 and `n` times, and which **may contain duplicates**, find the minimum element. The array was originally sorted in ascending order and then rotated at some unknown pivot.

## The First Impression

The classic version of this problem -- without duplicates -- is a textbook binary search exercise. You compare the middle element to the rightmost, and the side where the minimum lives reveals itself cleanly. But throw duplicates into the mix and suddenly that neat binary partition crumbles. When `nums[mid] == nums[right]`, you genuinely cannot tell which half contains the minimum. That ambiguity is the entire soul of this problem.

My initial thought was: can I still salvage binary search? The answer is yes, but with a caveat. In the worst case -- think an array like `[1, 1, 1, 1, 1, 1, 1]` rotated to `[1, 1, 1, 1, 1, 1, 1]` -- no algorithm can do better than linear time, because every element looks identical and you have no information to discard any portion of the array. The best we can do is degrade gracefully: use binary search when the duplicates let us, and shrink the window by one when they don't.

## The Three-Way Decision

The algorithm maintains two pointers, `left` and `right`, and repeatedly examines the middle element. There are exactly three cases:

### Case 1: `nums[mid] > nums[right]`

The minimum must be somewhere to the right of `mid`. If the middle is larger than the rightmost element, the rotation point -- where the array wraps from its maximum back to its minimum -- lies in the interval `(mid, right]`. We can safely move `left = mid + 1`.

### Case 2: `nums[mid] < nums[right]`

The minimum is at `mid` or to its left. The subarray `[mid, right]` is properly sorted, so the smallest value in that range is `nums[mid]` itself. We set `right = mid`, keeping `mid` as a candidate.

### Case 3: `nums[mid] == nums[right]`

This is the tricky case. We cannot determine which side the minimum is on. Consider `[3, 1, 3, 3, 3]` versus `[3, 3, 3, 1, 3]` -- both have `nums[mid] == nums[right] == 3`, but the minimum is on different sides. The only safe move is to shrink the search space by one: `right -= 1`. We lose at most one element that equals something we've already seen at `mid`, so we won't accidentally skip the minimum.

### Termination

The loop runs while `left < right`. When they meet, `nums[left]` is the answer.

## Rust Solution

```rust
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] > nums[right] {
                left = mid + 1;
            } else if nums[mid] < nums[right] {
                right = mid;
            } else {
                right -= 1;
            }
        }

        nums[left]
    }
}
```

The implementation is strikingly compact. The midpoint calculation `left + (right - left) / 2` avoids integer overflow -- a habit worth building even in Rust where `usize` is large. The three branches map directly to the three logical cases described above. Notice that we never use `left = mid` (which would risk infinite loops with integer division rounding down) -- the `left` pointer always advances by at least one, and `right` always decreases by at least one, guaranteeing convergence. Since `nums` is a `Vec<i32>`, indexing is checked in debug mode, and the final `nums[left]` is safe because `left` stays within bounds throughout.

## Conclusion

This problem is a masterclass in understanding the limits of binary search. With unique elements, binary search gives you a clean `O(log N)` guarantee. Duplicates erode that guarantee by introducing ambiguity at every step where values collide. The `right -= 1` fallback is elegant in its simplicity -- it's the minimal concession to uncertainty, preserving the binary search structure for every step where the algorithm *can* make a decisive split. The worst case is `O(N)`, but on most inputs with moderate duplicates, the algorithm still runs in logarithmic time. It's a reminder that sometimes the best algorithm isn't uniformly fast -- it's one that's fast when it can be and gracefully linear when it must be.
