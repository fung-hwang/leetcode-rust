struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rst = vec![];
        if matrix.is_empty() || matrix[0].is_empty() {
            return rst;
        }
        // 若下标类型为 usize 而非 i32/isize ，在进行下标加减时会很难办
        let (mut r1, mut r2, mut c1, mut c2) = (
            0_i32,
            (matrix.len() - 1) as i32,
            0_i32,
            (matrix[0].len() - 1) as i32,
        );
        while r1 <= r2 && c1 <= c2 {
            for j in c1..=c2 {
                rst.push(matrix[r1 as usize][j as usize]);
            }
            for i in (r1 + 1)..=r2 {
                rst.push(matrix[i as usize][c2 as usize]);
            }
            if r1 != r2 {
                for j in (c1..c2).rev() {
                    rst.push(matrix[r2 as usize][j as usize]);
                }
            }
            if c1 != c2 {
                for i in (r1 + 1..r2).rev() {
                    rst.push(matrix[i as usize][c1 as usize]);
                }
            }
            r1 += 1;
            r2 -= 1;
            c1 += 1;
            c2 -= 1;
        }
        rst
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jz29() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        let matrix = vec![vec![1, 2, 3]];
        assert_eq!(Solution::spiral_order(matrix), vec![1, 2, 3]);
        let matrix = vec![vec![1], vec![4], vec![7]];
        assert_eq!(Solution::spiral_order(matrix), vec![1, 4, 7]);
    }
}
