// 240. Search a 2D Matrix II
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut col, mut row) = (0, 0);

        while col < m && row < n {
            match matrix[col][n - row - 1].cmp(&target) {
                std::cmp::Ordering::Less => col += 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => row += 1,
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
        let result = Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30],
            ],
            5,
        );
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30],
            ],
            20,
        );
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::search_matrix(vec![vec![1]], 1);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::search_matrix(vec![vec![1]], 42);
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::search_matrix(vec![vec![5], vec![6]], 6);
        assert_eq!(true, result)
    }
}
