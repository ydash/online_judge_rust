// 823. Binary Trees With Factors

use std::collections::{HashMap, HashSet};

const MODULUS: i64 = 1000000007;
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        arr.sort();
        let mut cache: HashMap<i32, i64> = HashMap::new();
        let set: HashSet<i32> = arr.iter().cloned().collect();
        let mut result = 0;
        for &n in arr.iter() {
            let lim = f64::from(n).sqrt() as i32;
            let mut i = 0;
            while i < arr.len() && arr[i] <= lim {
                let a = arr[i];
                let b = n / a;
                if a * b == n && set.contains(&b) {
                    let ac = *cache.entry(a).or_insert(1);
                    let bc = *cache.entry(b).or_insert(1);
                    let counter = cache.entry(n).or_insert(1);
                    if a == b {
                        *counter += ac * bc;
                    } else {
                        *counter += 2 * ac * bc;
                    }
                    *counter %= MODULUS;
                }
                i += 1;
            }
            result += *cache.entry(n).or_insert(1);
            result %= MODULUS;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::num_factored_binary_trees(vec![2, 4]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::num_factored_binary_trees(vec![2, 4, 5, 10]);
        assert_eq!(7, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::num_factored_binary_trees(vec![100, 10, 2, 5]);
        assert_eq!(15, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::num_factored_binary_trees(vec![18, 3, 6, 2]);
        assert_eq!(12, result)
    }
}
