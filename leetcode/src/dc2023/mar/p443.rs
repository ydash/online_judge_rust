// 443. String Compression

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut prev = chars[0];
        let mut count = 1;
        let mut i = 0;
        let mut threshold = 10;
        for x in 1..chars.len() {
            let current = chars[x];
            if prev == current {
                count += 1;
                if count == 2 {
                    i += 1;
                } else if count == threshold {
                    i += 1;
                    threshold *= 10;
                }
            } else {
                if count > 1 {
                    let mut j = i;
                    while count > 0 {
                        chars[j] = char::from((count % 10) as u8 + b'0');
                        j -= 1;
                        count /= 10;
                    }
                }
                count = 1;
                i += 1;
                chars[i] = current;
                prev = current;
                threshold = 10;
            }
        }
        if count > 1 {
            let mut j = i;
            while count > 0 {
                chars[j] = char::from((count % 10) as u8 + b'0');
                j -= 1;
                count /= 10;
            }
        }
        chars.drain(i + 1..);
        chars.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let chars = &mut vec!['a'];
        let result = Solution::compress(chars);
        assert_eq!(*chars, vec!['a']);
        assert_eq!(result, 1)
    }

    #[test]
    fn test_case_02() {
        let chars = &mut vec!['a', 'a'];
        let result = Solution::compress(chars);
        assert_eq!(*chars, vec!['a', '2']);
        assert_eq!(result, 2)
    }

    #[test]
    fn test_case_03() {
        let chars = &mut vec!['a', 'a', 'a'];
        let result = Solution::compress(chars);
        assert_eq!(*chars, vec!['a', '3']);
        assert_eq!(result, 2)
    }

    #[test]
    fn test_case_04() {
        let chars = &mut vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'];
        let result = Solution::compress(chars);
        assert_eq!(*chars, vec!['a', '1', '0']);
        assert_eq!(result, 3)
    }

    #[test]
    fn test_case_05() {
        let chars = &mut vec!['a', 'b'];
        let result = Solution::compress(chars);
        assert_eq!(*chars, vec!['a', 'b']);
        assert_eq!(result, 2)
    }

    #[test]
    fn test_case_06() {
        let chars = &mut vec!['a', 'a', 'b'];
        let result = Solution::compress(chars);
        assert_eq!(*chars, vec!['a', '2', 'b']);
        assert_eq!(result, 3)
    }

    #[test]
    fn test_case_07() {
        let chars = &mut vec!['a', 'b', 'b'];
        let result = Solution::compress(chars);
        assert_eq!(*chars, vec!['a', 'b', '2']);
        assert_eq!(result, 3)
    }

    #[test]
    fn test_case_08() {
        let chars = &mut vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'b'];
        let result = Solution::compress(chars);
        assert_eq!(*chars, vec!['a', '1', '0', 'b']);
        assert_eq!(result, 4)
    }

    #[test]
    fn test_case_09() {
        let chars = &mut vec!['b', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'];
        let result = Solution::compress(chars);
        assert_eq!(*chars, vec!['b', 'a', '1', '0']);
        assert_eq!(result, 4)
    }

    #[test]
    fn test_case_10() {
        let chars = &mut vec![
            'b', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'c', 'c', 'c', 'c', 'c', 'c',
            'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c', 'c',
        ];
        let result = Solution::compress(chars);
        assert_eq!(*chars, vec!['b', 'a', '1', '0', 'c', '2', '1']);
        assert_eq!(result, 7)
    }
}
