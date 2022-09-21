// 985. Sum of Even Numbers After Queries

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let mut sum: i32 = nums.iter().filter(|&v| v % 2 == 0).sum();
        for q in queries.iter() {
            let v = q[0];
            let i = q[1] as usize;
            if nums[i] % 2 == 0 {
                sum -= nums[i];
            }
            nums[i] += v;
            if nums[i] % 2 == 0 {
                sum += nums[i];
            }
            result.push(sum);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::sum_even_after_queries(
            vec![1, 2, 3, 4],
            vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]],
        );
        assert_eq!(vec![8, 6, 2, 4], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::sum_even_after_queries(vec![1], vec![vec![4, 0]]);
        assert_eq!(vec![0], result)
    }
}
