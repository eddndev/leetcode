impl Solution {
    pub fn is_number(s: String) -> bool {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut seen_digit = false;
        let mut seen_exponent = false;
        let mut seen_dot = false;

        for (i, &b) in bytes.iter().enumerate() {
            match b {
                b'0'..=b'9' => {
                    seen_digit = true;
                }

                b'+' | b'-' => {
                    if i > 0 && bytes[i - 1] != b'e' && bytes[i - 1] != b'E' {
                        return false;
                    }
                }

                b'e' | b'E' => {
                    if seen_exponent || !seen_digit {
                        return false;
                    }
                    seen_exponent = true;

                    seen_digit = false;
                }

                b'.' => {
                    if seen_dot || seen_exponent {
                        return false;
                    }
                    seen_dot = true;
                }

                _ => return false,
            }
        }

        seen_digit
    }
}
