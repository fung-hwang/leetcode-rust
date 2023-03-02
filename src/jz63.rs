use std::cmp::{max, min};

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut max_profit = 0;
        let mut lowest_price = prices[0];
        for price in prices {
            lowest_price = min(lowest_price, price);
            max_profit = max(max_profit, price - lowest_price);
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
