use std::cmp::max;
use std::collections::BTreeSet;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = matrix.len();
        if rows == 0 {
            return 0;
        }
        let cols = matrix[0].len();

        let mut max_sum = i32::MIN;

        for r1 in 0..rows {
            let mut col_sums = vec![0; cols];

            for r2 in r1..rows {
                for c in 0..cols {
                    col_sums[c] += matrix[r2][c];
                }

                let mut current_prefix_sum = 0;
                let mut set = BTreeSet::new();

                set.insert(0);

                for &val in &col_sums {
                    current_prefix_sum += val;

                    let target = current_prefix_sum - k;

                    if let Some(&prev_sum) = set.range(target..).next() {
                        max_sum = max(max_sum, current_prefix_sum - prev_sum);
                    }

                    set.insert(current_prefix_sum);
                }

                if max_sum == k {
                    return k;
                }
            }
        }

        max_sum
    }
}
