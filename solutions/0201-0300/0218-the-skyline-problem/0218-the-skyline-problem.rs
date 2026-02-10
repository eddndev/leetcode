use std::collections::BinaryHeap;

impl Solution {
    pub fn get_skyline(mut buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points = Vec::with_capacity(buildings.len() * 2);
        for b in &buildings {
            points.push(b[0]);
            points.push(b[1]);
        }
        points.sort_unstable();
        points.dedup();

        buildings.sort_unstable_by_key(|b| b[0]);

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut i = 0;
        let n = buildings.len();

        for &x in &points {
            while i < n && buildings[i][0] == x {
                heap.push((buildings[i][2], buildings[i][1]));
                i += 1;
            }

            while let Some(&(_, right)) = heap.peek() {
                if right <= x {
                    heap.pop();
                } else {
                    break;
                }
            }

            let curr_height = if let Some(&(h, _)) = heap.peek() {
                h
            } else {
                0
            };

            if result.is_empty() || result.last().unwrap()[1] != curr_height {
                result.push(vec![x, curr_height]);
            }
        }

        result
    }
}
