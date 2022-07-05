#[cfg(test)]
mod tests {
    use super::solution::longest_consecutive;

    #[test]
    fn test_case_01() {
        let result = longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
        assert_eq!(9, result)
    }

    #[test]
    fn test_case_03() {
        let result = longest_consecutive(vec![42]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_04() {
        let result = longest_consecutive(vec![5, 6, 7, 3, 4]);
        assert_eq!(5, result)
    }
}

// 128. Longest Consecutive Sequence
mod solution {
    use std::collections::HashSet;

    #[allow(dead_code)]
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut result = 0;

        for &n in set.iter() {
            if !set.contains(&(n - 1)) {
                let mut i = n;
                while set.contains(&i) {
                    i += 1;
                }
                result = result.max(i - n);
            }
        }

        result
    }
}
