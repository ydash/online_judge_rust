// 886. Possible Bipartition

use std::collections::VecDeque;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut edge = vec![vec![]; n];
        for v in dislikes.iter() {
            let a = v[0] as usize - 1;
            let b = v[1] as usize - 1;
            edge[a].push(b);
            edge[b].push(a);
        }
        let mut group = vec![0; n];
        for i in 0..n {
            if group[i] != 0 {
                continue;
            }
            let mut group_no = 1;
            let mut queue = VecDeque::from([i]);
            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    let j = queue.pop_front().unwrap();
                    if group[j] == 0 {
                        group[j] = group_no;
                        queue.extend(edge[j].iter());
                    } else if group_no != group[j] {
                        return false;
                    }
                }
                group_no *= -1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::possible_bipartition(
            5,
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]],
        );
        assert_eq!(false, result)
    }
}
