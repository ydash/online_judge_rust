// 953. Verifying an Alien Dictionary

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut map = [0; 26];
        for (i, &b) in order.as_bytes().iter().enumerate() {
            map[(b - b'a') as usize] = i;
        }
        'outer: for (w1, w2) in words.iter().zip(words.iter().skip(1)) {
            'inner: for (b1, b2) in w1.as_bytes().iter().zip(w2.as_bytes()) {
                match map[(b1 - b'a') as usize].cmp(&map[(b2 - b'a') as usize]) {
                    std::cmp::Ordering::Less => continue 'outer,
                    std::cmp::Ordering::Equal => continue 'inner,
                    std::cmp::Ordering::Greater => return false,
                }
            }
            if w1.len() > w2.len() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::is_alien_sorted(
            vec!["hello".to_owned(), "leetcode".to_owned()],
            "hlabcdefgijkmnopqrstuvwxyz".to_owned(),
        );
        assert!(result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_alien_sorted(
            vec!["hello".to_owned(), "hello".to_owned()],
            "hlabcdefgijkmnopqrstuvwxyz".to_owned(),
        );
        assert!(result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::is_alien_sorted(
            vec!["hello".to_owned(), "helloo".to_owned()],
            "hlabcdefgijkmnopqrstuvwxyz".to_owned(),
        );
        assert!(result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::is_alien_sorted(
            vec!["helloo".to_owned(), "hello".to_owned()],
            "hlabcdefgijkmnopqrstuvwxyz".to_owned(),
        );
        assert!(!result)
    }
}
