/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() <= 2 {
            return 0;
        }
        let len = height.len();
        let mut ans = 0;
        let mut left_max = vec![0; len];
        let mut right_max = vec![0; len];
        left_max[0] = height[0];

        for i in 1..len {
            left_max[i] = std::cmp::max(height[i], left_max[i - 1]);
        }
        right_max[len - 1] = height[len - 1];
        for i in (0..len - 1).rev() {
            right_max[i] = std::cmp::max(height[i], right_max[i + 1]);
        }
        for i in 1..len - 1 {
            ans += std::cmp::min(left_max[i], right_max[i]) - height[i];
        }

        ans
    }
}
// @lc code=end

mod tests {

    #[test]
    fn test_it() {
        use super::*;
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
