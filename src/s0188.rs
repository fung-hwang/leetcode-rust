struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 {
            return 0;
        }
        let mut buy = vec![i32::MIN; (k + 1) as usize]; // 持有股票的状态
        let mut sell = vec![0; (k + 1) as usize]; // 不持有股票的状态
        for day in 0..n {
            let mut buy_tmp = buy.clone();
            let mut sell_tmp = sell.clone();
            for i in 1..=k as usize {
                buy_tmp[i] = std::cmp::max(buy[i], sell[i - 1] - prices[day]);
                sell_tmp[i] = std::cmp::max(buy[i] + prices[day], sell[i]);
            }
            buy = buy_tmp;
            sell = sell_tmp;
        }
        sell[k as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(2, vec![3]), 0);
        assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    }
}
