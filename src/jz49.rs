struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        use std::cmp::min;
        if n <= 0 {
            panic!()
        }
        let n = n as usize;
        if n == 1 {
            return 1;
        }
        let mut ugly_number = vec![0; n];
        ugly_number[0] = 1;
        let (mut i, mut j, mut k) = (0, 0, 0);
        for idx in 1..n {
            let (next_2, next_3, next_5) =
                (ugly_number[i] * 2, ugly_number[j] * 3, ugly_number[k] * 5);
            let next_ugly = min(min(next_2, next_3), next_5);
            ugly_number[idx] = next_ugly;
            // 下面的写法是因为可能存在 2 * 3 == 3 * 2 == next_ugly 的情况
            // 此时 i++ 和 j++ 都是要做的
            if next_ugly == next_2 {
                i += 1;
            }
            if next_ugly == next_3 {
                j += 1;
            }
            if next_ugly == next_5 {
                k += 1;
            }
        }

        *ugly_number.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
    }
}
