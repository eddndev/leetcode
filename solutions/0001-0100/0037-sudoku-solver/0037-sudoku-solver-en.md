---
title: "0037 Sudoku Solver - EN"
problemUrl: "https://leetcode.com/problems/sudoku-solver/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["backtracking", "bitmask", "constraint-propagation"]
complexity:
  time: "O(9^M) where M is the number of empty cells"
  space: "O(M)"
---

# Solving Sudoku with Bits and Backtracking

## The Problem
Given a partially filled 9x9 Sudoku board, fill it so that each row, each column, and each 3x3 sub-box contains the digits 1 through 9 exactly once. The puzzle is guaranteed to have exactly one solution.

## The Temptation of Brute Force

When you face a Sudoku programmatically, the first impulse is simple: find an empty cell, try numbers 1 through 9, check if it's valid, and if it doesn't work, backtrack. Classic backtracking. And it works, but it's painfully slow. Each validity check scans rows, columns, and boxes, and the search tree grows relentlessly.

What I needed was a way to instantly know which numbers are valid for a given cell, without scanning anything. And that's where **bitmasks** come into play.

## The Idea: Constraints as Bits

Imagine that each row, column, and 3x3 box has a 9-bit "mask." Each bit represents whether a digit (1 through 9) is already present. If bit 0 is set, 1 has been used. If bit 4 is set, 5 has been used.

With three arrays of masks (`rows`, `cols`, `boxes`), I can compute the valid candidates for any cell `(r, c)` with a single operation:

```
candidates = NOT(rows[r] OR cols[c] OR boxes[b]) AND 0x1FF
```

The `OR` combines all constraints, `NOT` inverts them to get available numbers, and `AND 0x1FF` ensures we only keep the lower 9 bits. In one line, no loops, I have exactly which digits can be placed.

## The Heuristic: MRV (Minimum Remaining Values)

Plain backtracking picks the first empty cell it finds. But not all empty cells are equal. If a cell has only one possible candidate, we should fill it first: there's no decision to make, and we reduce the search space immediately. If a cell has zero candidates, we know we've hit a dead end before wasting more work.

This is the **MRV** heuristic: at each step, we look for the empty cell with the fewest candidates. If `count_ones(candidates)` is 1, it's a forced move. If it's 0, we prune the branch instantly. This simple change transforms exponential backtracking into something that, in practice, solves any valid Sudoku in microseconds.

## Iterating Over Candidates with Bit Tricks

Once I have the candidate mask, I need to iterate over each set bit. Here I use a classic bit manipulation trick:

```
bit = candidates & !(candidates - 1)  // Isolate the lowest set bit
val = bit.trailing_zeros()              // Get the digit index
candidates &= !bit                     // Clear that bit for the next iteration
```

This lets me walk through only the valid candidates without any unnecessary loop from 1 to 9. Each iteration extracts a candidate, tries it, and if backtracking fails, undoes it cleanly with XOR.

## Rust Solution

```rust
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];
        let mut empty_count = 0;

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] != '.' {
                    let bit = 1 << (board[r][c] as u8 - b'1');
                    rows[r] |= bit;
                    cols[c] |= bit;
                    boxes[(r / 3) * 3 + (c / 3)] |= bit;
                } else {
                    empty_count += 1;
                }
            }
        }

        Self::backtrack(board, &mut rows, &mut cols, &mut boxes, empty_count);
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        rows: &mut [u16; 9],
        cols: &mut [u16; 9],
        boxes: &mut [u16; 9],
        count: i32,
    ) -> bool {
        if count == 0 {
            return true;
        }

        let mut min_candidates = 10;
        let mut best_r = 0;
        let mut best_c = 0;
        let mut best_mask = 0;

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    let b = (r / 3) * 3 + (c / 3);
                    let mask = !(rows[r] | cols[c] | boxes[b]) & 0x1FF;
                    let candidates_count = mask.count_ones();

                    if candidates_count < min_candidates {
                        min_candidates = candidates_count;
                        best_r = r;
                        best_c = c;
                        best_mask = mask;
                        if min_candidates == 1 {
                            break;
                        }
                    }
                }
            }
            if min_candidates == 1 {
                break;
            }
        }

        if min_candidates == 0 {
            return false;
        }

        let r = best_r;
        let c = best_c;
        let b = (r / 3) * 3 + (c / 3);
        let mut candidates = best_mask;

        while candidates > 0 {
            let bit = candidates & !(candidates - 1);
            let val_idx = bit.trailing_zeros();

            board[r][c] = (val_idx as u8 + b'1') as char;
            rows[r] |= bit;
            cols[c] |= bit;
            boxes[b] |= bit;

            if Self::backtrack(board, rows, cols, boxes, count - 1) {
                return true;
            }

            rows[r] ^= bit;
            cols[c] ^= bit;
            boxes[b] ^= bit;
            board[r][c] = '.';

            candidates &= !bit;
        }

        false
    }
}
```

The initialization phase scans the board once to build the three masks and count the empty cells. From there, `backtrack` does all the heavy lifting. What I like most about this implementation is that state is modified in-place with bit operations (OR to place, XOR to undo), with no need to copy the board or create auxiliary structures. Everything lives in three arrays of 9 sixteen-bit integers.

The use of `count_ones()` and `trailing_zeros()` in Rust is particularly elegant because the compiler translates them directly to hardware instructions (`POPCNT` and `TZCNT`), making these operations literally single CPU cycles.

## Conclusion

This problem is a perfect example of how bit manipulation techniques can transform a naive backtracking algorithm into something extremely efficient. Bitmasks compress constraint state into simple integers, the MRV heuristic prunes the search tree aggressively, and bit tricks allow iterating over candidates with zero waste. The result is a solver that, despite having a theoretical exponential complexity, solves Sudoku boards in practically instant time.
