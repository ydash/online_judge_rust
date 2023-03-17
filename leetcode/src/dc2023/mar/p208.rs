// 208. Implement Trie (Prefix Tree)

struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_term: bool,
}

impl Trie {
    #[allow(dead_code)]
    fn new() -> Self {
        Trie {
            children: Default::default(),
            is_term: false,
        }
    }

    #[allow(dead_code)]
    fn insert(&mut self, word: String) {
        let mut current = self;
        for &b in word.as_bytes() {
            let i = usize::from(b - b'a');
            current = current.children[i].get_or_insert(Box::new(Trie::new()));
        }
        current.is_term = true;
    }

    #[allow(dead_code)]
    fn search(&self, word: String) -> bool {
        let mut current = self;
        for &b in word.as_bytes() {
            let i = usize::from(b - b'a');
            if let Some(child) = &current.children[i] {
                current = child;
            } else {
                return false;
            }
        }
        current.is_term
    }

    #[allow(dead_code)]
    fn starts_with(&self, prefix: String) -> bool {
        let mut current = self;
        for &b in prefix.as_bytes() {
            let i = usize::from(b - b'a');
            if let Some(child) = &current.children[i] {
                current = child;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_case_01() {
        let mut trie = Trie::new();
        trie.insert("apple".to_owned());
        assert_eq!(true, trie.search("apple".to_owned()));
        assert_eq!(false, trie.search("app".to_owned()));
        assert_eq!(true, trie.starts_with("app".to_owned()));
        trie.insert("app".to_owned());
        assert_eq!(true, trie.search("app".to_owned()));
    }
}
