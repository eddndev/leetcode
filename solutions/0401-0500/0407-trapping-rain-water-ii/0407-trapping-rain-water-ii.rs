use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();

        if m < 3 || n < 3 {
            return 0;
        }

        let mut heap = BinaryHeap::new();
        let mut visited = vec![vec![false; n]; m];

        for r in 0..m {
            for c in 0..n {
                if r == 0 || r == m - 1 || c == 0 || c == n - 1 {
                    heap.push(Reverse((height_map[r][c], r, c)));
                    visited[r][c] = true;
                }
            }
        }

        let mut total_water = 0;

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some(Reverse((h, r, c))) = heap.pop() {
            for (dr, dc) in directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < m as isize && nc >= 0 && nc < n as isize {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if !visited[nr][nc] {
                        visited[nr][nc] = true;

                        let neighbor_height = height_map[nr][nc];

                        if neighbor_height < h {
                            total_water += h - neighbor_height;
                        }

                        heap.push(Reverse((h.max(neighbor_height), nr, nc)));
                    }
                }
            }
        }

        total_water
    }
}
