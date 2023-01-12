// 1519. Number of Nodes in the Sub-Tree With the Same Label

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for e in edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut result = vec![0; n];
        fn dfs(
            current: usize,
            parent: usize,
            adj: &[Vec<usize>],
            labels: &[u8],
            result: &mut [i32],
        ) -> [i32; 26] {
            let mut count = [0; 26];
            let current_label = usize::from(labels[current] - b'a');
            count[current_label] += 1;
            for &child in adj[current].iter().filter(|&&a| a != parent) {
                dfs(child, current, adj, labels, result)
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, c)| count[i] += c);
            }
            result[current] += count[current_label];
            count
        }
        dfs(0, 0, &adj, labels.as_bytes(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::count_sub_trees(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6],
            ],
            "abaedcd".to_owned(),
        );
        assert_eq!(vec![2, 1, 1, 1, 1, 1, 1], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::count_sub_trees(
            4,
            vec![vec![0, 1], vec![1, 2], vec![0, 3]],
            "bbbb".to_owned(),
        );
        assert_eq!(vec![4, 2, 1, 1], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::count_sub_trees(
            5,
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![0, 4]],
            "aabab".to_owned(),
        );
        assert_eq!(vec![3, 2, 1, 1, 1], result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::count_sub_trees(1, vec![], "a".to_owned());
        assert_eq!(vec![1], result)
    }
}
