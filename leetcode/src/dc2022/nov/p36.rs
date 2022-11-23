// 36. Valid Sudoku

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut seen_row = vec![vec![false; 9]; 9];
        let mut seen_col = vec![vec![false; 9]; 9];
        let mut seen_sub_box = vec![vec![false; 9]; 9];
        for col in 0..9 {
            for row in 0..9 {
                if let Some(v) = board[col][row].to_digit(10) {
                    let v = (v - 1) as usize;
                    let sb = (col / 3) * 3 + row / 3;
                    if seen_row[row][v] || seen_col[col][v] || seen_sub_box[sb][v] {
                        return false;
                    }
                    seen_col[col][v] = true;
                    seen_row[row][v] = true;
                    seen_sub_box[sb][v] = true;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_valid_sudoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
        assert_eq!(false, result)
    }
}
