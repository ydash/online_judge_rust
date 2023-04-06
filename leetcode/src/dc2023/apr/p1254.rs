// 1254. Number of Closed Islands

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(start: (usize, usize), lim_pos: (usize, usize), grid: &mut [Vec<i32>]) -> bool {
            let mut is_closed_island = true;
            let mut stack = vec![(start)];
            while let Some((x, y)) = stack.pop() {
                if x == 0 || y == 0 || x == lim_pos.0 || y == lim_pos.1 {
                    is_closed_island = false;
                }
                if x > 0 && grid[x - 1][y] == 0 {
                    grid[x - 1][y] = 1;
                    stack.push((x - 1, y));
                }
                if x < lim_pos.0 && grid[x + 1][y] == 0 {
                    grid[x + 1][y] = 1;
                    stack.push((x + 1, y));
                }
                if y > 0 && grid[x][y - 1] == 0 {
                    grid[x][y - 1] = 1;
                    stack.push((x, y - 1));
                }
                if y < lim_pos.1 && grid[x][y + 1] == 0 {
                    grid[x][y + 1] = 1;
                    stack.push((x, y + 1));
                }
            }
            is_closed_island
        }
        let mut result = 0;
        let m = grid.len() - 1;
        let n = grid[0].len() - 1;
        for i in 1..m {
            for j in 1..n {
                if grid[i][j] == 0 && dfs((i, j), (m, n), &mut grid) {
                    result += 1;
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
        let result = Solution::closed_island(vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::closed_island(vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0],
        ]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::closed_island(vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ]);
        assert_eq!(2, result)
    }
}
