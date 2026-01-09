// Difficulty: Easy
// Time: O(n)
// Space: O(n)

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - n)) {
                return vec![j as i32, i as i32];
            }
            map.insert(n, i);
        }
        vec![]
    }
}
