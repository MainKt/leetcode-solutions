use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let map: HashMap<_, _> = nums
            .iter()
            .enumerate()
            .map(|(index, &value)| (value, index))
            .collect();

        nums.iter()
            .enumerate()
            .find_map(|(i, value)| match map.get_key_value(&(target - value)) {
                Some((_, &j)) if j != i => Some(vec![i as i32, j as i32]),
                _ => None,
            })
            .unwrap()
    }
}
