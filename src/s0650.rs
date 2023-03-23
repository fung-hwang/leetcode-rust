struct Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n = n as usize;
        let mut min_op_cnt = vec![0; n + 1];
        (2..=n).for_each(|i| min_op_cnt[i] = i);
        for i in 2..n {
            for j in 1..=(n / i) {
                min_op_cnt[i * j] = std::cmp::min(min_op_cnt[i * j], min_op_cnt[i] + j);
            }
        }
        min_op_cnt[n].try_into().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_steps(2), 2);
        assert_eq!(Solution::min_steps(3), 3);
        assert_eq!(Solution::min_steps(8), 6);
    }
}
