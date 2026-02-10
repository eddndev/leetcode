use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();

        let mut dp: HashMap<i32, HashSet<i32>> = HashMap::new();

        for &stone in &stones {
            dp.insert(stone, HashSet::new());
        }

        if let Some(start_set) = dp.get_mut(&stones[0]) {
            start_set.insert(0);
        }

        for i in 0..n {
            let current_stone = stones[i];

            let jumps = dp[&current_stone].clone();

            for &k in &jumps {
                for step in k - 1..=k + 1 {
                    if step > 0 {
                        let next_pos = current_stone + step;

                        if let Some(next_set) = dp.get_mut(&next_pos) {
                            next_set.insert(step);
                        }
                    }
                }
            }
        }

        !dp[&stones[n - 1]].is_empty()
    }
}
