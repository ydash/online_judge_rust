// 218. The Skyline Problem

use std::collections::BTreeMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut corners = vec![];
        for building in buildings.iter() {
            let left_corner = (building[0], -building[2]);
            let right_corner = (building[1], building[2]);
            corners.push(left_corner);
            corners.push(right_corner);
        }
        corners.sort_by(|a, b| {
            if a.0 != b.0 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });
        let mut btree_map = BTreeMap::new();
        btree_map.insert(0, 1);
        let mut prev = 0;
        for &(x, y) in corners.iter() {
            if y < 0 {
                *btree_map.entry(-y).or_default() += 1;
            } else {
                if *btree_map.get(&y).unwrap() > 1 {
                    *btree_map.entry(y).or_default() -= 1;
                } else {
                    btree_map.remove(&y);
                }
            }
            let current = *btree_map.iter().last().unwrap().0;
            if current != prev {
                result.push(vec![x, current]);
                prev = current;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::get_skyline(vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8],
        ]);
        assert_eq!(
            vec![
                vec![2, 10],
                vec![3, 15],
                vec![7, 12],
                vec![12, 0],
                vec![15, 10],
                vec![20, 8],
                vec![24, 0]
            ],
            result
        )
    }

    #[test]
    fn test_case_02() {
        let result = Solution::get_skyline(vec![vec![0, 2, 3], vec![2, 5, 3]]);
        assert_eq!(vec![vec![0, 3], vec![5, 0]], result)
    }
}
