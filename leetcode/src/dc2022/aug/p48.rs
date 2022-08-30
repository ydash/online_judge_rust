// 48. Rotate Image

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            for j in 0..n - 2 * i - 1 {
                let tmp = matrix[i][i + j];
                matrix[i][i + j] = matrix[n - i - j - 1][i];
                matrix[n - i - j - 1][i] = matrix[n - i - 1][n - i - j - 1];
                matrix[n - i - 1][n - i - j - 1] = matrix[i + j][n - i - 1];
                matrix[i + j][n - i - 1] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]], matrix)
    }

    #[test]
    fn test_case_02() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ],
            matrix
        )
    }

    #[test]
    fn test_case_03() {
        let mut matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 3, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            vec![
                vec![21, 16, 11, 6, 1],
                vec![22, 17, 12, 7, 2],
                vec![23, 18, 3, 8, 3],
                vec![24, 19, 14, 9, 4],
                vec![25, 20, 15, 10, 5],
            ],
            matrix
        )
    }

    #[test]
    fn test_case_04() {
        let mut matrix = vec![
            vec![1, 2, 3, 4, 5, 6],
            vec![7, 8, 9, 10, 11, 12],
            vec![13, 14, 15, 16, 17, 18],
            vec![19, 20, 21, 22, 23, 24],
            vec![25, 26, 27, 28, 29, 30],
            vec![31, 32, 33, 34, 35, 36],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            vec![
                vec![31, 25, 19, 13, 7, 1],
                vec![32, 26, 20, 14, 8, 2],
                vec![33, 27, 21, 15, 9, 3],
                vec![34, 28, 22, 16, 10, 4],
                vec![35, 29, 23, 17, 11, 5],
                vec![36, 30, 24, 18, 12, 6],
            ],
            matrix
        )
    }
}
