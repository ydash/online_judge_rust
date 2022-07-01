#[cfg(test)]
mod tests {
    use super::solution::*;

    #[test]
    fn testcase01() {
        let result = maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4);
        assert_eq!(result, 8);
    }

    #[test]
    fn testcase02() {
        let result = maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10);
        assert_eq!(result, 91);
    }
}

// 1710. Maximum Units on a Truck
mod solution {
    #[allow(dead_code)]
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));
        let mut sum = 0;
        let mut count = 0;
        for v in box_types.iter() {
            let boxes = v[0].min(truck_size - count);
            sum += boxes * v[1];
            count += boxes;
            if count >= truck_size {
                break;
            }
        }
        sum
    }
}
