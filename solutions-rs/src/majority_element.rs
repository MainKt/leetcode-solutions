use crate::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        *nums
            .iter()
            .fold(HashMap::new(), |mut frequencies, &num| {
                *frequencies.entry(num).or_insert(0) += 1;
                frequencies
            })
            .iter()
            .max_by_key(|&(&_, &frequency)| frequency)
            .map(|(num, &_)| num)
            .unwrap()
    }
}
