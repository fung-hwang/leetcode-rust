struct Solution;

impl Solution {
    pub fn construct_arr(a: Vec<i32>) -> Vec<i32> {
        let mut b = vec![1; a.len()];
        if a.len() == 0 || a.len() == 1 {
            return b;
        }
        // left -> right
        let mut tmp = 1;
        for i in 0..a.len() - 1 {
            tmp *= a[i];
            b[i + 1] *= tmp;
        }
        // right -> left
        let mut tmp = 1;
        for i in (1..=a.len() - 1).rev() {
            tmp *= a[i];
            b[i - 1] *= tmp;
        }

        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::construct_arr(vec![1, 2, 3, 4, 5]),
            vec![120, 60, 40, 30, 24]
        );
        assert_eq!(Solution::construct_arr(vec![5]), vec![1]);
        assert_eq!(Solution::construct_arr(vec![]), vec![]);
    }
}
