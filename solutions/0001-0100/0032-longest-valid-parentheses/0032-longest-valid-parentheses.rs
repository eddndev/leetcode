use std::cmp;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }

        let mut stack: Vec<i32> = Vec::with_capacity(n + 1);

        stack.push(-1);

        let mut max_len = 0;

        for (i, &byte) in s.as_bytes().iter().enumerate() {
            if byte == b'(' {
                stack.push(i as i32);
            } else {
                stack.pop();

                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    let current_len = (i as i32) - stack.last().unwrap();
                    max_len = cmp::max(max_len, current_len);
                }
            }
        }

        max_len
    }
}
