---
title: "0085 Maximal Rectangle - EN"
problemUrl: "https://leetcode.com/problems/maximal-rectangle/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["stack", "array", "dynamic-programming", "matrix", "monotonic-stack"]
complexity:
  time: "O(R * C)"
  space: "O(C)"
---

# The Rectangle Hiding in Plain Sight

## The Problem
Given a `rows x cols` binary matrix filled with `'0'`s and `'1'`s, find the area of the largest rectangle containing only `'1'`s.

## The Key Insight: Flattening Two Dimensions into One

When I first saw this problem, I tried to think in purely two-dimensional terms: scanning every possible top-left and bottom-right pair, checking if the sub-matrix was all ones. That's O(R^2 * C^2) at best, and it felt like brute force wearing a disguise.

The breakthrough came when I stopped seeing the matrix as a grid and started seeing it as a **stack of histograms**. If I fix a row as my "ground level," then each column's height is the number of consecutive `'1'`s reaching up from that row. Row by row, I'm building a new histogram, and the question becomes: what's the largest rectangle in this histogram?

And that's a problem I already know how to solve in O(C) with a monotonic stack.

## Building the Histograms Row by Row

Consider this matrix:
```
1 0 1 0 0
1 0 1 1 1
1 1 1 1 1
1 0 0 1 0
```

After processing each row, the heights array looks like:
- Row 0: `[1, 0, 1, 0, 0]`
- Row 1: `[2, 0, 2, 1, 1]`
- Row 2: `[3, 1, 3, 2, 2]`
- Row 3: `[4, 0, 0, 3, 0]`

The rule is simple: if the current cell is `'1'`, increment the height from the previous row; if it's `'0'`, reset to zero. A `'0'` breaks the continuity -- no rectangle can pass through it vertically.

At row 2, the heights `[3, 1, 3, 2, 2]` encode a histogram where the largest rectangle has area 6 (a 3-wide, 2-tall rectangle spanning columns 2 through 4). That turns out to be the answer for the entire matrix.

## The Monotonic Stack: Measuring Before Demolishing

For each row's histogram, I sweep left to right, maintaining a stack of column indices in increasing order of height. When I encounter a column shorter than the top of the stack, the top bar can no longer extend rightward. I pop it and compute its rectangle: the height is the popped bar's value, and the width stretches from the new stack top (left boundary) to the current position (right boundary).

After processing all columns, I introduce a virtual column of height 0 to flush everything remaining in the stack. This ensures no bar escapes without being measured.

The beauty is that each column index is pushed and popped at most once per row, so the inner loop is amortized O(C) per row, giving O(R * C) total.

## Why the Space Is Only O(C)

I never store the full matrix of heights. A single array of length C is updated in place as I move from one row to the next. The stack also holds at most C elements. So the total extra space is O(C), independent of the number of rows.

## Rust Solution

```rust
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut heights = vec![0; cols];
        let mut max_area = 0;

        let mut stack = Vec::with_capacity(cols + 1);

        for row in matrix {
            for (i, &val) in row.iter().enumerate() {
                if val == '1' {
                    heights[i] += 1;
                } else {
                    heights[i] = 0;
                }
            }

            stack.clear();

            for i in 0..=cols {
                let current_h = if i == cols { 0 } else { heights[i] };

                while let Some(&top) = stack.last() {
                    if current_h < heights[top] {
                        stack.pop();
                        let h = heights[top];

                        let w = if let Some(&prev) = stack.last() {
                            i - prev - 1
                        } else {
                            i
                        };

                        max_area = max_area.max(h * w as i32);
                    } else {
                        break;
                    }
                }
                stack.push(i);
            }
        }

        max_area
    }
}
```

The Rust implementation is remarkably compact for how much it does. The outer loop iterates over each row of the matrix, updating the `heights` array in place: a `'1'` extends the bar upward, a `'0'` resets it to the ground. The stack is allocated once with `Vec::with_capacity(cols + 1)` and cleared between rows rather than reallocated, avoiding unnecessary heap churn. The sentinel trick -- ranging over `0..=cols` and treating `i == cols` as a virtual bar of height 0 -- elegantly forces the stack to drain without special-casing the end of the row. The width calculation with `i - prev - 1` when a previous index exists, or simply `i` when the stack is empty, captures both boundary cases without needing explicit sentinel values in the heights array. The cast `w as i32` is safe because the problem constraints guarantee that dimensions fit comfortably within 32-bit integers.

## Conclusion

This problem is a masterclass in reduction: a seemingly complex 2D problem collapses into a series of 1D problems, each solvable by a well-known technique. The histogram interpretation transforms the matrix into something a monotonic stack can consume row by row, and the result is an algorithm that is both optimal in time and minimal in space. The lesson here goes beyond this specific problem -- whenever you face a grid and feel overwhelmed by the dimensionality, ask yourself: can I fix one dimension and solve a simpler problem along the other? More often than not, the answer is yes, and the resulting solution is far more elegant than anything that tries to wrestle with both dimensions at once.
