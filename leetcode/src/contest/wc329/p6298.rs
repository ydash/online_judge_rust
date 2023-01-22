//  6298. Apply Bitwise Operations to Make Strings Equal

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn make_strings_equal(s: String, target: String) -> bool {
        s.as_bytes().iter().max() == target.as_bytes().iter().max()
    }
}
