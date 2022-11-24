// 79. Word Search

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn backtracking(
            board: &Vec<Vec<char>>,
            i: i32,
            j: i32,
            m: i32,
            n: i32,
            rest: &[u8],
            seen: &mut Vec<Vec<bool>>,
        ) -> bool {
            if rest.is_empty() {
                true
            } else {
                seen[i as usize][j as usize] = true;
                if vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .map(|(di, dj)| (i + di, j + dj))
                    .any(|(i, j)| {
                        i >= 0
                            && i < m
                            && j >= 0
                            && j < n
                            && !seen[i as usize][j as usize]
                            && board[i as usize][j as usize] as u8 == rest[0]
                            && backtracking(board, i, j, m, n, &rest[1..], seen)
                    })
                {
                    return true;
                }
                seen[i as usize][j as usize] = false;
                false
            }
        }
        let m = board.len();
        let n = board[0].len();
        let bytes = word.as_bytes();
        let mut seen = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                if board[i][j] as u8 == bytes[0]
                    && backtracking(
                        &board,
                        i as i32,
                        j as i32,
                        m as i32,
                        n as i32,
                        &bytes[1..],
                        &mut seen,
                    )
                {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCCED".to_owned(),
        );
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "EES".to_owned(),
        );
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCB".to_owned(),
        );
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::exist(vec![vec!['A']], "A".to_owned());
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::exist(vec![vec!['A']], "B".to_owned());
        assert_eq!(false, result)
    }
}
