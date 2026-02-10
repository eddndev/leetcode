use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut total = 0;
        let mut dp: Vec<HashMap<i64, i32>> = vec![HashMap::new(); n];

        for i in 0..n {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;
                let count = *dp[j].get(&diff).unwrap_or(&0);

                total += count;

                *dp[i].entry(diff).or_insert(0) += count + 1;
            }
        }

        total
    }
}
