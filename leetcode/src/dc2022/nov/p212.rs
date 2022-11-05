// 212. Word Search II

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        fn dfs(
            i: i32,
            j: i32,
            board: &Vec<Vec<char>>,
            current: &Trie,
            seen: &mut Vec<Vec<bool>>,
            res: &mut HashSet<String>,
        ) {
            if let Some(term) = &current.term {
                res.insert(term.to_owned());
            }
            let m = seen.len() as i32;
            let n = seen[0].len() as i32;
            vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                .into_iter()
                .filter(|&(x, y)| x >= 0 && x < m && y >= 0 && y < n)
                .for_each(|(x, y)| {
                    if !seen[x as usize][y as usize]
                        && current.children[usize::from(board[x as usize][y as usize] as u8 - b'a')]
                            .is_some()
                    {
                        seen[x as usize][y as usize] = true;
                        let next = current.children
                            [usize::from(board[x as usize][y as usize] as u8 - b'a')]
                        .as_ref()
                        .unwrap();
                        dfs(x, y, board, next, seen, res);
                        seen[x as usize][y as usize] = false;
                    }
                });
        }
        let m = board.len();
        let n = board[0].len();
        let mut result = HashSet::new();
        let mut trie = Trie::new();
        for word in words {
            trie.add(word);
        }
        let mut seen = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                trie.children[usize::from(board[i][j] as u8 - b'a')]
                    .as_ref()
                    .map(|current| {
                        seen[i][j] = true;
                        dfs(i as i32, j as i32, &board, current, &mut seen, &mut result);
                        seen[i][j] = false;
                    });
            }
        }
        result.into_iter().collect()
    }
}

#[derive(Clone)]
struct Trie {
    children: Vec<Option<Trie>>,
    term: Option<String>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: vec![None; 26],
            term: None,
        }
    }

    fn add(&mut self, str: String) {
        let mut current = self;
        for &b in str.as_bytes() {
            current = current.children[usize::from(b - b'a')].get_or_insert(Self::new())
        }
        current.term = Some(str);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let mut result = Solution::find_words(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v'],
            ],
            vec![
                "oath".to_owned(),
                "pea".to_owned(),
                "eat".to_owned(),
                "rain".to_owned(),
            ],
        );
        result.sort();
        assert_eq!(vec!["eat", "oath"], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_words(
            vec![vec!['a', 'b'], vec!['c', 'd']],
            vec!["abcb".to_owned()],
        );
        assert_eq!(Vec::<String>::new(), result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::find_words(vec![vec!['a', 'a']], vec!["aaa".to_owned()]);
        assert_eq!(Vec::<String>::new(), result)
    }
}
