/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let len = nums.len();
        if len < k || k < 1 {
            return vec![];
        }
        if k == 1 {
            return nums;
        }
        nums.windows(k)
            .map(|i| *(i.to_vec().iter().max().unwrap()))
            .collect()
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        println!("{:?}", Solution::max_sliding_window(nums, 3));
    }
}
