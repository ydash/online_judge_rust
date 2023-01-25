// 2359. Find Closest Node to Given Two Nodes

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        fn dfs(current: i32, dist: i32, edges: &[i32], visit: &mut [i32]) {
            if current != -1 && visit[current as usize] < 0 {
                visit[current as usize] = dist;
                dfs(edges[current as usize], dist + 1, edges, visit)
            }
        }
        let n = edges.len();
        let mut visit1 = vec![-1; n];
        let mut visit2 = vec![-1; n];
        dfs(node1, 0, &edges, &mut visit1);
        dfs(node2, 0, &edges, &mut visit2);
        let mut min_dist = i32::MAX;
        let mut result = -1;
        for i in 0..n {
            if visit1[i].min(visit2[i]) >= 0 && visit1[i].max(visit2[i]) < min_dist {
                min_dist = visit1[i].max(visit2[i]);
                result = i as i32;
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
        let result = Solution::closest_meeting_node(vec![2, 2, 3, -1], 0, 1);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::closest_meeting_node(vec![1, 2, -1], 0, 2);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::closest_meeting_node(vec![-1, -1], 0, 1);
        assert_eq!(-1, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::closest_meeting_node(vec![1, 0, 3, 2], 0, 2);
        assert_eq!(-1, result)
    }
}
