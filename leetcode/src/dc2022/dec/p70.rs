// 70. Climbing Stairs

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 1;
        let mut b = 1;
        for _ in 2..=n {
            let tmp = a + b;
            a = b;
            b = tmp;
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::climb_stairs(2);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::climb_stairs(3);
        assert_eq!(3, result)
    }
}
