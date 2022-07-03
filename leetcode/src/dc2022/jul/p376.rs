#[cfg(test)]
mod tests {
    use super::solution::wiggle_max_length;

    #[test]
    fn test_case_01() {
        let result = wiggle_max_length(vec![1, 7, 4, 9, 2, 5]);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_02() {
        let result = wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]);
        assert_eq!(7, result)
    }

    #[test]
    fn test_case_03() {
        let result = wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_04() {
        let result = wiggle_max_length(vec![5, 2, 9, 4, 7, 1]);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_05() {
        let result = wiggle_max_length(vec![0]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_06() {
        let result = wiggle_max_length(vec![3, 0]);
        assert_eq!(2, result)
    }
}

// 376. Wiggle Subsequence
mod solution {
    use std::cmp::Ordering;

    #[allow(dead_code)]
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let (mut up, mut down) = (1, 1);
        for i in 1..nums.len() {
            match nums[i].cmp(&nums[i - 1]) {
                Ordering::Less => down = up + 1,
                Ordering::Greater => up = down + 1,
                Ordering::Equal => (),
            }
        }
        up.max(down)
    }
}
