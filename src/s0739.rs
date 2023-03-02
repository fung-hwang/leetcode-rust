struct Solution;

impl Solution {
    // 单调栈维护不严格单调递减的温度
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut rst = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            while let Some(&top) = stack.last() {
                if temperatures[top] < temperatures[i] {
                    rst[top] = (i - top) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        rst
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 30, 30, 30]),
            vec![0, 0, 0, 0]
        );
    }
}
