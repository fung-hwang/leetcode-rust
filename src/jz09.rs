#[derive(Default)]
struct CQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        // CQueue {
        //     s1: vec![],
        //     s2: vec![],
        // }
        Default::default()
    }

    fn append_tail(&mut self, value: i32) {
        self.s1.push(value)
    }

    fn delete_head(&mut self) -> i32 {
        if let Some(s2_top) = self.s2.pop() {
            return s2_top;
        }
        while let Some(s1_top) = self.s1.pop() {
            self.s2.push(s1_top);
        }
        self.s2.pop().unwrap_or(-1)
        // match self.s2.pop() {
        //     Some(s2_top) => s2_top,
        //     None => {
        //         if self.s1.is_empty() {
        //             -1
        //         } else {
        //             while !self.s1.is_empty() {
        //                 self.s2.push(self.s1.pop().unwrap());
        //             }
        //             self.s2.pop().unwrap()
        //         }
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jz50() {
        //   Your CQueue object will be instantiated and called as such:
        let mut obj = CQueue::new();
        obj.append_tail(1);
        let ret_2: i32 = obj.delete_head();
    }
}
