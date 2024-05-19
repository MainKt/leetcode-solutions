use crate::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let subsequence: Vec<_> = s.chars().rev().collect();
        t.chars()
            .fold(subsequence, |mut subsequence: Vec<char>, c| {
                if subsequence.last() == Some(&c) {
                    subsequence.pop();
                }
                subsequence
            })
            .is_empty()
    }
}
