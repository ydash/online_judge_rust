// 1926. Nearest Exit from Entrance in Maze

use std::collections::VecDeque;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let m = maze.len() as i32;
        let n = maze[0].len() as i32;
        let mut queue = VecDeque::new();
        queue.push_back((entrance[0], entrance[1]));
        maze[entrance[0] as usize][entrance[1] as usize] = '+';
        let mut step = 1;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let (i, j) = queue.pop_front().unwrap();
                for (x, y) in directions
                    .iter()
                    .map(|&(di, dj)| (i + di, j + dj))
                    .filter(|&(x, y)| x >= 0 && x < m && y >= 0 && y < n)
                {
                    if maze[x as usize][y as usize] != '+' {
                        if x == 0 || x == m - 1 || y == 0 || y == n - 1 {
                            return step;
                        }
                        queue.push_back((x as i32, y as i32));
                        maze[x as usize][y as usize] = '+';
                    }
                }
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
        let result = Solution::nearest_exit(
            vec![
                vec!['+', '+', '.', '+'],
                vec!['.', '.', '.', '+'],
                vec!['+', '+', '+', '.'],
            ],
            vec![1, 2],
        );
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::nearest_exit(
            vec![
                vec!['+', '+', '+'],
                vec!['.', '.', '.'],
                vec!['+', '+', '+'],
            ],
            vec![1, 0],
        );
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::nearest_exit(vec![vec!['.', '+']], vec![0, 0]);
        assert_eq!(-1, result)
    }
}
