impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut result = 0;
        let mut current_number = 0;
        let mut sign = 1;

        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            if c.is_ascii_digit() {
                current_number = 0;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    current_number = current_number * 10 + (chars[i] as i32 - '0' as i32);
                    i += 1;
                }
                result += sign * current_number;
                i -= 1;
            } else if c == '+' {
                sign = 1;
            } else if c == '-' {
                sign = -1;
            } else if c == '(' {
                stack.push(result);
                stack.push(sign);
                result = 0;
                sign = 1;
            } else if c == ')' {
                let prev_sign = stack.pop().unwrap();
                let prev_result = stack.pop().unwrap();
                result = prev_result + (prev_sign * result);
            }
            i += 1;
        }

        result
    }
}
