// 1293. Shortest Path in a Grid with Obstacles Elimination

use std::collections::VecDeque;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let direction = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let m = grid.len();
        let n = grid[0].len();
        let mut rest_k = vec![vec![-1; n]; m];
        let mut q = VecDeque::new();
        q.push_back((0, 0, k));
        rest_k[0][0] = k;
        let mut step = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let (i, j, k) = q.pop_front().unwrap();
                if i == m - 1 && j == n - 1 {
                    return step;
                }
                direction
                    .iter()
                    .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                    .filter(|&(x, y)| x >= 0 && y >= 0 && (x as usize) < m && (y as usize) < n)
                    .for_each(|(x, y)| {
                        let x = x as usize;
                        let y = y as usize;
                        if rest_k[x][y] < k - grid[x][y] {
                            rest_k[x][y] = k - grid[x][y];
                            q.push_back((x, y, k - grid[x][y]));
                        }
                    });
            }
            step += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::shortest_path(
            vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![0, 0, 0],
                vec![0, 1, 1],
                vec![0, 0, 0],
            ],
            1,
        );
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::shortest_path(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1);
        assert_eq!(-1, result)
    }
}
