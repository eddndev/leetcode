impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];
        let mut empty_count = 0;

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] != '.' {
                    let bit = 1 << (board[r][c] as u8 - b'1');
                    rows[r] |= bit;
                    cols[c] |= bit;
                    boxes[(r / 3) * 3 + (c / 3)] |= bit;
                } else {
                    empty_count += 1;
                }
            }
        }

        Self::backtrack(board, &mut rows, &mut cols, &mut boxes, empty_count);
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        rows: &mut [u16; 9],
        cols: &mut [u16; 9],
        boxes: &mut [u16; 9],
        count: i32,
    ) -> bool {
        if count == 0 {
            return true;
        }

        let mut min_candidates = 10;
        let mut best_r = 0;
        let mut best_c = 0;
        let mut best_mask = 0;

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    let b = (r / 3) * 3 + (c / 3);
                    let mask = !(rows[r] | cols[c] | boxes[b]) & 0x1FF;
                    let candidates_count = mask.count_ones();

                    if candidates_count < min_candidates {
                        min_candidates = candidates_count;
                        best_r = r;
                        best_c = c;
                        best_mask = mask;
                        if min_candidates == 1 {
                            break;
                        }
                    }
                }
            }
            if min_candidates == 1 {
                break;
            }
        }

        if min_candidates == 0 {
            return false;
        }

        let r = best_r;
        let c = best_c;
        let b = (r / 3) * 3 + (c / 3);
        let mut candidates = best_mask;

        while candidates > 0 {
            let bit = candidates & !(candidates - 1);
            let val_idx = bit.trailing_zeros();

            board[r][c] = (val_idx as u8 + b'1') as char;
            rows[r] |= bit;
            cols[c] |= bit;
            boxes[b] |= bit;

            if Self::backtrack(board, rows, cols, boxes, count - 1) {
                return true;
            }

            rows[r] ^= bit;
            cols[c] ^= bit;
            boxes[b] ^= bit;
            board[r][c] = '.';

            candidates &= !bit;
        }

        false
    }
}
