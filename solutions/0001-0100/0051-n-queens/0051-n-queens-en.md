---
title: "0051 N-Queens - EN"
problemUrl: "https://leetcode.com/problems/n-queens/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["backtracking", "bitmask", "recursion"]
complexity:
  time: "O(N!)"
  space: "O(N)"
---

# Crowning Queens with Bits

## The Problem
Given an integer `n`, find all ways to place `n` queens on an `n x n` chessboard such that no two queens attack each other. Two queens attack each other if they share the same row, column, or diagonal. Return all valid configurations.

## Classic Backtracking and Its Achilles Heel

The natural approach to N-Queens is to place queens row by row, trying each column in the current row. Before placing a queen, we check that there's no conflict with previously placed queens. In a naive implementation, that means looping through all prior queens for each candidate, checking columns and both diagonals.

It works, but each check is linear in the number of queens already placed. What I wanted was a way to know in constant time whether a position is safe. And the answer, as in so many backtracking problems, lies in **bitmasks**.

## The Idea: Three Masks, Zero Validation Loops

The key is to represent constraints with three integers: `cols`, `ld` (left diagonal), and `rd` (right diagonal). Each set bit marks an attacked position.

- `cols` records which columns already have a queen.
- `ld` records the upper-left diagonals that are occupied. When we move down a row, left diagonal threats shift one bit to the left (`<< 1`).
- `rd` records the upper-right diagonals that are occupied. Moving down a row shifts them one bit to the right (`>> 1`).

With these three masks, the available positions in the current row are computed with a single operation:

```
possibilities = NOT(cols OR ld OR rd) AND limit
```

Where `limit = (1 << n) - 1` is a mask with the lower `n` bits set, ensuring we stay within the board. No loops, no scanning: one bitwise operation and I have exactly where the next queen can go.

## Extracting Positions with Bit Tricks

Once I have the possibilities mask, I need to iterate over each set bit to try each position. Here I use the classic trick of isolating the lowest set bit:

```
bit = possibilities & -possibilities   // Isolate the lowest set bit
col_idx = bit.trailing_zeros()          // Get the column index
possibilities ^= bit                   // Clear that bit for the next iteration
```

Each isolated `bit` represents a safe column. I push it onto the current state, make the recursive call with updated masks, and on return simply pop it off. The backtracking is clean and copy-free.

## Rust Solution

```rust
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut results = Vec::new();
        let limit = (1 << n) - 1;
        let mut current_board: Vec<usize> = Vec::with_capacity(n as usize);

        Self::backtrack(0, 0, 0, limit, n, &mut current_board, &mut results);

        results
    }

    fn backtrack(
        cols: i32,
        ld: i32,
        rd: i32,
        limit: i32,
        n: i32,
        current_board: &mut Vec<usize>,
        results: &mut Vec<Vec<String>>,
    ) {
        if cols == limit {
            results.push(Self::format_board(current_board, n));
            return;
        }

        let mut possibilities = !(cols | ld | rd) & limit;

        while possibilities > 0 {
            let bit = possibilities & -possibilities;

            let col_idx = bit.trailing_zeros() as usize;

            current_board.push(col_idx);

            Self::backtrack(
                cols | bit,
                (ld | bit) << 1,
                (rd | bit) >> 1,
                limit,
                n,
                current_board,
                results,
            );

            current_board.pop();

            possibilities ^= bit;
        }
    }

    fn format_board(indices: &Vec<usize>, n: i32) -> Vec<String> {
        let mut board = Vec::with_capacity(n as usize);
        for &col in indices {
            let mut row_str = String::with_capacity(n as usize);
            for i in 0..n {
                if i == col as i32 {
                    row_str.push('Q');
                } else {
                    row_str.push('.');
                }
            }
            board.push(row_str);
        }
        board
    }
}
```

What I like most about this implementation is the elegance of the base condition: `cols == limit`. When all columns are occupied, we know we've placed exactly `n` queens without conflicts. We don't need a separate row counter; the column mask tells us everything.

The diagonal constraint propagation is the most ingenious part. By computing `(ld | bit) << 1` and `(rd | bit) >> 1`, diagonal threats "fall" naturally to the next row. Each recursion level receives constraints already shifted, ready to use with no extra computation.

The `current_board` stores only the column index of each queen, which makes board reconstruction in `format_board` trivial: for each row, the queen is at the stored index and the rest are dots. The entire mutable state boils down to a vector of indices and three integers.

## Conclusion

The N-Queens problem is a classic that seems to call for pure backtracking, but combining it with bitmasks transforms the solution entirely. The three masks eliminate all conflict checking, bit shifting propagates diagonal constraints naturally, and the lowest-set-bit trick yields available positions one by one without scanning the board. The result is an algorithm that, while still exponential in the worst case, prunes the search space aggressively and executes each step in constant time.
