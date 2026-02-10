use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 2 {
            return n as i32;
        }

        let mut max_points = 1;

        for i in 0..n {
            let p1 = &points[i];
            let mut slopes: HashMap<(i32, i32), i32> = HashMap::new();

            for j in i + 1..n {
                let p2 = &points[j];

                let delta_y = p2[1] - p1[1];
                let delta_x = p2[0] - p1[0];

                let slope = Self::get_normalized_slope(delta_y, delta_x);

                *slopes.entry(slope).or_insert(0) += 1;
            }

            let current_max = slopes.values().max().unwrap_or(&0) + 1;
            max_points = max_points.max(current_max);
        }

        max_points
    }

    fn get_normalized_slope(dy: i32, dx: i32) -> (i32, i32) {
        if dx == 0 {
            return (1, 0);
        }
        if dy == 0 {
            return (0, 1);
        }

        let divisor = Self::gcd(dy.abs(), dx.abs());
        let mut res_dy = dy / divisor;
        let mut res_dx = dx / divisor;

        if res_dx < 0 {
            res_dy = -res_dy;
            res_dx = -res_dx;
        }

        (res_dy, res_dx)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
