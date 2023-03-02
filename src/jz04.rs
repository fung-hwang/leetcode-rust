struct Solution;

impl Solution {
    // 基本思路：通过有序关系，每次排除一行或一列元素
    // time: O(m+n)
    // space: O(1)
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        // let m = matrix.len();
        // if m == 0 {
        //     return false;
        // }
        // let n = matrix[0].len();
        // if n == 0 {
        //     return false;
        // }
        let (mut i, mut j) = (0, matrix[0].len() - 1);
        // let mut i = 0;
        // let mut j = n - 1;
        while i < matrix.len() {
            if matrix[i][j] == target {
                return true;
            } else if matrix[i][j] < target {
                // i = i + 1;
                i += 1;
            } else {
                // 此处判断 j == 0 的原因是 j 是 usize 类型
                // 本应该 while i < m && j >= 0，但 j 因为类型原因必然 >= 0
                if j == 0 {
                    return false;
                }
                // j = j - 1;
                j -= 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jz04() {
        let m = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![19, 21, 23, 26, 30],
        ];
        assert_eq!(Solution::find_number_in2_d_array(m.clone(), 5), true);
        assert_eq!(Solution::find_number_in2_d_array(m, 18), false);
    }
}
