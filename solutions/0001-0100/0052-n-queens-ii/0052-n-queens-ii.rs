impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        Self::solve(n, 0, 0, 0, 0)
    }

    fn solve(n: i32, row: i32, cols: i32, diags: i32, anti_diags: i32) -> i32 {
        if row == n {
            return 1;
        }

        let mut count = 0;

        let mut available = ((1 << n) - 1) & !(cols | diags | anti_diags);

        while available != 0 {
            let position = available & -available;

            available = available & (available - 1);

            count += Self::solve(
                n,
                row + 1,
                cols | position,
                (diags | position) << 1,
                (anti_diags | position) >> 1,
            );
        }

        count
    }
}
