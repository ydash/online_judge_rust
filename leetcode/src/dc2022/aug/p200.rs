// 200. Number of Islands

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn dfs(i: usize, j: usize, m: usize, n: usize, grid: &mut Vec<Vec<char>>) {
            if grid[i][j] == '0' {
                return;
            }
            grid[i][j] = '0';
            if i > 0 {
                dfs(i - 1, j, m, n, grid)
            }
            if j > 0 {
                dfs(i, j - 1, m, n, grid);
            }
            if i < m - 1 {
                dfs(i + 1, j, m, n, grid)
            }
            if j < n - 1 {
                dfs(i, j + 1, m, n, grid)
            }
        }
        let m = grid.len();
        let n = grid[0].len();
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    count += 1;
                    dfs(i, j, m, n, &mut grid);
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let result = Solution::num_islands(grid);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let result = Solution::num_islands(grid);
        assert_eq!(3, result)
    }
}
