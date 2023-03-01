use crate::Solution;

impl Solution {
    // 模拟栈的操作过程
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut j = 0;
        for num in pushed {
            stack.push(num);

            while let Some(&top) = stack.last() {
                if top == popped[j] {
                    stack.pop();
                    j += 1;
                } else {
                    break;
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jz31() {
        assert_eq!(
            Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]),
            true
        );
        assert_eq!(
            Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]),
            false
        );
    }
}
