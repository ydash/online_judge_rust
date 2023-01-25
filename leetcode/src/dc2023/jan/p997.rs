// 997. Find the Town Judge

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut trust_nobody = (1..=n).collect::<HashSet<_>>();
        let mut count = vec![0; n as usize + 1];
        for v in trust {
            trust_nobody.remove(&v[0]);
            count[v[1] as usize] += 1;
        }

        for &i in trust_nobody.iter() {
            if count[i as usize] == n - 1 {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::find_judge(2, vec![vec![1, 2]]);
        assert_eq!(2, result)
    }
}
