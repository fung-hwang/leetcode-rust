struct Solution;

impl Solution {
    // 通过数学证明每段长度为3时乘积最大，并讨论特殊情况
    // time: O(1)
    // space: O(1)
    pub fn cutting_rope(n: i32) -> i32 {
        let n: u32 = n as u32;
        match n {
            2 => 1,
            3 => 2,
            _ => {
                let quotient = n / 3;
                let remainder = n % 3;
                if remainder == 0 {
                    3_i32.pow(quotient)
                } else if remainder == 1 {
                    3_i32.pow(quotient - 1) * 4
                } else {
                    3_i32.pow(quotient) * 2
                }
            }
        }
        // match n {
        //     2 => 1,
        //     3 => 2,
        //     4 => 4,
        //     _ => {
        //         let mut product = 1;
        //         while n > 4 {
        //             n -= 3;
        //             product *= 3;
        //         }

        //         product * n
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::cutting_rope(2), 1);
        assert_eq!(Solution::cutting_rope(5), 6);
        assert_eq!(Solution::cutting_rope(10), 36);
    }
}
