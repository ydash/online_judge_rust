// 1162. As Far from Land as Possible

use std::collections::VecDeque;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut q = VecDeque::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    q.push_back((i, j));
                }
            }
        }
        if q.len() == n * n {
            return -1;
        }
        let mut result = -1;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let (i, j) = q.pop_front().unwrap();
                if i > 0 && grid[i - 1][j] == 0 {
                    grid[i - 1][j] = 1;
                    q.push_back((i - 1, j));
                }
                if i < n - 1 && grid[i + 1][j] == 0 {
                    grid[i + 1][j] = 1;
                    q.push_back((i + 1, j));
                }
                if j > 0 && grid[i][j - 1] == 0 {
                    grid[i][j - 1] = 1;
                    q.push_back((i, j - 1));
                }
                if j < n - 1 && grid[i][j + 1] == 0 {
                    grid[i][j + 1] = 1;
                    q.push_back((i, j + 1));
                }
            }
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::max_distance(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::max_distance(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::max_distance(vec![vec![0]]);
        assert_eq!(-1, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::max_distance(vec![vec![1]]);
        assert_eq!(-1, result)
    }
}
