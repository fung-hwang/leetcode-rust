struct Solution;

impl Solution {
    pub fn min_number(mut nums: Vec<i32>) -> String {
        let concat = |num_1: i32, num_2: i32| num_1.to_string() + &num_2.to_string();
        nums.sort_unstable_by(|num_1, num_2| concat(*num_1, *num_2).cmp(&concat(*num_2, *num_1)));
        nums.iter().map(|num| num.to_string()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::min_number(vec![3, 30, 34, 5, 9]),
            "3033459".to_owned()
        );
    }
}
