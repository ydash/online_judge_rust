#[cfg(test)]
mod tests {
    use super::solution::min_cost;

    #[test]
    fn test_case_01() {
        let result = min_cost(
            vec![0, 0, 0, 0, 0],
            vec![
                vec![1, 10],
                vec![10, 1],
                vec![10, 1],
                vec![1, 10],
                vec![5, 1],
            ],
            5,
            2,
            3,
        );
        assert_eq!(9, result)
    }

    #[test]
    fn test_case_02() {
        let result = min_cost(
            vec![0, 2, 1, 2, 0],
            vec![
                vec![1, 10],
                vec![10, 1],
                vec![10, 1],
                vec![1, 10],
                vec![5, 1],
            ],
            5,
            2,
            3,
        );
        assert_eq!(11, result)
    }

    #[test]
    fn test_case_03() {
        let result = min_cost(
            vec![3, 1, 2, 3],
            vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
            4,
            3,
            3,
        );
        assert_eq!(-1, result)
    }

    #[test]
    fn test_case_04() {
        let result = min_cost(vec![0], vec![vec![4, 2, 5, 1, 6]], 1, 5, 1);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_05() {
        let result = min_cost(vec![1], vec![vec![4, 2, 5, 1, 6]], 1, 5, 1);
        assert_eq!(0, result)
    }
}

// 1473. Paint House III
mod solution {
    #[allow(dead_code)]
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let (m, n, target) = (m as usize, n as usize, target as usize);
        let mut dp = vec![vec![vec![None; target]; n]; m];

        if houses[0] == 0 {
            for j in 0..n {
                dp[0][j][0] = Some(cost[0][j]);
            }
        } else {
            dp[0][(houses[0] - 1) as usize][0] = Some(0);
        }
        for i in 1..m {
            for prev_color in 0..n {
                if houses[i] == 0 {
                    for current_color in 0..n {
                        for k in 0..target {
                            let neighbors = k + if prev_color == current_color { 0 } else { 1 };
                            if neighbors < target {
                                dp[i - 1][prev_color][k]
                                    .map(|v| {
                                        (v + if houses[i] == 0 {
                                            cost[i][current_color]
                                        } else {
                                            0
                                        })
                                        .min(dp[i][current_color][neighbors].unwrap_or(i32::MAX))
                                    })
                                    .map(|v| dp[i][current_color][neighbors] = Some(v));
                            }
                        }
                    }
                } else {
                    for k in 0..target {
                        let current_color = (houses[i] - 1) as usize;
                        let neighbors = k + if prev_color == current_color { 0 } else { 1 };
                        if neighbors < target {
                            dp[i - 1][prev_color][k]
                                .map(|v| v.min(dp[i][current_color][neighbors].unwrap_or(i32::MAX)))
                                .map(|v| dp[i][current_color][neighbors] = Some(v));
                        }
                    }
                }
            }
        }
        let mut result = None;
        for j in 0..n {
            if dp[m - 1][j][target - 1].is_some() {
                dp[m - 1][j][target - 1].map(|v| result = Some(result.unwrap_or(i32::MAX).min(v)));
            }
        }
        result.unwrap_or(-1)
    }
}
