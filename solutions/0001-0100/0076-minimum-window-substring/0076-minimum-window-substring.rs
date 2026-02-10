impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let mut map = [0; 128];

        for &b in t_bytes {
            map[b as usize] += 1;
        }

        let mut left = 0;
        let mut min_len = usize::MAX;
        let mut start_index = 0;
        let mut count = t.len();
        for (right, &char_right) in s_bytes.iter().enumerate() {
            if map[char_right as usize] > 0 {
                count -= 1;
            }
            map[char_right as usize] -= 1;

            while count == 0 {
                let current_len = right - left + 1;

                if current_len < min_len {
                    min_len = current_len;
                    start_index = left;
                }

                let char_left = s_bytes[left];
                map[char_left as usize] += 1;

                if map[char_left as usize] > 0 {
                    count += 1;
                }

                left += 1;
            }
        }

        if min_len == usize::MAX {
            "".to_string()
        } else {
            String::from_utf8_lossy(&s_bytes[start_index..start_index + min_len]).to_string()
        }
    }
}
