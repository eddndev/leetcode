impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut dp = vec![vec![0; n + 1]; m + 1];

        let mut max_len = 0;

        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = mat[i - 1][j - 1] + dp[i - 1][j] + dp[i][j - 1] - dp[i - 1][j - 1];

                let current_len = max_len + 1;

                if i >= current_len && j >= current_len {
                    let sum = dp[i][j] - dp[i - current_len][j] - dp[i][j - current_len]
                        + dp[i - current_len][j - current_len];

                    if sum <= threshold {
                        max_len += 1;
                    }
                }
            }
        }

        max_len as i32
    }
}
