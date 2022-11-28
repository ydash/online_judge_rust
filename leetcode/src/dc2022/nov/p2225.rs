// 2225. Find Players With Zero or One Losses

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let size = matches.iter().fold(0, |acc, v| acc.max(v[0].max(v[1]))) as usize + 1;
        let mut lose = vec![-1; size];
        for v in matches.iter() {
            let winner = v[0] as usize;
            let loser = v[1] as usize;
            if lose[winner] < 0 {
                lose[winner] = 0;
            }
            if lose[loser] < 0 {
                lose[loser] = 1;
            } else {
                lose[loser] += 1;
            }
        }

        let mut result = vec![vec![], vec![]];
        for i in 1..size {
            if lose[i] == 0 {
                result[0].push(i as i32);
            } else if lose[i] == 1 {
                result[1].push(i as i32);
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
        let result = Solution::find_winners(vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ]);
        assert_eq!(vec![vec![1, 2, 10], vec![4, 5, 7, 8]], result);
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]]);
        assert_eq!(vec![vec![1, 2, 5, 6], vec![]], result)
    }
}
