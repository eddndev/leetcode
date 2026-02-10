impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut padded_nums = Vec::with_capacity(n + 2);
        padded_nums.push(1);
        padded_nums.extend(nums);
        padded_nums.push(1);

        let len = padded_nums.len();

        let mut dp = vec![vec![0; len]; len];

        for window in 2..len {
            for left in 0..len - window {
                let right = left + window;

                for k in (left + 1)..right {
                    let coins = dp[left][k]
                        + dp[k][right]
                        + (padded_nums[left] * padded_nums[k] * padded_nums[right]);

                    if coins > dp[left][right] {
                        dp[left][right] = coins;
                    }
                }
            }
        }

        dp[0][len - 1]
    }
}
