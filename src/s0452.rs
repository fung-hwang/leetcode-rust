struct Solution;

impl Solution {
    // 不重叠区间问题的变种：不用管重叠的区间（意味着会被同一支箭射到），只需要找到不重叠的区间即可
    // 贪心：因为每个气球都必须射到，因此在按区间右侧的大小排序后，每次选择区间右侧较小且不重叠的区间（该区间的右侧作为射击位置）
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut cnt = 1;
        let mut end = points[0][1];
        for i in 1..points.len() {
            if points[i][0] > end {
                cnt += 1;
                end = points[i][1];
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
            2
        );
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
            4
        );
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
            2
        );
    }
}
