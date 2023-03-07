struct Solution;

impl Solution {
    // 分治法，会重复计算小区间的值，所以DP会更好
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let expression: Vec<char> = expression.chars().collect();
        Self::diff_ways_to_compute_2(&expression, 0, expression.len() - 1)
    }

    fn diff_ways_to_compute_2(expression: &Vec<char>, left: usize, right: usize) -> Vec<i32> {
        let mut rst = vec![];
        for i in left..=right {
            if expression[i] == '*' || expression[i] == '-' || expression[i] == '+' {
                let left_rst = Self::diff_ways_to_compute_2(&expression, left, i - 1);
                let right_rst = Self::diff_ways_to_compute_2(&expression, i + 1, right);
                for left_num in &left_rst {
                    for right_num in &right_rst {
                        match expression[i] {
                            '*' => rst.push(left_num * right_num),
                            '-' => rst.push(left_num - right_num),
                            '+' => rst.push(left_num + right_num),
                            _ => panic!("wrong operator"),
                        }
                    }
                }
            }
        }
        if rst.is_empty() {
            rst.push(Self::chars_to_num(&expression, left, right));
        }
        // println!("{:?} {} {}", rst, left, right);
        rst
    }

    fn chars_to_num(expression: &Vec<char>, left: usize, right: usize) -> i32 {
        match right - left + 1 {
            1 => expression[left] as i32 - '0' as i32,
            2 => {
                (expression[left] as i32 - '0' as i32) * 10
                    + (expression[right] as i32 - '0' as i32)
            }
            _ => {
                println!("{:?}", expression);
                panic!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(
        //     Solution::diff_ways_to_compute("2-1-1".to_string()),
        //     vec![0, 2]
        // );
        // assert_eq!(
        //     Solution::diff_ways_to_compute("2*3-4*5".to_string()),
        //     vec![-34, -14, -10, -10, 10]
        // );
    }
}
