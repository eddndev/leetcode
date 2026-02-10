impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let m = t.len();
        let n = s.len();

        if n < m {
            return 0;
        }

        let mut dp = vec![0u64; m + 1];

        dp[0] = 1;

        for &s_char in s_bytes {
            for j in (0..m).rev() {
                if s_char == t_bytes[j] {
                    dp[j + 1] += dp[j];
                }
            }
        }

        dp[m] as i32
    }
}
