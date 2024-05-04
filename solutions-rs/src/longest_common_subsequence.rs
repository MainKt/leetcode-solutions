use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut memo = HashMap::new();
        longest_common_subsequence(&mut memo, text1.as_str(), text2.as_str()) as i32
    }
}

pub fn longest_common_subsequence(
    memo: &mut HashMap<(usize, usize), usize>,
    text1: &str,
    text2: &str,
) -> usize {
    if let Some(&length) = memo.get(&(text1.len(), text2.len())) {
        return length;
    }

    let Some(ch1) = text1.chars().next() else {
        return 0;
    };

    let Some(ch2) = text2.chars().next() else {
        return 0;
    };

    let length = if ch1 == ch2 {
        1 + longest_common_subsequence(memo, &text1[1..], &text2[1..])
    } else {
        longest_common_subsequence(memo, &text1[1..], text2).max(longest_common_subsequence(
            memo,
            text1,
            &text2[1..],
        ))
    };

    memo.insert((text1.len(), text2.len()), length);

    length
}
