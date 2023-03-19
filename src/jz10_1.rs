struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        // const N: i32 = 1000000007;
        // if n == 0 || n == 1 {
        //     return n;
        // }
        // let (mut a, mut b) = (0, 1);
        // for _ in 2..=n {
        //     let tmp = (a + b) % N;
        //     a = b;
        //     b = tmp;
        // }
        // b
        match n {
            0 => 0,
            1 => 1,
            _ => {
                (1..n)
                    .fold((0, 1), |mut a, _| {
                        (a.1 % 1000000007, (a.0 + a.1) % 1000000007)
                    })
                    .1
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::fib(100),
            (354224848179261915075_i128 % 1000000007) as i32
        );
    }
}
