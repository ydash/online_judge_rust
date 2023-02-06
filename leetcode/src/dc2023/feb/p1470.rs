// 1470. Shuffle the Array

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result = vec![];
        for (&x, &y) in nums.iter().zip(nums.iter().skip(n as usize)) {
            result.push(x);
            result.push(y);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::shuffle(vec![1, 2], 1);
        assert_eq!(vec![1, 2], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3);
        assert_eq!(vec![2, 3, 5, 4, 1, 7], result)
    }
}
