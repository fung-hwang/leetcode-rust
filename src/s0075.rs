struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // let mut idx_0 = 0;
        // for i in 0..nums.len() {
        //     if nums[i] == 0 {
        //         nums.swap(idx_0, i);
        //         idx_0 += 1;
        //     }
        // }
        // let mut idx_2 = nums.len() - 1;
        // for i in (0..nums.len()).rev() {
        //     if nums[i] == 2 {
        //         nums.swap(idx_2, i);
        //         idx_2 -= 1;
        //     }
        // }
        let len = nums.len();
        let (mut idx_0, mut idx_2) = (0, len - 1);
        for i in 0..len {
            while i < idx_2 && nums[i] == 2 {
                nums.swap(idx_2, i);
                idx_2 -= 1;
            }
            // 注意 '== 0' 的情况写在下面，因为可能出现 while 后 nums[i] == 0 的情况
            // 这个 0 需要在本轮就处理掉
            if nums[i] == 0 {
                nums.swap(idx_0, i);
                idx_0 += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut v = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![0, 0, 1, 1, 2, 2]);

        let mut v = vec![2, 0, 1];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![0, 1, 2]);
    }
}
