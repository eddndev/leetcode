use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        if value_diff < 0 {
            return false;
        }

        let mut buckets: HashMap<i64, i64> = HashMap::new();
        let w = value_diff as i64 + 1;

        let get_bucket_id = |val: i64| -> i64 {
            if val >= 0 {
                val / w
            } else {
                (val + 1) / w - 1
            }
        };

        for (i, &num) in nums.iter().enumerate() {
            let val = num as i64;
            let bucket_id = get_bucket_id(val);

            if buckets.contains_key(&bucket_id) {
                return true;
            }

            if let Some(&neighbor) = buckets.get(&(bucket_id - 1)) {
                if (val - neighbor).abs() < w {
                    return true;
                }
            }

            if let Some(&neighbor) = buckets.get(&(bucket_id + 1)) {
                if (val - neighbor).abs() < w {
                    return true;
                }
            }

            buckets.insert(bucket_id, val);

            if i as i32 >= index_diff {
                let old_val = nums[i - index_diff as usize] as i64;
                let old_bucket = get_bucket_id(old_val);
                buckets.remove(&old_bucket);
            }
        }

        false
    }
}
