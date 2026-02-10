use std::cmp;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();

        let mut dp = vec![i32::MAX; n + 1];

        dp[n - 1] = 1;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let min_hp_next = cmp::min(dp[j], dp[j + 1]);

                let need = min_hp_next - dungeon[i][j];

                dp[j] = if need <= 0 { 1 } else { need };
            }
        }

        dp[0]
    }
}
