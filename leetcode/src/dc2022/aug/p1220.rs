// 1220. Count Vowels Permutation

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let modulus = 1000000007;
        let mut patterns: Vec<u32> = vec![1; 5];
        let mut prev = vec![0; 5];
        for _ in 1..n {
            std::mem::swap(&mut prev, &mut patterns);
            patterns[0] = (prev[1] + prev[2] + prev[4]) % modulus;
            patterns[1] = (prev[0] + prev[2]) % modulus;
            patterns[2] = (prev[1] + prev[3]) % modulus;
            patterns[3] = prev[2];
            patterns[4] = (prev[2] + prev[3]) % modulus;
        }
        patterns.into_iter().fold(0, |acc, i| (acc + i) % modulus) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::count_vowel_permutation(1);
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::count_vowel_permutation(2);
        assert_eq!(10, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::count_vowel_permutation(5);
        assert_eq!(68, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::count_vowel_permutation(20000);
        assert_eq!(759959057, result)
    }
}
