// 1466. Reorder Routes to Make All Paths Lead to the City Zero

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut from_to = vec![vec![]; n];
        let mut to_from = vec![vec![]; n];
        for v in connections {
            let a = v[0] as usize;
            let b = v[1] as usize;
            from_to[a].push(b);
            to_from[b].push(a);
        }
        let mut stack = vec![(0, 0)];
        let mut result = 0;
        while let Some((current, prev)) = stack.pop() {
            for &next in to_from[current].iter() {
                if prev != next {
                    stack.push((next, current));
                }
            }
            for &next in from_to[current].iter() {
                if prev != next {
                    stack.push((next, current));
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
        let result = Solution::min_reorder(
            6,
            vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]],
        );
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_reorder(3, vec![vec![1, 0], vec![2, 0]]);
        assert_eq!(0, result)
    }
}
