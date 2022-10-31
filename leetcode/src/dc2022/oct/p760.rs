// 766. Toeplitz Matrix

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        for j in 0..n {
            let v = matrix[0][j];
            let mut x = 0;
            let mut y = j;
            while x < m && y < n {
                if matrix[x][y] != v {
                    return false;
                }
                x += 1;
                y += 1;
            }
        }
        for i in 1..m {
            let v = matrix[i][0];
            let mut x = i;
            let mut y = 0;
            while x < m && y < n {
                if matrix[x][y] != v {
                    return false;
                }
                x += 1;
                y += 1;
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
        let result = Solution::is_toeplitz_matrix(vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![9, 5, 1, 2],
        ]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]);
        assert_eq!(false, result)
    }
}

/*
[
    [36,59,71,15,26,82,87],
    [56,36,59,71,15,26,82],
    [15,0,36,59,71,15,26]
]
*/
