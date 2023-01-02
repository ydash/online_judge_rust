// 1834. Single-Threaded CPU

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tasks = tasks
            .into_iter()
            .enumerate()
            .map(|(i, t)| (i, t[0], t[1]))
            .collect::<Vec<_>>();
        tasks.sort_by(|(_, enq_time_0, _), (_, enq_time_1, _)| enq_time_0.cmp(&enq_time_1));
        let mut available = BinaryHeap::new();
        let mut current_time = 0;
        let mut i = 0;
        let mut result = vec![];
        while i < tasks.len() || !available.is_empty() {
            while tasks.get(i).map_or(false, |&(_idx, enq_time, _proc_time)| {
                enq_time <= current_time
            }) {
                available.push(Reverse((tasks[i].2, tasks[i].0)));
                i += 1;
            }
            if let Some(Reverse((proc_time, idx))) = available.pop() {
                result.push(idx as i32);
                current_time += proc_time;
            } else {
                current_time = tasks[i].1;
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
        let result = Solution::get_order(vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]]);
        assert_eq!(vec![0, 2, 3, 1], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::get_order(vec![
            vec![7, 10],
            vec![7, 12],
            vec![7, 5],
            vec![7, 4],
            vec![7, 2],
        ]);
        assert_eq!(vec![4, 3, 2, 0, 1], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::get_order(vec![
            vec![9, 10],
            vec![7, 12],
            vec![7, 5],
            vec![7, 4],
            vec![7, 2],
        ]);
        assert_eq!(vec![4, 3, 2, 0, 1], result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::get_order(vec![vec![0, 10], vec![1, 10], vec![2, 10], vec![3, 10]]);
        assert_eq!(vec![0, 1, 2, 3], result)
    }
}
