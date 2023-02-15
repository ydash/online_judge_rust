// 989. Add to Array-Form of Integer

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut carry_up = 0;
        while let Some(n) = num.pop() {
            let x = n + (k % 10) + carry_up;
            result.push(x % 10);
            carry_up = x / 10;
            k /= 10;
        }
        while k > 0 {
            let x = (k % 10) + carry_up;
            result.push(x % 10);
            carry_up = x / 10;
            k /= 10;
        }
        if carry_up > 0 {
            result.push(carry_up);
        }
        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::add_to_array_form(vec![1, 2, 0, 0], 34);
        assert_eq!(vec![1, 2, 3, 4], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::add_to_array_form(vec![2, 7, 4], 181);
        assert_eq!(vec![4, 5, 5], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::add_to_array_form(vec![2, 1, 5], 806);
        assert_eq!(vec![1, 0, 2, 1], result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::add_to_array_form(vec![4, 5, 5], 9783);
        assert_eq!(vec![1, 0, 2, 3, 8], result)
    }
}
