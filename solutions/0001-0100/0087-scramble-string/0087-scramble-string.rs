impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        if n != s2.len() {
            return false;
        }
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        let mut memo = vec![vec![vec![None; n]; n]; n + 1];

        Self::solve(s1_bytes, s2_bytes, 0, 0, n, &mut memo)
    }

    fn solve(
        s1: &[u8],
        s2: &[u8],
        i1: usize,
        i2: usize,
        len: usize,
        memo: &mut Vec<Vec<Vec<Option<bool>>>>,
    ) -> bool {
        if let Some(res) = memo[len][i1][i2] {
            return res;
        }
        if s1[i1..i1 + len] == s2[i2..i2 + len] {
            memo[len][i1][i2] = Some(true);
            return true;
        }

        let mut counts = [0; 26];
        for k in 0..len {
            counts[(s1[i1 + k] - b'a') as usize] += 1;
            counts[(s2[i2 + k] - b'a') as usize] -= 1;
        }
        if counts.iter().any(|&c| c != 0) {
            memo[len][i1][i2] = Some(false);
            return false;
        }

        for k in 1..len {
            if Self::solve(s1, s2, i1, i2, k, memo)
                && Self::solve(s1, s2, i1 + k, i2 + k, len - k, memo)
            {
                memo[len][i1][i2] = Some(true);
                return true;
            }

            if Self::solve(s1, s2, i1, i2 + len - k, k, memo)
                && Self::solve(s1, s2, i1 + k, i2, len - k, memo)
            {
                memo[len][i1][i2] = Some(true);
                return true;
            }
        }

        memo[len][i1][i2] = Some(false);
        false
    }
}
