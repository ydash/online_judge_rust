// 834. Sum of Distances in Tree

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut edge = vec![vec![]; n as usize];
        for v in edges.iter() {
            let a = v[0] as usize;
            let b = v[1] as usize;
            edge[a].push(b);
            edge[b].push(a);
        }
        let mut cache: HashMap<(usize, usize), (i32, i32)> = HashMap::new();
        fn dfs(
            current: usize,
            edge: &Vec<Vec<usize>>,
            cache: &mut HashMap<(usize, usize), (i32, i32)>,
            prev: usize,
        ) -> (i32, i32) {
            let mut sum = 0;
            let mut node_count = 1;
            for &next in edge[current].iter() {
                if next != prev {
                    if !cache.contains_key(&(current, next)) {
                        let (dist, count) = dfs(next, edge, cache, current);
                        cache.insert((current, next), (dist + count, count));
                    }
                    cache.get(&(current, next)).map(|(d, c)| {
                        sum += d;
                        node_count += c;
                    });
                }
            }
            (sum, node_count)
        }
        (0..n as usize)
            .map(|i| dfs(i, &edge, &mut cache, i).0)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::sum_of_distances_in_tree(
            6,
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]],
        );
        assert_eq!(vec![8, 12, 6, 10, 10, 10], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::sum_of_distances_in_tree(1, vec![]);
        assert_eq!(vec![0], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::sum_of_distances_in_tree(2, vec![vec![1, 0]]);
        assert_eq!(vec![1, 1], result)
    }
}
