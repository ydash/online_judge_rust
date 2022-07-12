// 473. Matchsticks to Square
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        let lim = sum / 4;
        if sum % 4 != 0 || matchsticks.iter().any(|&n| n > lim) {
            false
        } else {
            aux(&matchsticks, 0, 0, 0, 0, sum / 4)
        }
    }
}
fn aux(acc: &[i32], a: i32, b: i32, c: i32, d: i32, lim: i32) -> bool {
    if acc.is_empty() {
        a == b && b == c && c == d
    } else {
        (acc[0] + a <= lim && aux(&acc[1..], a + acc[0], b, c, d, lim))
            || (b + acc[0] <= lim && aux(&acc[1..], a, b + acc[0], c, d, lim))
            || (c + acc[0] <= lim && aux(&acc[1..], a, b, c + acc[0], d, lim))
            || (d + acc[0] <= lim && aux(&acc[1..], a, b, c, d + acc[0], lim))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::makesquare(vec![1, 1, 2, 2, 2]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::makesquare(vec![3, 3, 3, 3, 4]);
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::makesquare(vec![1, 1, 1, 1]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::makesquare(vec![
            100000000, 100000000, 100000000, 100000000, 100000000, 100000000, 100000000, 100000000,
            100000000, 100000000, 100000000, 100000000,
        ]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::makesquare(vec![10, 6, 5, 5, 5, 3, 3, 3, 2, 2, 2, 2]);
        assert_eq!(true, result)
    }
}
