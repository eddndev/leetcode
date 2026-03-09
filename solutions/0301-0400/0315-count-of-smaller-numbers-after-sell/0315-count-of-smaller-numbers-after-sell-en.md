---
title: "0315 Count of Smaller Numbers After Self - EN"
problemUrl: "https://leetcode.com/problems/count-of-smaller-numbers-after-self/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["merge-sort", "divide-and-conquer", "inversion-count"]
complexity:
  time: "O(N log N) where N is the length of the array"
  space: "O(N) for the temporary merge buffer and index tracking"
---

# Counting Inversions in the Rearview Mirror

## The Problem
Given an integer array `nums`, return a new array `counts` where `counts[i]` is the number of elements to the right of `nums[i]` that are strictly smaller than `nums[i]`.

## The Brute Force Temptation

The straightforward approach checks, for each element, every element to its right and counts how many are smaller. This double loop runs in O(N^2) time, which is fine for small inputs but collapses under the constraint of up to 100,000 elements. We are essentially counting a specific kind of inversion: pairs `(i, j)` where `i < j` but `nums[i] > nums[j]`. The twist is that we need to attribute each inversion to the *left* element of the pair and record those counts per position.

## The Strategy: Merge Sort with Inversion Tracking

### The Core Insight

I recognized that merge sort naturally reveals inversions. When merging two sorted halves, every time an element from the right half is placed before an element from the left half, that right element is smaller than *all remaining elements in the left half*. But here I need the reverse perspective: for each left-half element, how many right-half elements were placed before it? That count is exactly the number of right-half elements already merged when the left-half element gets its turn.

The key observation is this: during the merge step, I maintain a counter `right_counter` that increments each time I pick an element from the right half. When I finally pick an element from the left half, `right_counter` tells me how many right-half elements are smaller than it. Since both halves are sorted and all right-half elements originally appeared to the right of all left-half elements in the current subarray, this count is precisely the number of smaller elements after self for that position.

### Why This Works Across Recursion Levels

At each level of recursion, the merge step only counts inversions between the left and right halves -- it does not recount inversions *within* each half, because those were already counted (and accumulated) in deeper recursive calls. The counts accumulate additively: each element's count grows as more distant elements are compared against it at higher levels of the recursion tree.

### Preserving Original Indices

There is a practical complication: merge sort rearranges elements, but I need to record counts at *original* positions. I solve this by pairing each value with its original index, creating tuples `(value, original_index)`. As elements move during sorting, their original indices travel with them. When I add to `right_counter` and attribute it to a left-half element, I write the count to `counts[original_index]`, not to the element's current position in the array.

### A Concrete Example

With `nums = [5, 2, 6, 1]`:

```
Initial: [(5,0), (2,1), (6,2), (1,3)]

Split: [(5,0), (2,1)] and [(6,2), (1,3)]

Left sub-merge: [(5,0), (2,1)]
  Split: [(5,0)] and [(2,1)]
  Merge: 2 < 5, pick (2,1), right_counter=1
         Left remaining: (5,0) gets right_counter=1 -> counts[0] += 1
  Result: [(2,1), (5,0)]      counts = [1, 0, 0, 0]

Right sub-merge: [(6,2), (1,3)]
  Split: [(6,2)] and [(1,3)]
  Merge: 1 < 6, pick (1,3), right_counter=1
         Left remaining: (6,2) gets right_counter=1 -> counts[2] += 1
  Result: [(1,3), (6,2)]      counts = [1, 0, 1, 0]

Final merge: [(2,1), (5,0)] and [(1,3), (6,2)]
  1 < 2, pick (1,3), right_counter=1
  2 < 6, pick (2,1), counts[1] += 1 -> counts = [1, 1, 1, 0]
  5 < 6, pick (5,0), counts[0] += 1 -> counts = [2, 1, 1, 0]
  pick (6,2), counts[2] += 1 -> counts = [2, 1, 2, 0]
  Wait -- that is wrong. Let me retrace.

Actually:
  i=0(left), j=0(right): arr[j]=(1,3) < arr[i]=(2,1) -> pick (1,3), right_counter=1
  arr[j]=(6,2) >= arr[i]=(2,1) -> pick (2,1), counts[1] += 1 -> counts = [1, 1, 1, 0]
  arr[j]=(6,2) >= arr[i]=(5,0) -> pick (5,0), counts[0] += 1 -> counts = [2, 1, 1, 0]
  Left exhausted, pick (6,2), right_counter stays 1, no left element benefits.

Result: [(1,3), (2,1), (5,0), (6,2)]     counts = [2, 1, 1, 0]
```

The final answer `[2, 1, 1, 0]` matches: to the right of 5 there are two smaller elements (2 and 1), to the right of 2 there is one (1), to the right of 6 there is one (1), and to the right of 1 there are none.

## Rust Solution

```rust
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return vec![];
        }

        let mut indexed_nums: Vec<(i32, usize)> = nums
            .into_iter()
            .enumerate()
            .map(|(i, val)| (val, i))
            .collect();

        let mut counts = vec![0; n];

        Self::merge_sort(&mut indexed_nums, &mut counts);

        counts
    }

    fn merge_sort(arr: &mut [(i32, usize)], counts: &mut [i32]) {
        let mid = arr.len() / 2;
        if mid == 0 {
            return;
        }

        Self::merge_sort(&mut arr[0..mid], counts);
        Self::merge_sort(&mut arr[mid..], counts);

        Self::merge(arr, mid, counts);
    }

    fn merge(arr: &mut [(i32, usize)], mid: usize, counts: &mut [i32]) {
        let mut temp = Vec::with_capacity(arr.len());

        let mut i = 0;
        let mut j = mid;
        let mut right_counter = 0;

        while i < mid && j < arr.len() {
            if arr[j].0 < arr[i].0 {
                temp.push(arr[j]);
                right_counter += 1;
                j += 1;
            } else {
                counts[arr[i].1] += right_counter;
                temp.push(arr[i]);
                i += 1;
            }
        }

        while i < mid {
            counts[arr[i].1] += right_counter;
            temp.push(arr[i]);
            i += 1;
        }

        while j < arr.len() {
            temp.push(arr[j]);
            j += 1;
        }

        arr.copy_from_slice(&temp);
    }
}
```

The implementation pairs each value with its original index as `(i32, usize)` tuples, then performs an in-place merge sort over the slice. The `merge` function allocates a temporary buffer sized to the current subarray and merges into it, then copies back with `copy_from_slice`. The `right_counter` variable tracks how many right-half elements have been placed so far; when a left-half element is finally placed, `right_counter` is added to `counts[original_index]`. After the main merge loop, any remaining left-half elements also receive the full `right_counter`, since all right-half elements (which are smaller) have already been placed. Remaining right-half elements need no count updates because they are larger than all left-half elements. The base case `mid == 0` catches single-element slices and empty slices, stopping the recursion. The use of Rust slices (`&mut arr[0..mid]`) avoids allocating new vectors at each recursion level, keeping the space overhead to O(N) total across all merge operations.

## Conclusion

Count of Smaller Numbers After Self is a classic inversion-counting problem dressed in slightly different clothes. By augmenting merge sort to carry original indices alongside values and tracking how many right-half elements cross over each left-half element during merging, we count all relevant inversions in O(N log N) time without ever resorting to nested loops. The divide-and-conquer structure ensures each pair is compared exactly once, and the additive accumulation of counts across recursion levels produces the correct per-position answer.
