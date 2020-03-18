/*
 * @lc app=leetcode.cn id=34 lang=rust
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //nums.iter().f
        vec![
            nums.iter()
                .position(|&x| x == target)
                .map(|x| x as i32)
                .unwrap_or(-1),
            nums.iter()
                .rposition(|&x| x == target)
                .map(|x| x as i32)
                .unwrap_or(-1),
        ]
    }
}
// @lc code=end

mod tests {
    #[test]
    fn test_it() {
        use super::*;

        let a = vec![5, 7, 7, 8, 8, 10];
        assert_eq!(Solution::search_range(a, 8), vec![3, 4]);
    }
}
