impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut b_count = 0;
        let mut res = 0;

        for c in s.bytes() {
            if c == b'a' {
                res = std::cmp::min(res + 1, b_count);
            } else {
                b_count += 1;
            }
        }
        res
    }
}
