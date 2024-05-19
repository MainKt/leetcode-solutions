use crate::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .last()
            .map(|s| s.len())
            .unwrap_or(0) as i32
    }
}
