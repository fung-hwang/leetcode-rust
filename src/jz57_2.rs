use crate::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let mut rst = vec![];
        let (mut i, mut j, mut sum) = (1, 2, 3);
        let mut seq = VecDeque::new();
        seq.push_back(1);
        seq.push_back(2);

        while j <= (target / 2 + 1) {
            if sum == target {
                rst.push(seq.clone().into());
                sum -= i;
                i += 1;
                seq.pop_front();
            } else if sum < target {
                j += 1;
                sum += j;
                seq.push_back(j);
            } else {
                sum -= i;
                i += 1;
                seq.pop_front();
            }
        }

        rst
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jz57_2() {
        assert_eq!(
            Solution::find_continuous_sequence(9),
            vec![vec![2, 3, 4], vec![4, 5]]
        );
        assert_eq!(
            Solution::find_continuous_sequence(15),
            vec![vec![1, 2, 3, 4, 5], vec![4, 5, 6], vec![7, 8]]
        );
    }
}
