// 1323. Maximum 69 Number

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn maximum69_number(num: i32) -> i32 {
        let mut current = 1;
        let mut last_index_of_six = 0;
        while current < num {
            if num / current % 10 == 6 {
                last_index_of_six = current;
            }
            current *= 10;
        }
        num + last_index_of_six * 3
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::maximum69_number(9669);
        assert_eq!(9969, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::maximum69_number(6);
        assert_eq!(9, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::maximum69_number(9);
        assert_eq!(9, result)
    }
}
