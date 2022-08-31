// 417. Pacific Atlantic Water Flow

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(heights: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, height: i32, x: i32, y: i32) {
            let m = heights.len() as i32;
            let n = heights[0].len() as i32;
            if x < 0
                || y < 0
                || x >= m
                || y >= n
                || visited[x as usize][y as usize]
                || heights[x as usize][y as usize] < height
            {
                return;
            }
            visited[x as usize][y as usize] = true;
            for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                dfs(
                    heights,
                    visited,
                    heights[x as usize][y as usize],
                    x + dx,
                    y + dy,
                )
            }
        }
        let m = heights.len();
        let n = heights[0].len();
        let mut pacific = vec![vec![false; n]; m];
        let mut atlantic = vec![vec![false; n]; m];
        for i in 0..m {
            dfs(&heights, &mut pacific, i32::MIN, i as i32, 0);
            dfs(&heights, &mut atlantic, i32::MIN, i as i32, n as i32 - 1);
        }
        for j in 0..n {
            dfs(&heights, &mut pacific, i32::MIN, 0, j as i32);
            dfs(&heights, &mut atlantic, i32::MIN, m as i32 - 1, j as i32);
        }
        let mut result = vec![];
        for i in 0..m {
            for j in 0..n {
                if pacific[i][j] && atlantic[i][j] {
                    result.push(vec![i as i32, j as i32]);
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
        let result = Solution::pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ]);
        assert_eq!(
            vec![
                vec![0, 4],
                vec![1, 3],
                vec![1, 4],
                vec![2, 2],
                vec![3, 0],
                vec![3, 1],
                vec![4, 0]
            ],
            result
        )
    }

    #[test]
    fn test_case_02() {
        let result = Solution::pacific_atlantic(vec![vec![1]]);
        assert_eq!(vec![vec![0, 0]], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::pacific_atlantic(vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]);
        assert_eq!(
            vec![
                vec![0, 2],
                vec![1, 0],
                vec![1, 1],
                vec![1, 2],
                vec![2, 0],
                vec![2, 1],
                vec![2, 2]
            ],
            result
        )
    }
}
