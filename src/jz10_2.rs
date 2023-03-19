struct Solution;

impl Solution {
    pub fn num_ways(n: i32) -> i32 {
        const N: i32 = 1000000007;
        if n == 0 || n == 1 {
            return 1;
        }
        (2..=n)
            .fold((1, 1), |a: (i32, i32), _| (a.1 % N, (a.0 + a.1) % N))
            .1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::num_ways(7), 21);
    }
}
