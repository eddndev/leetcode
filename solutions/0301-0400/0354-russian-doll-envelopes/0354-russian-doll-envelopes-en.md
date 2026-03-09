---
title: "0354 Russian Doll Envelopes - EN"
problemUrl: "https://leetcode.com/problems/russian-doll-envelopes/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["dynamic-programming", "binary-search", "sorting", "greedy"]
complexity:
  time: "O(N log N) where N is the number of envelopes"
  space: "O(N)"
---

# Paper Nesting Dolls

## The Problem
Given a 2D array of integers `envelopes` where `envelopes[i] = [wi, hi]` represents the width and height of an envelope, return the maximum number of envelopes you can Russian doll (i.e., put one inside the other). An envelope can fit into another if and only if both its width and height are strictly less than the other envelope's width and height. You cannot rotate an envelope.

## The Initial Intuition

This problem looks like multidimensional nesting, but it has an elegant reduction to a classic one-dimensional problem. If I could somehow fix one dimension, the problem would reduce to finding the Longest Increasing Subsequence (LIS) on the other dimension. The key is finding a sorting order that makes this valid.

If I sort envelopes by width in ascending order, then I only need to worry about heights: any increasing subsequence in heights automatically satisfies the width constraint, because the envelopes are already in width order. But there is a catch -- envelopes with the same width cannot nest inside each other, and a naive LIS on heights might select multiple envelopes sharing the same width.

## The Sorting Trick

The fix for the same-width trap is subtle but powerful. When two envelopes share the same width, I sort their heights in descending order. Why does this work? Consider envelopes with width 5 and heights 3, 5, 7. If I order them as `[5,7], [5,5], [5,3]`, then the LIS algorithm on heights will never pick more than one of these, because the heights are decreasing -- you cannot build an increasing subsequence from a decreasing sequence. If they were in ascending order `[5,3], [5,5], [5,7]`, the LIS might take all three, violating the constraint that width must be strictly less.

So the sorting rule is: width ascending as the primary key, height descending as the tiebreaker. This transforms the 2D problem into a pure LIS on heights.

## From Quadratic LIS to Binary Search LIS

The classic O(N^2) LIS algorithm would compare every pair of elements. For input sizes up to 10^5 envelopes, this would be too slow. I need the binary search LIS algorithm that runs in O(N log N).

The idea is to maintain a `tails` array where `tails[i]` is the smallest possible tail value among all increasing subsequences of length `i + 1` found so far. This array is always sorted, which enables binary search. For each new height, I search for where it should be inserted. If it is larger than every element in `tails`, it extends the longest subsequence. Otherwise, it replaces the element at the found position, preserving the possibility of future extensions with lower values.

## Why the Tails Array Works

The `tails` array does not store an actual valid subsequence -- it stores the best possible tail for each length. When I replace `tails[idx]` with a smaller value, I am not breaking any existing subsequence. I am recording that there now exists a subsequence of length `idx + 1` ending at a smaller value, which can only be equal or better for future extensions.

The length of `tails` at the end of the process is exactly the length of the LIS, which is the answer to our original problem.

## Rust Solution

```rust
impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut tails: Vec<i32> = Vec::new();

        for env in envelopes {
            let height = env[1];

            match tails.binary_search(&height) {
                Ok(_) => {}
                Err(idx) => {
                    if idx == tails.len() {
                        tails.push(height);
                    } else {
                        tails[idx] = height;
                    }
                }
            }
        }

        tails.len() as i32
    }
}
```

The implementation starts with an unstable sort for performance, using a custom comparator that sorts by width ascending and by height descending when widths are equal. It then iterates over each envelope, extracting only the height since width is handled by the sort order. Rust's `binary_search` call returns `Ok(pos)` if the value already exists in `tails` -- in that case we do nothing, because a duplicate neither extends nor improves any subsequence. If it returns `Err(idx)`, we get the insertion point: if `idx` equals the length of `tails`, the value is larger than all existing elements and extends the longest subsequence; otherwise, it replaces `tails[idx]` to maintain the optimal tail for that length.

## Conclusion

Russian Doll Envelopes is an elegant example of dimensionality reduction. What looks like a two-dimensional nesting problem transforms, through clever sorting, into a classic one-dimensional LIS. The trick of sorting by height descending within the same width neutralizes the possibility of selecting invalid envelopes. Binary search over the tails array brings the complexity from O(N^2) down to O(N log N), making the solution efficient even for large inputs. The Rust implementation is remarkably concise: a sort, a loop, and a binary search are all it takes to solve a Hard-difficulty problem.
