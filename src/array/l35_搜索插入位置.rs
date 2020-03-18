/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.iter()
            .position(|x| *x >= target)
            .map(|x| x as i32)
            .unwrap_or(nums.len() as i32)
    }
}
// @lc code=end

mod tests {
    #[test]
    fn test_it() {
        use super::*;

        
        let r = Solution::search_insert(vec![1, 3, 5, 6], 5);
        assert_eq!(r, 2);
        let r = Solution::search_insert(vec![1, 3, 5, 6], 2);
        assert_eq!(r, 1);

        let r = Solution::search_insert(vec![1, 3, 5, 6], 7);
        assert_eq!(r, 4);

        let r = Solution::search_insert(vec![1, 3, 5, 6], 0);
        assert_eq!(r, 0);
    }
}
