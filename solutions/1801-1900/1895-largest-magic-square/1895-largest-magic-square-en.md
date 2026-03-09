---
title: "1895 Largest Magic Square - EN"
problemUrl: "https://leetcode.com/problems/largest-magic-square/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["array", "matrix", "prefix-sum"]
complexity:
  time: "O(m * n * min(m, n))"
  space: "O(m * n)"
---

# Largest Magic Square: Prefix Sums in Four Directions

## The Problem
Given an `m x n` grid of integers, find the largest square subgrid that forms a magic square. A magic square is a square where the sum of every row, every column, and both diagonals are all equal. Return the side length of the largest such square. Every 1x1 subgrid is trivially a magic square, so the answer is at least 1.

The brute force approach would recompute the sum of every row, column, and diagonal from scratch for each candidate square. That would add an extra factor of `k` per check. With prefix sums, we can verify any candidate in constant time.

## The Intuition: Precompute Everything

The core idea is that checking whether a subgrid is a magic square boils down to comparing sums: `k` rows, `k` columns, and 2 diagonals. If we precompute prefix sums along all four directions -- rows, columns, main diagonal, and anti-diagonal -- then each of those sums becomes an O(1) lookup.

For rows and columns, this is straightforward 1D prefix sums. For the main diagonal (top-left to bottom-right), we define `d1[r+1][c+1] = d1[r][c] + grid[r][c]`, so the sum along any diagonal segment of length `k` starting at `(r, c)` is `d1[r+k][c+k] - d1[r][c]`. The anti-diagonal (top-right to bottom-left) needs a slight offset shift: `d2[r+1][c+1] = d2[r][c+2] + grid[r][c]`, which accumulates values moving down-left. The sum of the anti-diagonal of a `k x k` square starting at `(r, c)` is `d2[r+k][c+1] - d2[r][c+k+1]`.

Once all four prefix arrays are built, we iterate from the largest possible side length down to 2. For each candidate size `k`, we try every top-left corner `(r, c)` and check whether the subgrid is magic. The moment we find a valid one, we return `k` immediately, since we are searching from largest to smallest. If no square of size 2 or larger works, the answer is 1.

## The Verification

The `is_magic` function takes a candidate square at position `(r, c)` with side `k` and checks:

1. **Rows** -- The first row's sum becomes the target. Every subsequent row must match it.
2. **Columns** -- All `k` columns must sum to the target.
3. **Main diagonal** -- Must equal the target.
4. **Anti-diagonal** -- Must equal the target.

Each check is a single subtraction on the corresponding prefix array. Any mismatch triggers an early return.

## Rust Solution

The four prefix arrays `rows`, `cols`, `d1`, and `d2` are allocated with extra padding to avoid boundary checks. The `#[inline(always)]` on `is_magic` encourages the compiler to eliminate the function call overhead in the tight inner loop. Iterating `k` in reverse with `.rev()` means we return the first valid result, which is guaranteed to be the largest.

```rust
impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut rows = vec![vec![0; n + 1]; m];
        let mut cols = vec![vec![0; m + 1]; n];
        let mut d1 = vec![vec![0; n + 2]; m + 2];
        let mut d2 = vec![vec![0; n + 2]; m + 2];

        for r in 0..m {
            for c in 0..n {
                let val = grid[r][c];
                rows[r][c + 1] = rows[r][c] + val;
                cols[c][r + 1] = cols[c][r] + val;
                d1[r + 1][c + 1] = d1[r][c] + val;
                d2[r + 1][c + 1] = d2[r][c + 2] + val;
            }
        }

        for k in (2..=m.min(n)).rev() {
            for r in 0..=(m - k) {
                for c in 0..=(n - k) {
                    if Self::is_magic(r, c, k, &grid, &rows, &cols, &d1, &d2) {
                        return k as i32;
                    }
                }
            }
        }

        1
    }

    #[inline(always)]
    fn is_magic(
        r: usize,
        c: usize,
        k: usize,
        grid: &Vec<Vec<i32>>,
        rows: &Vec<Vec<i32>>,
        cols: &Vec<Vec<i32>>,
        d1: &Vec<Vec<i32>>,
        d2: &Vec<Vec<i32>>,
    ) -> bool {
        let target = rows[r][c + k] - rows[r][c];

        for i in 1..k {
            if rows[r + i][c + k] - rows[r + i][c] != target {
                return false;
            }
        }

        for j in 0..k {
            if cols[c + j][r + k] - cols[c + j][r] != target {
                return false;
            }
        }

        if d1[r + k][c + k] - d1[r][c] != target {
            return false;
        }

        if d2[r + k][c + 1] - d2[r][c + k + 1] != target {
            return false;
        }

        true
    }
}
```

## Conclusion

The time complexity is $O(m \times n \times \min(m, n))$: for each candidate side length `k` (up to `min(m, n)` values), we check up to `O(m \times n)` positions, and each check takes O(k) time for the row and column loops, but in the worst case we still scan all positions for each `k`. The space complexity is $O(m \times n)$ for the four prefix sum arrays. The key takeaway is that prefix sums in four directions turn what would be an expensive per-element summation into constant-time range queries, and searching from largest to smallest lets us short-circuit the moment we find a valid magic square.
