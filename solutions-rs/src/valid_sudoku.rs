use crate::Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] != '.' && !is_valid_placement(&board, row, col) {
                    return false;
                }
            }
        }
        return true;
    }
}

fn is_valid_placement(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let value = board[row][col];
    for i in 0..9 {
        let is_row_invalid = board[row][i] == value && i != col;
        let is_col_invalid = board[i][col] == value && i != row;

        let sub_row = 3 * (row / 3) + i / 3;
        let sub_col = 3 * (col / 3) + i % 3;
        let is_block_invalid = board[sub_row][sub_col] == value && sub_row != row && sub_col != col;

        if is_row_invalid || is_col_invalid || is_block_invalid {
            return false;
        }
    }

    return true;
}
