---
title: "0052 N-Queens II - EN"
problemUrl: "https://leetcode.com/problems/n-queens-ii/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["backtracking", "bitmask", "recursion"]
complexity:
  time: "O(N!)"
  space: "O(1)"
---

# Counting Crowns Without Building Thrones

## The Problem
Given an integer `n`, return the number of distinct ways to place `n` queens on an `n x n` chessboard so that no two queens attack each other. Unlike its sibling N-Queens I, we don't need to return the board configurations -- just the count.

## From Boards to Pure Arithmetic

In N-Queens I, the main work goes into constructing and storing every valid board layout. But when the question is simply "how many?", all of that bookkeeping vanishes. No board vector, no formatting function, no result list. The entire problem collapses into a single recursive counter driven by three bitmasks. What remains is the purest form of the algorithm: constraint propagation through bits and nothing else.

## Three Masks, One Counter

The approach is identical to N-Queens I in spirit, but stripped of everything unnecessary. Three integers encode all the information we need:

- `cols` tracks which columns are already occupied by a queen.
- `diags` tracks the upper-left-to-lower-right diagonals under attack. When we descend one row, these threats shift left by one bit (`<< 1`).
- `anti_diags` tracks the upper-right-to-lower-left diagonals under attack. These shift right by one bit (`>> 1`) as we move down.

The available positions in the current row are computed in a single expression:

```
available = ((1 << n) - 1) & NOT(cols OR diags OR anti_diags)
```

The mask `(1 << n) - 1` keeps us within the board's bounds. The `NOT` of the combined constraints gives us exactly the safe squares. No loops, no array lookups -- one bitwise operation and I know every valid placement for the row.

## Isolating Positions with the Lowest-Bit Trick

To iterate over the available positions, I use the classic lowest-set-bit extraction:

```
position = available & -available      // Isolate the lowest set bit
available = available & (available - 1) // Clear that bit
```

Each `position` represents a safe column. I recurse with updated masks and, on return, the next iteration of the while loop picks the next available position. There's no explicit undo step because the masks are passed by value -- each recursive call gets its own copy of the constraints. The backtracking is implicit and allocation-free.

## The Base Case: Row Equals N

When `row == n`, we've placed a queen on every row without conflict. That means one more valid configuration found, so we return 1. The return values bubble up through the recursion, accumulating the total count without ever materializing a board.

## Rust Solution

```rust
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        Self::solve(n, 0, 0, 0, 0)
    }

    fn solve(n: i32, row: i32, cols: i32, diags: i32, anti_diags: i32) -> i32 {
        if row == n {
            return 1;
        }

        let mut count = 0;

        let mut available = ((1 << n) - 1) & !(cols | diags | anti_diags);

        while available != 0 {
            let position = available & -available;

            available = available & (available - 1);

            count += Self::solve(
                n,
                row + 1,
                cols | position,
                (diags | position) << 1,
                (anti_diags | position) >> 1,
            );
        }

        count
    }
}
```

What I find most satisfying about this solution is how little state it carries. There is no mutable structure passed through the recursion -- no vectors, no arrays, no board representation at all. The five parameters to `solve` are plain integers, passed by value. Each recursive call receives its own snapshot of constraints, so backtracking happens automatically when the call returns. The entire algorithm lives in arithmetic.

The diagonal shift trick deserves attention: `(diags | position) << 1` and `(anti_diags | position) >> 1` propagate the diagonal threats downward in a single operation per direction. By the time the next row's `solve` call runs, the masks already reflect where every previously placed queen attacks. No post-processing, no separate constraint arrays -- just shifted integers.

The space complexity drops to O(1) auxiliary space (beyond the recursion stack of depth `n`) because we store nothing except the integer parameters on each frame. Compare that to N-Queens I, which needs O(N^2) space for the board configurations alone.

## Conclusion

N-Queens II is a beautiful example of how removing output requirements can simplify a solution dramatically. The same bitmask backtracking from N-Queens I, stripped of board construction and storage, becomes a pure counting recursion with zero allocations. Three masks propagate constraints, the lowest-set-bit trick iterates over candidates, and value-passing semantics handle backtracking for free. The result is an algorithm that counts all valid configurations using nothing but integer arithmetic and the call stack.
