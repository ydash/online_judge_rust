// 211. Design Add and Search Words Data Structure

struct WordDictionary {
    trie_node: TrieNode,
}

impl WordDictionary {
    #[allow(dead_code)]
    fn new() -> Self {
        WordDictionary {
            trie_node: TrieNode::new(),
        }
    }

    #[allow(dead_code)]
    fn add_word(&mut self, word: String) {
        self.trie_node.add(word.as_bytes());
    }

    #[allow(dead_code)]
    fn search(&self, word: String) -> bool {
        self.trie_node.search(word.as_bytes())
    }
}

struct TrieNode {
    is_term: bool,
    children: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
    #[allow(dead_code)]
    fn new() -> Self {
        TrieNode {
            is_term: false,
            children: Default::default(),
        }
    }

    #[allow(dead_code)]
    fn add(&mut self, word: &[u8]) {
        let mut current = self;
        for &b in word {
            let i = usize::from(b - b'a');
            current = current.children[i].get_or_insert(Box::new(TrieNode::new()));
        }
        current.is_term = true;
    }

    #[allow(dead_code)]
    fn search(&self, word: &[u8]) -> bool {
        if word.is_empty() {
            self.is_term
        } else {
            if word[0] == b'.' {
                for b in b'a'..=b'z' {
                    if let Some(next) = &self.children[usize::from(b - b'a')] {
                        if next.search(&word[1..]) {
                            return true;
                        }
                    }
                }
                false
            } else {
                if let Some(next) = &self.children[usize::from(word[0] - b'a')] {
                    next.search(&word[1..])
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WordDictionary;

    #[test]
    fn test_case_01() {
        let mut word_dictionary = WordDictionary::new();
        word_dictionary.add_word("bad".to_owned());
        word_dictionary.add_word("dad".to_owned());
        word_dictionary.add_word("mad".to_owned());
        assert_eq!(false, word_dictionary.search("pad".to_owned()));
        assert_eq!(true, word_dictionary.search("bad".to_owned()));
        assert_eq!(true, word_dictionary.search(".ad".to_owned()));
        assert_eq!(true, word_dictionary.search("b..".to_owned()));
        assert_eq!(false, word_dictionary.search("b.x".to_owned()));
    }
}
