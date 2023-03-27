struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        use std::cmp::max;
        if prices.len() < 2 {
            return 0;
        }
        let mut buy = -prices[0]; // 买入，持有
        let mut sell = 0; // 卖出，不持有
        for i in 1..prices.len() {
            let buy_tmp = max(buy, sell - prices[i]);
            let sell_tmp = max(buy + prices[i] - fee, sell);
            buy = buy_tmp;
            sell = sell_tmp;
        }
        sell
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
        assert_eq!(Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6);
    }
}
