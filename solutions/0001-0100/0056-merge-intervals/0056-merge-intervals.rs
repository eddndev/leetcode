impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut result: Vec<Vec<i32>> = Vec::new();

        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        for interval in intervals {
            match result.last_mut() {
                Some(last) if last[1] >= interval[0] => {
                    last[1] = last[1].max(interval[1]);
                }

                _ => {
                    result.push(interval);
                }
            }
        }

        result
    }
}
