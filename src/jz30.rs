#[derive(Default)]
struct MinStack {
    stack: Vec<i32>,
    stack_min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
// 在实现上更简单的方法是“对等”记录 stack 和 stack_min
// 以下实现方法会稍稍节约stack_min的空间
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        match self.stack_min.last() {
            Some(&min) => {
                if x <= min {
                    self.stack_min.push(x)
                }
            }
            None => self.stack_min.push(x),
        }
    }

    fn pop(&mut self) {
        if let Some(top) = self.stack.pop() {
            if top == *self.stack_min.last().unwrap() {
                self.stack_min.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap_or(&-1)
    }

    fn min(&self) -> i32 {
        *self.stack_min.last().unwrap_or(&-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jz30() {
        let mut obj = MinStack::new();
        obj.push(1);
        obj.pop();
        let _ret_3: i32 = obj.top();
        let _ret_4: i32 = obj.min();
    }
}
