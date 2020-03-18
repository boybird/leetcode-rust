/*
 * @lc app=leetcode.cn id=33 lang=rust
 *
 * [33] 搜索旋转排序数组
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return -1;
        }
        if n == 1 {
            return if nums[0] == target { 0 } else { -1 };
        }
        let mut a = 1;
        while a < n {
            if nums[a] < nums[a - 1] {
                break;
            }
            a = a + 1;
        }
        let mut r = nums[0..a].binary_search(&target);
        if r.is_ok() {
            return r.unwrap() as i32;
        }
        r = nums[a..n].binary_search(&target);
        if r.is_ok() {
            return (r.unwrap() + a) as i32;
        }
        -1
    }
}
// @lc code=end

mod tests {
    #[test]
    fn test_it() {
        use super::*;

        let a = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(Solution::search(a, 0), 4);
    }
}
