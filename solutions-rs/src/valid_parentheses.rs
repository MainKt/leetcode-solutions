use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::HashMap;
        let mirrored = HashMap::from([('(', ')'), ('{', '}'), ('[', ']')]);
        s.chars()
            .fold(Vec::new(), |mut stack, parenthesis| {
                if !stack.is_empty()
                    && mirrored
                        .get(&stack.last().unwrap())
                        .is_some_and(|&p| p == parenthesis)
                {
                    stack.pop();
                } else {
                    stack.push(parenthesis);
                }
                stack
            })
            .is_empty()
    }
}
