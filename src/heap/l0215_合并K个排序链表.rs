/*
 * @lc app=leetcode.cn id=215 lang=rust
 *
 * [215] 数组中的第K个最大元素
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        nums[nums.len() - k as usize]
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {}
}
