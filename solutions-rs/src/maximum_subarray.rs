use crate::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter()
            .scan(0, |sum, &num| {
                *sum = num.max(*sum + num);
                Some(*sum)
            })
            .max()
            .unwrap_or(0)
    }
}
