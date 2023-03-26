// 2360. Longest Cycle in a Graph

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn longest_cycle(mut edges: Vec<i32>) -> i32 {
        fn dfs(start: usize, edges: &mut [i32], seen: &mut [(i32, usize)]) -> i32 {
            let mut current = start;
            let mut step = 1;
            seen[current] = (step, start);
            while edges[current] >= 0 {
                step += 1;
                let next = edges[current] as usize;
                edges[current] = -1;
                current = next;
                if seen[current].0 > 0 {
                    return if seen[current].1 == start {
                        step - seen[current].0
                    } else {
                        -1
                    };
                }
                seen[current] = (step, start);
            }
            -1
        }
        let n = edges.len();
        let mut result = -1;
        let mut seen = vec![(0, 0); n];
        for i in 0..n {
            result = result.max(dfs(i, &mut edges, &mut seen));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::longest_cycle(vec![3, 3, 4, 2, 3]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::longest_cycle(vec![2, -1, 3, 1]);
        assert_eq!(-1, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::longest_cycle(vec![3, 4, 0, 2, -1, 2]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::longest_cycle(vec![-1, 4, -1, 2, 0, 4]);
        assert_eq!(-1, result)
    }
}
