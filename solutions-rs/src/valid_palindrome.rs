use crate::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let orignal: String = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        let reversed: String = orignal.chars().rev().collect();

        orignal == reversed
    }
}
