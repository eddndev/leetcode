impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut low = 0;
        let mut high = 0;

        for &num in nums.iter() {
            low = low.max(num);
            high += num;
        }

        while low < high {
            let mid = low + (high - low) / 2;

            if Self::can_split(&nums, k, mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        low
    }

    fn can_split(nums: &[i32], k: i32, limit: i32) -> bool {
        let mut count = 1;
        let mut current_sum = 0;

        for &num in nums {
            if current_sum + num > limit {
                count += 1;
                current_sum = num;

                if count > k {
                    return false;
                }
            } else {
                current_sum += num;
            }
        }

        true
    }
}
