use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let chars: Vec<char> = password.chars().collect();
        let n = chars.len() as i32;

        let has_lower = chars.iter().any(|c| c.is_ascii_lowercase());
        let has_upper = chars.iter().any(|c| c.is_ascii_uppercase());
        let has_digit = chars.iter().any(|c| c.is_ascii_digit());

        let missing_types = !has_lower as i32 + !has_upper as i32 + !has_digit as i32;

        let mut replace = 0;
        let mut one_seq = Vec::new();
        let mut two_seq = Vec::new();

        let mut i = 2;
        while i < n {
            if chars[i as usize] == chars[(i - 1) as usize]
                && chars[i as usize] == chars[(i - 2) as usize]
            {
                let mut length = 2;
                while i < n && chars[i as usize] == chars[(i - 1) as usize] {
                    length += 1;
                    i += 1;
                }
                replace += length / 3;
                if length % 3 == 0 {
                    one_seq.push(length);
                } else if length % 3 == 1 {
                    two_seq.push(length);
                }
            } else {
                i += 1;
            }
        }

        if n < 6 {
            max(missing_types, 6 - n)
        } else if n <= 20 {
            max(missing_types, replace)
        } else {
            let delete_needed = n - 20;
            let mut delete_left = delete_needed;

            replace -= min(delete_left, one_seq.len() as i32 * 1) / 1;
            delete_left = max(0, delete_left - one_seq.len() as i32 * 1);

            replace -= min(delete_left, two_seq.len() as i32 * 2) / 2;
            delete_left = max(0, delete_left - two_seq.len() as i32 * 2);

            replace -= delete_left / 3;

            delete_needed + max(missing_types, replace)
        }
    }
}
