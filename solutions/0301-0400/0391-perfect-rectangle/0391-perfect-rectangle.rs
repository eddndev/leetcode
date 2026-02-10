use std::collections::HashSet;
use std::i32;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        if rectangles.is_empty() {
            return false;
        }

        let mut corners = HashSet::new();
        let mut area_sum: i64 = 0;

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for rect in &rectangles {
            let x1 = rect[0];
            let y1 = rect[1];
            let x2 = rect[2];
            let y2 = rect[3];

            min_x = min_x.min(x1);
            min_y = min_y.min(y1);
            max_x = max_x.max(x2);
            max_y = max_y.max(y2);

            area_sum += (x2 - x1) as i64 * (y2 - y1) as i64;

            let points = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];

            for p in points.iter() {
                if corners.contains(p) {
                    corners.remove(p);
                } else {
                    corners.insert(*p);
                }
            }
        }

        if corners.len() != 4 {
            return false;
        }

        let expected_corners = [
            (min_x, min_y),
            (min_x, max_y),
            (max_x, min_y),
            (max_x, max_y),
        ];

        for p in expected_corners.iter() {
            if !corners.contains(p) {
                return false;
            }
        }

        let expected_area = (max_x - min_x) as i64 * (max_y - min_y) as i64;

        area_sum == expected_area
    }
}
