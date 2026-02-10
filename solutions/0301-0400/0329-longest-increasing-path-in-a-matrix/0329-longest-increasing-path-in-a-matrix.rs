use std::cmp;

impl Solution {
    const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let m = matrix.len();
        let n = matrix[0].len();

        let mut cache = vec![vec![0; n]; m];
        let mut max_len = 0;

        for i in 0..m {
            for j in 0..n {
                max_len = cmp::max(max_len, Self::dfs(&matrix, &mut cache, i, j, m, n));
            }
        }

        max_len
    }

    fn dfs(
        matrix: &Vec<Vec<i32>>,
        cache: &mut Vec<Vec<i32>>,
        i: usize,
        j: usize,
        m: usize,
        n: usize,
    ) -> i32 {
        if cache[i][j] != 0 {
            return cache[i][j];
        }

        let mut current_max = 1;
        let current_val = matrix[i][j];

        for &(di, dj) in &Self::DIRS {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if ni >= 0 && ni < m as isize && nj >= 0 && nj < n as isize {
                let ni = ni as usize;
                let nj = nj as usize;

                if matrix[ni][nj] > current_val {
                    current_max = cmp::max(current_max, 1 + Self::dfs(matrix, cache, ni, nj, m, n));
                }
            }
        }

        cache[i][j] = current_max;
        current_max
    }
}
