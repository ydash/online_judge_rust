// 936. Stamping The Sequence

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        fn can_stamp(target: &mut [u8], stamp: &[u8]) -> Status {
            for (b1, b2) in target.iter().zip(stamp.iter()) {
                if b1 != &b'?' && b1 != b2 {
                    return Status::UnStampable;
                }
            }
            let mut changed = false;
            for i in 0..stamp.len() {
                if target[i] != b'?' {
                    changed = true;
                }
                target[i] = b'?';
            }
            if changed {
                Status::Stampable
            } else {
                Status::AlreadyStamped
            }
        }
        let stamp = stamp.as_bytes();
        let mut target = target.bytes().collect::<Vec<_>>();
        let mut result = vec![];
        let mut seen = HashSet::new();
        loop {
            let mut stamped = false;
            for i in 0..=target.len() - stamp.len() {
                if seen.contains(&i) {
                    continue;
                }
                match can_stamp(&mut target[i..], stamp) {
                    Status::UnStampable => continue,
                    Status::AlreadyStamped => {
                        seen.insert(i);
                        continue;
                    }
                    Status::Stampable => {
                        seen.insert(i);
                        result.push(i as i32);
                        stamped = true;
                        break;
                    }
                }
            }
            if !stamped {
                break;
            }
        }

        if target.iter().all(|&b| b == b'?') {
            result.reverse();
            result
        } else {
            vec![]
        }
    }
}

enum Status {
    Stampable,
    UnStampable,
    AlreadyStamped,
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::moves_to_stamp("abc".to_owned(), "ababc".to_owned());
        assert_eq!(vec![0, 2], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::moves_to_stamp("abca".to_owned(), "aabcaca".to_owned());
        assert_eq!(vec![3, 0, 1], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::moves_to_stamp("aye".to_owned(), "eyeye".to_owned());
        assert_eq!(Vec::<i32>::new(), result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::moves_to_stamp("mda".to_owned(), "mdadddaaa".to_owned());
        assert_eq!(Vec::<i32>::new(), result)
    }
}
