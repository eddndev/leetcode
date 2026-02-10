impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut cur = 1;
        let mut k = k - 1;

        while k > 0 {
            let mut steps: i64 = 0;
            let mut first = cur as i64;
            let mut last = first + 1;
            let target = n as i64;

            while first <= target {
                steps += std::cmp::min(target + 1, last) - first;
                first *= 10;
                last *= 10;
            }

            if steps <= k as i64 {
                cur += 1;
                k -= steps as i32;
            } else {
                cur *= 10;
                k -= 1;
            }
        }
        cur
    }
}
