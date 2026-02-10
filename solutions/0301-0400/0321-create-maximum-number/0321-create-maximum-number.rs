impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let k = k as usize;
        let mut best_result = vec![0; k];

        let start = if k > n2 { k - n2 } else { 0 };
        let end = if k < n1 { k } else { n1 };

        for i in start..=end {
            let len1 = i;
            let len2 = k - i;

            let sub1 = Self::get_max_subsequence(&nums1, len1);
            let sub2 = Self::get_max_subsequence(&nums2, len2);

            let candidate = Self::merge(&sub1, &sub2);

            if candidate > best_result {
                best_result = candidate;
            }
        }

        best_result
    }

    fn get_max_subsequence(nums: &Vec<i32>, k: usize) -> Vec<i32> {
        let mut stack = Vec::with_capacity(k);
        let n = nums.len();
        let mut drop = n - k;

        for &val in nums {
            while drop > 0 && !stack.is_empty() && val > *stack.last().unwrap() {
                stack.pop();
                drop -= 1;
            }
            stack.push(val);
        }

        stack.truncate(k);
        stack
    }

    fn merge(s1: &Vec<i32>, s2: &Vec<i32>) -> Vec<i32> {
        let len = s1.len() + s2.len();
        let mut res = Vec::with_capacity(len);
        let mut i = 0;
        let mut j = 0;

        while i < s1.len() || j < s2.len() {
            if i < s1.len() && (j == s2.len() || s1[i..] > s2[j..]) {
                res.push(s1[i]);
                i += 1;
            } else {
                res.push(s2[j]);
                j += 1;
            }
        }
        res
    }
}
