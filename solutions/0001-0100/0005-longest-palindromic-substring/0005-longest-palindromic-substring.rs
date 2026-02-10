impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut max_len = 0;

        for i in 0..chars.len() {
            let (s1, l1) = Self::expand(&chars, i, i);

            let (s2, l2) = Self::expand(&chars, i, i + 1);

            if l1 > max_len {
                max_len = l1;
                start = s1;
            }

            if l2 > max_len {
                max_len = l2;
                start = s2;
            }
        }

        chars[start..start + max_len].iter().collect()
    }

    fn expand(chars: &[char], mut l: usize, mut r: usize) -> (usize, usize) {
        let n = chars.len();

        while r < n && chars[l] == chars[r] {
            if l == 0 && chars[l] == chars[r] {
                if l == 0 {
                    return (0, r + 1);
                }
            }
            l -= 1;
            r += 1;

            if r >= n || chars[l] != chars[r] {
                return (l + 1, r - (l + 1));
            }
        }

        (l + 1, r - (l + 1))
    }
}
