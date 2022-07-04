mod tests {
    use super::solution::*;

    #[test]
    fn test_case_01() {
        let result = candy(vec![1, 0, 2]);
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_02() {
        let result = candy(vec![1, 2, 2]);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_03() {
        let result = candy(vec![100]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_04() {
        let result = candy(vec![1, 3, 4, 5, 2]);
        assert_eq!(11, result)
    }
}

// 135. Candy
mod solution {
    use std::cmp::Ordering;
    #[allow(dead_code)]
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; ratings.len()];
        let mut indices = (0..ratings.len()).collect::<Vec<_>>();
        indices.sort_by(|&i, &j| ratings[i].cmp(&ratings[j]));

        for i in indices.into_iter() {
            let left = ratings
                .get(if i == 0 { 0 } else { i - 1 })
                .unwrap_or(&i32::MAX);
            let right = ratings.get(i + 1).unwrap_or(&i32::MAX);
            match (ratings[i].cmp(left), ratings[i].cmp(right)) {
                (Ordering::Greater, Ordering::Greater) => dp[i] = dp[i - 1].max(dp[i + 1]) + 1,
                (Ordering::Greater, _) => dp[i] = dp[i - 1] + 1,
                (_, Ordering::Greater) => dp[i] = dp[i + 1] + 1,
                _ => dp[i] = 1,
            }
        }

        dp.iter().sum()
    }
}
