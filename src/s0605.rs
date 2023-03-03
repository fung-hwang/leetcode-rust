struct Solution;

impl Solution {
    // 贪心的点在于从左到右遍历时，尽可能在最左边种花，即在能种花时立刻种花，而不是全盘考虑在哪种花（启发式的）
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut cnt = 0;
        for i in 0..flowerbed.len() {
            if flowerbed[i] == 0
                && (i == 0 || flowerbed[i - 1] == 0)
                && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0)
            {
                cnt += 1;
                if cnt >= n {
                    return true;
                }
                flowerbed[i] = 1;
            }
        }
        cnt >= n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
        assert_eq!(
            Solution::can_place_flowers(vec![0, 0, 1, 0, 0, 0, 1], 2),
            true
        );
    }
}
