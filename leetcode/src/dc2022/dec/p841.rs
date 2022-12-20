// 841. Keys and Rooms

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut stack = vec![0];
        let mut visited = HashSet::from([0]);
        while let Some(i) = stack.pop() {
            for &j in rooms[i].iter() {
                if !visited.contains(&j) {
                    stack.push(j as usize);
                    visited.insert(j);
                }
            }
        }
        visited.len() == rooms.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result =
            Solution::can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]);
        assert_eq!(false, result)
    }
}
