// 804. Unique Morse Code Words

use std::collections::HashSet;
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse_code = vec![
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        words
            .iter()
            .map(|word| {
                word.as_bytes()
                    .iter()
                    .map(|b| morse_code[usize::from(b - b'a')])
                    .collect::<String>()
            })
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::unique_morse_representations(util::str_vec_2_string_vec(vec![
            "gin", "zen", "gig", "msg",
        ]));
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::unique_morse_representations(util::str_vec_2_string_vec(vec!["a"]));
        assert_eq!(1, result)
    }
}
