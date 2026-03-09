---
title: "0840 Magic Squares In Grid - EN"
problemUrl: "https://leetcode.com/problems/magic-squares-in-grid/"
difficulty: "Medium"
pubDate: "2026-03-08"
tags: ["array", "matrix", "math"]
complexity:
  time: "O(R * C)"
  space: "O(1)"
---

# Magic Squares In Grid: The Center Tells You Everything

## The Problem
Given a `grid` of integers, count how many 3x3 sub-grids are magic squares. A 3x3 magic square is a 3x3 grid filled with distinct numbers from 1 to 9 such that every row, column, and both diagonals all sum to the same number (15).

The naive approach would be to check every 3x3 sub-grid by verifying all properties one by one. But there is an elegant shortcut that saves us unnecessary work.

## The Intuition: 5 Is Always at the Center

The magic sum for a 3x3 grid with numbers 1 through 9 is always 15 (the total sum 1+2+...+9 = 45 divided by 3 rows). There is a less obvious property: the center of any valid 3x3 magic square is **always 5**. You can prove this by summing both diagonals and the center row and column: the center cell appears in all four sums, and the only way everything works out is if it equals 5.

This gives us a fast filter. Before doing any expensive verification, we check whether the center of the sub-grid is 5. If it is not, we skip it immediately.

After that filter, we verify three things in order:
1. **All values are distinct and between 1 and 9** - we use a `seen` boolean array of size 10.
2. **All three rows sum to 15** - if any fails, we move on to the next sub-grid.
3. **All three columns and both diagonals sum to 15** - the final check.

The order matters. Each check acts as a short-circuit: if it fails, we jump to the next candidate without spending time on the remaining checks.

## C Solution

The grid handling in C with double pointers directly mirrors LeetCode's function signature. The fixed-size `seen` array (10 elements) means there is no dynamic allocation. The chained `continue` statements keep the flow flat instead of nesting conditionals deeply.

```c
#include <stdbool.h>
#include <string.h>

int numMagicSquaresInside(int **grid, int gridSize, int *gridColSize) {
    size_t r = gridSize, c = gridColSize[0];

    if (r < 3 || c < 3) return 0;

    int result = 0;
    for (int i = 0; i <= r - 3; i++) {
        for (int j = 0; j <= c - 3; j++) {
            if (grid[i + 1][j + 1] != 5) {
                continue;
            }

            bool seen[10] = {false};
            bool valid = true;

            for (int k = i; k < i + 3; k++) {
                for (int m = j; m < j + 3; m++) {
                    int val = grid[k][m];
                    if (val < 1 || val > 9 || seen[val]) {
                        valid = false;
                        break;
                    }
                    seen[val] = true;
                }
                if (!valid) break;
            }

            if (!valid) continue;

            if (grid[i][j] + grid[i][j + 1] + grid[i][j + 2] != 15) continue;
            if (grid[i + 1][j] + grid[i + 1][j + 1] + grid[i + 1][j + 2] != 15) continue;
            if (grid[i + 2][j] + grid[i + 2][j + 1] + grid[i + 2][j + 2] != 15) continue;

            if (grid[i][j] + grid[i + 1][j] + grid[i + 2][j] != 15) continue;
            if (grid[i][j + 1] + grid[i + 1][j + 1] + grid[i + 2][j + 1] != 15) continue;
            if (grid[i][j + 2] + grid[i + 1][j + 2] + grid[i + 2][j + 2] != 15) continue;

            if (grid[i][j] + grid[i + 1][j + 1] + grid[i + 2][j + 2] != 15) continue;
            if (grid[i][j + 2] + grid[i + 1][j + 1] + grid[i + 2][j] != 15) continue;

            result++;
        }
    }
    return result;
}
```

## Conclusion

Time complexity is $O(R \times C)$ where R and C are the grid dimensions: we iterate over every possible sub-grid position, and each check takes constant time (it is always 3x3). Space is $O(1)$ since the `seen` array has a fixed size. The center-equals-5 trick is the observation that transforms an exhaustive check into something that discards most candidates instantly.
