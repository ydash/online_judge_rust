// 658. Find K Closest Elements

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut right = 0;
        while right < arr.len() && arr[right] < x {
            right += 1;
        }
        let mut left = right;
        while right - left < k {
            if left <= 0 {
                right += 1;
            } else if right >= arr.len() {
                left -= 1;
            } else {
                let ad = (x - arr[left - 1]).abs();
                let bd = (x - arr[right]).abs();
                if ad < bd {
                    left -= 1;
                } else if bd < ad {
                    right += 1;
                } else {
                    left -= 1;
                }
            }
        }
        arr[left..right].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3);
        assert_eq!(vec![1, 2, 3, 4], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1);
        assert_eq!(vec![1, 2, 3, 4], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 3, 6);
        assert_eq!(vec![3, 4, 5], result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::find_closest_elements(vec![1], 1, 1);
        assert_eq!(vec![1], result)
    }
}
