// 2477. Minimum Fuel Cost to Report to the Capital

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let n = roads.len() + 1;
        let mut adj = vec![vec![]; n];
        for road in roads {
            let a = road[0] as usize;
            let b = road[1] as usize;
            adj[a].push(b);
            adj[b].push(a);
        }
        fn dfs(
            current: usize,
            prev: usize,
            adj: &[Vec<usize>],
            seats: i32,
            result: &mut i64,
        ) -> i32 {
            let mut people = 1;
            for &next in adj[current].iter() {
                if next == prev {
                    continue;
                }
                people += dfs(next, current, adj, seats, result);
            }
            if current != 0 {
                *result += ((people - 1) / seats + 1) as i64
            }
            people
        }
        let mut result = 0;
        dfs(0, 0, &adj, seats, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::minimum_fuel_cost(vec![vec![0, 1], vec![0, 2], vec![0, 3]], 5);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::minimum_fuel_cost(
            vec![
                vec![3, 1],
                vec![3, 2],
                vec![1, 0],
                vec![0, 4],
                vec![0, 5],
                vec![4, 6],
            ],
            2,
        );
        assert_eq!(7, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::minimum_fuel_cost(
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]],
            2,
        );
        assert_eq!(8, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::minimum_fuel_cost(vec![], 1);
        assert_eq!(0, result)
    }
}
