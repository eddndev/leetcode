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
