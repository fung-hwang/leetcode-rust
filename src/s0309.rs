struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp::max;
        let n = prices.len();
        if n < 2 {
            return 0;
        }
        let mut buy = -prices[0]; // 持有
        let mut sell = 0; // 不持有，不冻结
        let mut freeze = 0; // 不持有，冻结

        for i in 1..n {
            let buy_tmp = max(buy, freeze - prices[i]);
            let sell_tmp = max(sell, buy + prices[i]);
            let freeze_tmp = max(freeze, sell);
            buy = buy_tmp;
            sell = sell_tmp;
            freeze = freeze_tmp;
        }

        max(freeze, sell)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    }
}
