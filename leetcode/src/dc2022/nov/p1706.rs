// 1706. Where Will the Ball Fall

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();

        let mut result = vec![];
        'outer: for mut j in 0..n {
            for i in 0..m {
                let adj = j as i32 + grid[i][j];
                if adj < 0 || adj as usize >= n || grid[i][adj as usize] != grid[i][j] {
                    result.push(-1);
                    continue 'outer;
                }
                j = adj as usize;
            }
            result.push(j as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::find_ball(vec![
            vec![1, 1, 1, -1, -1],
            vec![1, 1, 1, -1, -1],
            vec![-1, -1, -1, 1, 1],
            vec![1, 1, 1, 1, -1],
            vec![-1, -1, -1, -1, -1],
        ]);
        assert_eq!(vec![1, -1, -1, -1, -1], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_ball(vec![vec![-1]]);
        assert_eq!(vec![-1], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::find_ball(vec![
            vec![1, 1, 1, 1, 1, 1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![1, 1, 1, 1, 1, 1],
            vec![-1, -1, -1, -1, -1, -1],
        ]);
        assert_eq!(vec![0, 1, 2, 3, 4, -1], result)
    }
}
