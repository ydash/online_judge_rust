// 909. Snakes and Ladders

use std::collections::VecDeque;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn snakes_and_ladders(mut board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let goal = n * n;
        board.reverse();
        for i in (1..n).step_by(2) {
            board[i].reverse()
        }
        let mut seen = vec![vec![false; n]; n];
        let mut queue = VecDeque::new();
        queue.push_back(1);
        let mut step = 0;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();
                if current == goal {
                    return step;
                }
                for next in current + 1..=(current + 6).min(goal) {
                    let (i, j) = ((next - 1) / n, (next - 1) % n);
                    if seen[i][j] {
                        continue;
                    }
                    seen[i][j] = true;
                    if board[i][j] == -1 {
                        queue.push_back(next);
                    } else {
                        queue.push_back(board[i][j] as usize);
                    }
                }
            }
            step += 1;
        }
        -1
    }
}
