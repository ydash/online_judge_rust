// 1338. Reduce Array Size to The Half

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut freq = HashMap::new();
        for &n in arr.iter() {
            *freq.entry(n).or_insert(0) += 1;
        }
        let mut count = vec![0; arr.len() + 1];
        for (_, &v) in freq.iter() {
            count[v] += 1;
        }
        let mut result = 0;
        let mut removed = 0;
        let threashold = arr.len() / 2;
        let mut i = count.len() - 1;
        while removed < threashold {
            if count[i] > 0 {
                removed += i;
                count[i] -= 1;
                result += 1;
            } else {
                i -= 1;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_set_size(vec![7, 7, 7, 7, 7]);
        assert_eq!(1, result)
    }
}
