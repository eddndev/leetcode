impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut heights = vec![0; cols];
        let mut max_area = 0;

        let mut stack = Vec::with_capacity(cols + 1);

        for row in matrix {
            for (i, &val) in row.iter().enumerate() {
                if val == '1' {
                    heights[i] += 1;
                } else {
                    heights[i] = 0;
                }
            }

            stack.clear();

            for i in 0..=cols {
                let current_h = if i == cols { 0 } else { heights[i] };

                while let Some(&top) = stack.last() {
                    if current_h < heights[top] {
                        stack.pop();
                        let h = heights[top];

                        let w = if let Some(&prev) = stack.last() {
                            i - prev - 1
                        } else {
                            i
                        };

                        max_area = max_area.max(h * w as i32);
                    } else {
                        break;
                    }
                }
                stack.push(i);
            }
        }

        max_area
    }
}
