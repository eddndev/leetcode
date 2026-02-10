impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let n = s.len();
        if n == 0 {
            return s;
        }

        let s_bytes = s.as_bytes();
        let mut rev_s_bytes = s_bytes.to_vec();
        rev_s_bytes.reverse();

        let mut combined = Vec::with_capacity(2 * n + 1);
        combined.extend_from_slice(s_bytes);
        combined.push(b'#');
        combined.extend_from_slice(&rev_s_bytes);

        let m = combined.len();
        let mut lps = vec![0; m];
        let mut j = 0;

        for i in 1..m {
            while j > 0 && combined[i] != combined[j] {
                j = lps[j - 1];
            }
            if combined[i] == combined[j] {
                j += 1;
            }
            lps[i] = j;
        }

        let palindrome_len = lps[m - 1];

        let suffix_to_add =
            unsafe { String::from_utf8_unchecked(rev_s_bytes[0..n - palindrome_len].to_vec()) };

        suffix_to_add + &s
    }
}
