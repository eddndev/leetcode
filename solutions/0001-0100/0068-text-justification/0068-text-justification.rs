impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = Vec::new();
        let mut current_line: Vec<&String> = Vec::new();
        let mut current_len = 0;

        for word in &words {
            if current_len + word.len() + current_line.len() > max_width {
                result.push(Self::justify_line(
                    &current_line,
                    current_len,
                    max_width,
                    false,
                ));

                current_line.clear();
                current_len = 0;
            }

            current_line.push(word);
            current_len += word.len();
        }

        if !current_line.is_empty() {
            result.push(Self::justify_line(
                &current_line,
                current_len,
                max_width,
                true,
            ));
        }

        result
    }

    fn justify_line(
        line: &[&String],
        line_char_len: usize,
        max_width: usize,
        is_last_line: bool,
    ) -> String {
        if is_last_line || line.len() == 1 {
            let mut s = String::with_capacity(max_width);

            for (i, word) in line.iter().enumerate() {
                if i > 0 {
                    s.push(' ');
                }
                s.push_str(word);
            }

            let remaining = max_width - s.len();
            for _ in 0..remaining {
                s.push(' ');
            }
            return s;
        }

        let gaps = line.len() - 1;
        let total_spaces = max_width - line_char_len;

        let spaces_per_gap = total_spaces / gaps;
        let extra_spaces = total_spaces % gaps;

        let mut s = String::with_capacity(max_width);

        for (i, word) in line.iter().enumerate() {
            s.push_str(word);

            if i < gaps {
                let spaces_to_add = spaces_per_gap + if i < extra_spaces { 1 } else { 0 };
                for _ in 0..spaces_to_add {
                    s.push(' ');
                }
            }
        }

        s
    }
}
