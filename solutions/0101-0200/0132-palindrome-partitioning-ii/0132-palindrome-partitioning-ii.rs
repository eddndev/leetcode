use std::cmp;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }

        let s = s.as_bytes();

        let mut dp: Vec<i32> = (0..n as i32).collect();

        for center in 0..n {
            let (mut l, mut r) = (center, center);
            while r < n && s[l] == s[r] {
                let new_cut = if l == 0 { 0 } else { dp[l - 1] + 1 };
                if new_cut < dp[r] {
                    dp[r] = new_cut;
                }

                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }

            let (mut l, mut r) = (center, center + 1);
            while r < n && s[l] == s[r] {
                let new_cut = if l == 0 { 0 } else { dp[l - 1] + 1 };
                if new_cut < dp[r] {
                    dp[r] = new_cut;
                }

                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }
        }

        dp[n - 1]
    }
}
