// 2007. Find Original Array From Doubled Array

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut count = vec![0; 100000 + 1];
        for &n in changed.iter() {
            count[n as usize] += 1;
        }
        for i in 0..=50000 {
            while count[i] > 0 {
                result.push(i as i32);
                count[i] -= 1;
                count[2 * i] -= 1;
            }
        }
        if count.iter().all(|&c| c == 0) {
            result
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::Solution;
    #[test]

    fn test_case_01() {
        let result = Solution::find_original_array(vec![1, 3, 4, 2, 6, 8]);
        assert_eq!(vec![1, 3, 4], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_original_array(vec![6, 3, 0, 1]);
        assert_eq!(Vec::<i32>::new(), result)
    }
}
