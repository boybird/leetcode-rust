/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut steps = 0;
        let mut end = 0i32;
        let mut maxposition = 0;

        let mut i: usize = 0;
        let l = nums.len();

        while i < l - 1 {
            maxposition = std::cmp::max(maxposition, nums[i] + i as i32);
            if i == end as usize {
                end = maxposition;
                steps += 1;
            }
            i += 1;
        }

        steps
    }
}
// @lc code=end

mod tests {

    #[test]
    fn test_it() {
        use super::*;
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }
}
