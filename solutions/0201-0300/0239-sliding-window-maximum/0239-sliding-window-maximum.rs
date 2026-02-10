use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut deque: VecDeque<usize> = VecDeque::with_capacity(k);
        let mut result = Vec::with_capacity(n - k + 1);

        for i in 0..n {
            if let Some(&front) = deque.front() {
                if i >= k && front <= i - k {
                    deque.pop_front();
                }
            }

            while let Some(&back) = deque.back() {
                if nums[back] <= nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(i);

            if i >= k - 1 {
                result.push(nums[*deque.front().unwrap()]);
            }
        }

        result
    }
}
