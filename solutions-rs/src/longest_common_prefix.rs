use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let prefixes: Vec<String> = strs
            .iter()
            .min_by_key(|s| s.len())
            .unwrap()
            .chars()
            .scan("".to_string(), |prefix, c| {
                prefix.push(c);
                Some(prefix.to_string())
            })
            .collect();
        prefixes
            .iter()
            .rev()
            .find(|&prefix| strs.iter().all(|s| s.starts_with(prefix)))
            .map(|s| s.to_string())
            .unwrap_or("".to_string())
    }
}
