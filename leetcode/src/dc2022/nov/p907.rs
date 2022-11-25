// 907. Sum of Subarray Minimums

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut stack_left = vec![];
        let mut stack_right = vec![];
        let mut left = (1..=arr.len()).collect::<Vec<_>>();
        let mut right = (1..=arr.len()).rev().collect::<Vec<_>>();
        for i in 0..arr.len() {
            while stack_left.last().map_or(false, |&x| arr[i] < arr[x]) {
                stack_left.pop();
            }
            left[i] = if let Some(&x) = stack_left.last() {
                i - x
            } else {
                i + 1
            };
            stack_left.push(i);

            while stack_right.last().map_or(false, |&x| arr[i] < arr[x]) {
                stack_right.pop().map(|x| right[x] = i - x);
            }
            stack_right.push(i);
        }
        let mut result = 0_i64;
        for i in 0..arr.len() {
            result += arr[i] as i64 * left[i] as i64 * right[i] as i64;
            result %= 1000000007;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::sum_subarray_mins(vec![3, 1, 2, 4]);
        assert_eq!(17, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]);
        assert_eq!(444, result)
    }
}
