struct Solution;

impl Solution {
    // 注意：0^(负数)没有意义，此处认为结果是inf，但rust标准库认为是0
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let n = n as i64; // 避免出现 n == i32::MIN 时 n.abs() 出错
        let rst = Self::pow(x, n.abs());
        if n >= 0 {
            rst
        } else {
            if rst == std::f64::INFINITY {
                // 当 rst == inf 时，1.0 / std::f64::INFINITY 不能直接得到结果
                // 所以需要单独考虑
                0.0
            } else {
                1.0 / rst
            }
        }
    }

    fn pow(x: f64, n: i64) -> f64 {
        match n {
            0 => 1.0,
            1 => x,
            _ => {
                if n % 2 == 0 {
                    let a = Self::pow(x, n / 2);
                    a * a
                } else {
                    let a = Self::pow(x, n / 2);
                    a * a * x
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::my_pow(2.0, 10).partial_cmp(&1024.0),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Solution::my_pow(2.0, -2147483648).partial_cmp(&0.0),
            Some(Ordering::Equal)
        );
        // println!("{:?}", Solution::pow(0.0, -2));
        assert_eq!(
            Solution::my_pow(0.0, -2).partial_cmp(&std::f64::INFINITY),
            Some(Ordering::Equal)
        );
    }
}
