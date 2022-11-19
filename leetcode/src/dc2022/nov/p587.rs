// 587. Erect the Fence

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        trees.sort_by(|a, b| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
            other => other,
        });
        let mut result = HashSet::new();
        let mut stack: Vec<Vec<i32>> = vec![];
        for t in trees.iter() {
            while stack.len() > 1
                && Self::slope(&stack[stack.len() - 2], &stack[stack.len() - 1])
                    < Self::slope(&stack[stack.len() - 2], t)
            {
                stack.pop();
            }
            stack.push(t.to_vec());
        }
        result.extend(stack.drain(..).collect::<Vec<_>>());
        for t in trees.iter() {
            while stack.len() > 1
                && Self::slope(&stack[stack.len() - 2], &stack[stack.len() - 1])
                    > Self::slope(&stack[stack.len() - 2], t)
            {
                stack.pop();
            }
            stack.push(t.to_vec());
        }
        if stack.len() > 2 {
            result.extend(stack.drain(1..stack.len() - 1).collect::<Vec<_>>());
        }
        result.into_iter().collect()
    }

    fn slope(p1: &Vec<i32>, p2: &Vec<i32>) -> f64 {
        let (x1, y1) = (p1[0], p1[1]);
        let (x2, y2) = (p2[0], p2[1]);
        (y2 - y1) as f64 / (x2 - x1) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let mut result = Solution::outer_trees(vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2],
        ]);
        result.sort_by(|a, b| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
            other => other,
        });
        assert_eq!(
            vec![vec![1, 1], vec![2, 0], vec![2, 4], vec![3, 3], vec![4, 2]],
            result
        )
    }

    #[test]
    fn test_case_02() {
        let mut result = Solution::outer_trees(vec![vec![1, 2], vec![2, 2], vec![4, 2]]);
        result.sort_by(|a, b| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
            other => other,
        });
        assert_eq!(vec![vec![1, 2], vec![2, 2], vec![4, 2]], result)
    }
}
