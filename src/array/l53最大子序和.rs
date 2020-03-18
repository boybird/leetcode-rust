/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子序和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = std::i32::MIN;
        let mut tmp_max = 0;
        for n in nums.iter() {
            tmp_max = tmp_max + n;
            if tmp_max > max {
                max = tmp_max;
            }
            if tmp_max < 0 {
                tmp_max = 0;
            }
        }
        max
    }
}
// @lc code=end

mod tests {

    #[test]
    fn test_it() {
        use super::*;
        let c = Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        dbg!(c);
    }
}
