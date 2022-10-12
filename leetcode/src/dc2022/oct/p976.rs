// 976. Largest Perimeter Triangle

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for i in (2..nums.len()).rev() {
            if nums[i] < nums[i - 1] + nums[i - 2] {
                return nums[i] + nums[i - 1] + nums[i - 2];
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::largest_perimeter(vec![2, 1, 2]);
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::largest_perimeter(vec![1, 2, 1]);
        assert_eq!(0, result)
    }
}
