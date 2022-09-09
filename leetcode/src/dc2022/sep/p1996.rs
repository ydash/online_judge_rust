// 1996. The Number of Weak Characters in the Game

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut map = HashMap::new();
        for prop in properties.iter() {
            map.entry(prop[0]).or_insert(vec![]).push(prop[1]);
        }
        let mut max = 0;
        for i in (1..=100000).rev() {
            if let Some(v) = map.get(&i) {
                let mut tmp = max;
                for &d in v.iter() {
                    if d < max {
                        result += 1;
                    }
                    tmp = tmp.max(d);
                }
                max = tmp;
            }
        }
        result
    }
    // pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
    //     let mut result = 0;
    //     let mut map = HashMap::new();
    //     for prop in properties.iter() {
    //         let a = prop[0];
    //         let d = prop[1];
    //         map.entry(a).or_insert(vec![]).push(d);
    //     }
    //     println!("{:?}", map);
    //     let mut bit = BIT::new(100000);
    //     for i in 1..=100000 {
    //         if let Some(v) = map.get(&i) {
    //             for &d in v.iter() {
    //                 println!("{},{}:{}", i, d - 1, bit.sum(d - 1));
    //                 result += bit.sum(d - 1);
    //             }
    //             for &d in v.iter() {
    //                 bit.update(d, 1);
    //             }
    //         }
    //     }
    //     result
    // }
}

// struct BIT {
//     arr: Vec<i32>,
//     size: usize,
// }

// impl BIT {
//     fn new(size: usize) -> Self {
//         BIT {
//             arr: vec![0; size + 1],
//             size,
//         }
//     }

//     fn sum(&self, i: i32) -> i32 {
//         let mut i = i;
//         let mut ret = 0;
//         while i > 0 {
//             ret += self.arr[i as usize];
//             i -= i & -i
//         }
//         ret
//     }

//     fn update(&mut self, i: i32, diff: i32) {
//         let mut i = i;
//         let lim = self.size as i32;
//         while i <= lim {
//             self.arr[i as usize] += diff;
//             i += i & -i;
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::number_of_weak_characters(vec![
            vec![1, 1],
            vec![2, 1],
            vec![2, 2],
            vec![1, 2],
        ]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::number_of_weak_characters(vec![
            vec![7, 7],
            vec![1, 2],
            vec![9, 7],
            vec![7, 3],
            vec![3, 10],
            vec![9, 8],
            vec![8, 10],
            vec![4, 3],
            vec![1, 5],
            vec![1, 5],
        ]);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::number_of_weak_characters(vec![vec![1, 5], vec![1, 1]]);
        assert_eq!(0, result)
    }
}
