// 1443. Minimum Time to Collect All Apples in a Tree

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for e in edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            adj[a].push(b);
            adj[b].push(a);
        }
        fn dfs(current: usize, parent: usize, has_apple: &[bool], adj: &[Vec<usize>]) -> i32 {
            adj[current]
                .iter()
                .filter(|&&child| parent != child)
                .fold(0, |acc, &child| {
                    let sum_child = dfs(child, current, has_apple, adj);
                    acc + sum_child
                        + if sum_child > 0 || has_apple[child] {
                            2
                        } else {
                            0
                        }
                })
        }
        dfs(0, 0, &has_apple, &adj)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_time(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6],
            ],
            vec![false, false, true, false, true, true, false],
        );
        assert_eq!(8, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_time(
            7,
            vec![
                vec![0, 1],
                vec![2, 0],
                vec![4, 1],
                vec![1, 5],
                vec![3, 2],
                vec![2, 6],
            ],
            vec![false, false, true, false, false, true, false],
        );
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_time(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![4, 5],
                vec![2, 3],
                vec![4, 6],
            ],
            vec![false, false, true, false, false, false, false],
        );
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::min_time(
            4,
            vec![vec![0, 1], vec![1, 2], vec![0, 3]],
            vec![true, true, true, true],
        );
        assert_eq!(6, result)
    }
}
