impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n = nums.len();
        let mut sums = vec![0i64; n + 1];
        for i in 0..n {
            sums[i + 1] = sums[i] + nums[i] as i64;
        }

        let mut cache = vec![0i64; n + 1];

        Self::merge_sort_recursive(&mut sums, &mut cache, 0, n + 1, lower as i64, upper as i64)
    }

    fn merge_sort_recursive(
        sums: &mut [i64],
        cache: &mut [i64],
        left: usize,
        right: usize,
        lower: i64,
        upper: i64,
    ) -> i32 {
        if right - left <= 1 {
            return 0;
        }

        let mid = left + (right - left) / 2;

        let mut count = Self::merge_sort_recursive(sums, cache, left, mid, lower, upper)
            + Self::merge_sort_recursive(sums, cache, mid, right, lower, upper);

        let mut k = mid;
        let mut m = mid;

        for i in left..mid {
            while k < right && sums[k] - sums[i] < lower {
                k += 1;
            }
            while m < right && sums[m] - sums[i] <= upper {
                m += 1;
            }
            count += (m - k) as i32;
        }

        let mut i = left;
        let mut j = mid;
        let mut idx = 0;

        while i < mid && j < right {
            if sums[i] < sums[j] {
                cache[left + idx] = sums[i];
                i += 1;
            } else {
                cache[left + idx] = sums[j];
                j += 1;
            }
            idx += 1;
        }

        while i < mid {
            cache[left + idx] = sums[i];
            i += 1;
            idx += 1;
        }
        while j < right {
            cache[left + idx] = sums[j];
            j += 1;
            idx += 1;
        }

        sums[left..right].copy_from_slice(&cache[left..right]);

        count
    }
}
