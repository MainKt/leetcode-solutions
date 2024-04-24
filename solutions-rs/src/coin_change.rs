use crate::Solution;

use std::collections::HashMap;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut memo = HashMap::new();
        coin_change_helper(&coins, amount, &mut memo)
    }
}

fn coin_change_helper(coins: &[i32], amount: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&count) = memo.get(&amount) {
        return count;
    }
    let count = match amount {
        0 => 0,
        amount if amount < 0 => -1,
        amount => coins
            .iter()
            .map(|coin| coin_change_helper(coins, amount - coin, memo))
            .filter(|&count| count >= 0)
            .min()
            .map(|count| count + 1)
            .unwrap_or(-1),
    };
    memo.insert(amount, count);
    count
}
