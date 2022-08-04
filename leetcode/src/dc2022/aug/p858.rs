// 858. Mirror Reflection

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let mut i = 1;
        let mut j = 1;
        while i * p != j * q {
            j += 1;
            i = j * q / p;
        }
        if i % 2 == 0 && j % 2 == 1 {
            0
        } else if i % 2 == 1 && j % 2 == 1 {
            1
        } else if i % 2 == 1 && j % 2 == 0 {
            2
        } else {
            -1
        }
    }
}
