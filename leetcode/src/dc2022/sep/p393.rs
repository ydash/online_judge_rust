// 393. UTF-8 Validation

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut count = 0;
        for c in data {
            if count == 0 {
                if (c >> 5) == 0b110 {
                    count = 1;
                } else if (c >> 4) == 0b1110 {
                    count = 2;
                } else if (c >> 3) == 0b11110 {
                    count = 3;
                } else if (c >> 7) != 0 {
                    return false;
                }
            } else {
                if (c >> 6) != 0b10 {
                    return false;
                }
                count -= 1;
            }
        }
        count == 0
    }
}
