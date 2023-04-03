// 881. Boats to Save People

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();
        let mut left = 0;
        let mut result = 0;
        while left < people.len() {
            let right = people.len() - 1;
            if left < right && people[right] + people[left] <= limit {
                left += 1;
            }
            people.pop();
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::num_rescue_boats(vec![1, 2], 3);
        assert_eq!(1, result);
    }

    #[test]
    fn test_case_02() {
        let result = Solution::num_rescue_boats(vec![3, 2, 2, 1], 3);
        assert_eq!(3, result);
    }

    #[test]
    fn test_case_03() {
        let result = Solution::num_rescue_boats(vec![3, 5, 3, 4], 5);
        assert_eq!(4, result);
    }

    #[test]
    fn test_case_04() {
        let result = Solution::num_rescue_boats(
            vec![
                2, 49, 10, 7, 11, 41, 47, 2, 22, 6, 13, 12, 33, 18, 10, 26, 2, 6, 50, 10,
            ],
            50,
        );
        assert_eq!(11, result);
    }
}
