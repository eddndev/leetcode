impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut result = Vec::new();
        if num.is_empty() {
            return result;
        }

        let chars: Vec<char> = num.chars().collect();
        let target = target as i64;

        Self::backtrack(0, "", 0, 0, &chars, target, &mut result);

        result
    }

    fn backtrack(
        index: usize,
        path: &str,
        current_val: i64,
        last_operand: i64,
        chars: &Vec<char>,
        target: i64,
        result: &mut Vec<String>,
    ) {
        if index == chars.len() {
            if current_val == target {
                result.push(path.to_string());
            }
            return;
        }

        for i in index..chars.len() {
            if i > index && chars[index] == '0' {
                break;
            }

            let part_str: String = chars[index..=i].iter().collect();
            let val: i64 = part_str.parse().unwrap();

            if index == 0 {
                Self::backtrack(i + 1, &part_str, val, val, chars, target, result);
            } else {
                Self::backtrack(
                    i + 1,
                    &format!("{}+{}", path, part_str),
                    current_val + val,
                    val,
                    chars,
                    target,
                    result,
                );

                Self::backtrack(
                    i + 1,
                    &format!("{}-{}", path, part_str),
                    current_val - val,
                    -val,
                    chars,
                    target,
                    result,
                );

                Self::backtrack(
                    i + 1,
                    &format!("{}*{}", path, part_str),
                    (current_val - last_operand) + (last_operand * val),
                    last_operand * val,
                    chars,
                    target,
                    result,
                );
            }
        }
    }
}
