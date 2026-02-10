impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen = [0usize; 128];
        let mut max_len = 0;
        let mut left = 0;

        // .as_bytes() is O(1) in Rust 'cause String already saves the buffer internaly
        // .enumerate() gives us the index (right) and the value (byte)
        for (right, &byte) in s.as_bytes().iter().enumerate() {
            let idx = byte as usize;

            // If the character has been seen in the current range of the window
            if last_seen[idx] > left {
                left = last_seen[idx];
            }

            // We update the position of the character (idx + 1)
            last_seen[idx] = right + 1;

            let current_len = (right - left + 1) as i32;
            if current_len > max_len {
                max_len = current_len;
            }
        }

        max_len
    }
}
