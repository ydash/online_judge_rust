// 26. Remove Duplicates from Sorted Array

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last = 0;
        for i in 0..nums.len() {
            if nums[last] != nums[i] {
                last += 1;
                nums[last] = nums[i]
            }
        }
        last as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let mut nums = vec![1, 1, 2];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(2, result);
        assert_eq!(vec![1, 2], nums.into_iter().take(2).collect::<Vec<_>>())
    }

    #[test]
    fn test_case_02() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(5, result);
        assert_eq!(
            vec![0, 1, 2, 3, 4],
            nums.into_iter().take(5).collect::<Vec<_>>()
        )
    }

    #[test]
    fn test_case_03() {
        let mut nums = vec![0];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(1, result);
        assert_eq!(vec![0], nums)
    }

    #[test]
    fn test_case_04() {
        let mut nums = vec![0, 1, 2, 3, 4];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(5, result);
        assert_eq!(vec![0, 1, 2, 3, 4], nums)
    }
}
