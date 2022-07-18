use std::collections::HashMap;

// 1074. Number of Submatrices That Sum to Target
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn num_submatrix_sum_target(mut matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 0..m {
            for j in 1..n {
                matrix[i][j] += matrix[i][j - 1]
            }
        }
        let mut result = 0;
        let mut counter = HashMap::new();
        for y1 in 0..n {
            for y2 in y1..n {
                counter.clear();
                counter.insert(0, 1);
                let mut current = 0;
                for x in 0..m {
                    current += matrix[x][y2] - if y1 > 0 { matrix[x][y1 - 1] } else { 0 };
                    result += counter.get(&(current - target)).unwrap_or(&0);
                    *counter.entry(current).or_insert(0) += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::num_submatrix_sum_target(
            vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
            0,
        );
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::num_submatrix_sum_target(vec![vec![1, -1], vec![-1, 1]], 0);
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::num_submatrix_sum_target(vec![vec![42]], 0);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::num_submatrix_sum_target(vec![vec![42]], 42);
        assert_eq!(1, result)
    }
}
