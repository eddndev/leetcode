impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut results = Vec::new();
        let limit = (1 << n) - 1;
        let mut current_board: Vec<usize> = Vec::with_capacity(n as usize);

        Self::backtrack(0, 0, 0, limit, n, &mut current_board, &mut results);

        results
    }

    fn backtrack(
        cols: i32,
        ld: i32,
        rd: i32,
        limit: i32,
        n: i32,
        current_board: &mut Vec<usize>,
        results: &mut Vec<Vec<String>>,
    ) {
        if cols == limit {
            results.push(Self::format_board(current_board, n));
            return;
        }

        let mut possibilities = !(cols | ld | rd) & limit;

        while possibilities > 0 {
            let bit = possibilities & -possibilities;

            let col_idx = bit.trailing_zeros() as usize;

            current_board.push(col_idx);

            Self::backtrack(
                cols | bit,
                (ld | bit) << 1,
                (rd | bit) >> 1,
                limit,
                n,
                current_board,
                results,
            );

            current_board.pop();

            possibilities ^= bit;
        }
    }

    fn format_board(indices: &Vec<usize>, n: i32) -> Vec<String> {
        let mut board = Vec::with_capacity(n as usize);
        for &col in indices {
            let mut row_str = String::with_capacity(n as usize);
            for i in 0..n {
                if i == col as i32 {
                    row_str.push('Q');
                } else {
                    row_str.push('.');
                }
            }
            board.push(row_str);
        }
        board
    }
}
