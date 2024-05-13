use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy_day = 0;
        let mut profit = 0;
        for day in 1..prices.len() {
            if prices[day] > prices[buy_day] {
                profit = profit.max(prices[day] - prices[buy_day]);
            }
            if prices[day] < prices[buy_day] {
                buy_day = day;
            }
        }
        profit
    }
}
