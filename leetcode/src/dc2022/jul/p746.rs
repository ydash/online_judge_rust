#[cfg(test)]
mod tests {
    use super::solution::min_cost_climbing_stairs;

    #[test]
    fn test_case_01() {
        let result = min_cost_climbing_stairs(vec![4, 2]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = min_cost_climbing_stairs(vec![10, 15, 20]);
        assert_eq!(15, result)
    }

    #[test]
    fn test_case_03() {
        let result = min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]);
        assert_eq!(6, result)
    }
}

// 746. Min Cost Climbing Stairs
mod solution {
    #[allow(dead_code)]
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (a, b) = cost
            .iter()
            .skip(2)
            .fold((cost[0], cost[1]), |acc, n| (acc.1, acc.0.min(acc.1) + n));
        a.min(b)
    }
}
