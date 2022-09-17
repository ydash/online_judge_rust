// 336. Palindrome Pairs

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut trie = Trie::new();
        let words: Vec<&[u8]> = words.iter().map(|str| str.as_bytes()).collect();
        for i in 0..words.len() {
            trie.add(words[i], i);
        }
        'outer: for i in 0..words.len() {
            let word = words[i];
            let mut current = &mut trie;
            for j in 0..word.len() {
                if current.index != -1 && current.index as usize != i && is_palindrome(&word[j..]) {
                    result.push(vec![i as i32, current.index])
                }
                let next = current.children.get_mut(&(word[j] - 'a' as u8));
                if next.is_none() {
                    continue 'outer;
                }
                current = next.unwrap();
            }
            for &j in current.palindrome_indices.iter() {
                if i != j {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }
        result
    }
}

fn is_palindrome(str: &[u8]) -> bool {
    let mut l = 0;
    let mut r = str.len() - 1;
    while l < r {
        if str[l] != str[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

struct Trie {
    children: HashMap<u8, Trie>,
    index: i32,
    palindrome_indices: Vec<usize>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            index: -1,
            palindrome_indices: vec![],
        }
    }
    fn add(&mut self, str: &[u8], i: usize) {
        let mut current = self;
        for j in (0..str.len()).rev() {
            if is_palindrome(&str[..=j]) {
                current.palindrome_indices.push(i);
            }
            let next = str[j] - 'a' as u8;
            if !current.children.contains_key(&next) {
                current.children.insert(next, Trie::new());
            }
            current = current.children.get_mut(&next).unwrap();
        }
        current.index = i as i32;
        current.palindrome_indices.push(i);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::palindrome_pairs(
            vec!["abcd", "dcba", "lls", "s", "sssll"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        );
        let expected = vec![vec![0, 1], vec![1, 0], vec![3, 2], vec![2, 4]];
        assert_eq!(
            expected.iter().collect::<HashSet<_>>(),
            result.iter().collect::<HashSet<_>>()
        );
        assert_eq!(expected.len(), result.len());
    }

    #[test]
    fn test_case_02() {
        let result = Solution::palindrome_pairs(
            vec!["bat", "tab", "cat"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        );
        let expected = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(
            expected.iter().collect::<HashSet<_>>(),
            result.iter().collect::<HashSet<_>>()
        );
        assert_eq!(expected.len(), result.len());
    }

    #[test]
    fn test_case_03() {
        let result =
            Solution::palindrome_pairs(vec!["a", ""].into_iter().map(|s| s.to_owned()).collect());
        let expected = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(
            expected.iter().collect::<HashSet<_>>(),
            result.iter().collect::<HashSet<_>>()
        );
        assert_eq!(expected.len(), result.len());
    }
}
