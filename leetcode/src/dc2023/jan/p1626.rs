// 1626. Best Team With No Conflicts

use std::collections::{HashMap, HashSet};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut score_count = HashMap::new();
        for (&s, &a) in scores.iter().zip(ages.iter()) {
            *score_count
                .entry(a)
                .or_insert(HashMap::new())
                .entry(s)
                .or_insert(0) += 1
        }
        let mut score_count = score_count.into_iter().collect::<Vec<_>>();
        score_count.sort_by(|a, b| a.0.cmp(&b.0));
        let mut unique_scores = scores
            .iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        unique_scores.sort();
        let mut dp = vec![0; unique_scores.len() + 1];
        for (_, count) in score_count {
            for (i, &score) in unique_scores.iter().enumerate() {
                let d = *count.get(&score).unwrap_or(&0) * score;
                dp[i + 1] = dp[i].max(dp[i + 1]) + d;
            }
        }
        dp[unique_scores.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::best_team_score(vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5]);
        assert_eq!(34, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::best_team_score(vec![4, 5, 6, 5], vec![2, 1, 2, 1]);
        assert_eq!(16, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::best_team_score(vec![1, 2, 3, 5], vec![8, 9, 10, 1]);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::best_team_score(vec![9, 2, 8, 8, 2], vec![4, 1, 3, 3, 5]);
        assert_eq!(27, result)
    }
}
