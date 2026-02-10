impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut stack: Vec<usize> = Vec::with_capacity(n);
        let mut max_area = 0;

        for i in 0..=n {
            let current_h = if i == n { 0 } else { heights[i] };

            while let Some(&top_index) = stack.last() {
                if current_h < heights[top_index] {
                    stack.pop();
                    let height = heights[top_index];
                    let width = if let Some(&prev_index) = stack.last() {
                        i - prev_index - 1
                    } else {
                        i
                    };

                    max_area = max_area.max(height * width as i32);
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        max_area
    }
}
