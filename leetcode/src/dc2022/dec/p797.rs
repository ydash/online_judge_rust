// 797. All Paths From Source to Target

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let end = graph.len() - 1;
        let mut result = vec![];
        let mut stack = vec![(0, vec![0])];
        while let Some((i, path)) = stack.pop() {
            if i == end {
                result.push(path);
            } else {
                for &adj in graph[i].iter() {
                    let mut p = path.clone();
                    p.push(adj);
                    stack.push((adj as usize, p));
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
        let mut result =
            Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]);
        result.sort();
        assert_eq!(vec![vec![0, 1, 3], vec![0, 2, 3]], result)
    }

    #[test]
    fn test_case_02() {
        let mut result = Solution::all_paths_source_target(vec![
            vec![4, 3, 1],
            vec![3, 2, 4],
            vec![3],
            vec![4],
            vec![],
        ]);
        result.sort();
        assert_eq!(
            vec![
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 4],
                vec![0, 3, 4],
                vec![0, 4],
            ],
            result
        )
    }
}
