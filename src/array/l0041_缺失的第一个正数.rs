/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut n = 0;
        for (idx, i) in nums.iter().enumerate().skip_while(|(_idx, x)| **x <= 0) {
            if idx > 0 && nums[idx] == nums[idx - 1] {
                continue;
            }
            n = n + 1;
            if n != *i {
                n = n - 1;
                break;
            }
        }
        n + 1
    }
}
// @lc code=end

mod tests {
    
    #[test]
    fn test_it() {
        use super::*;
        
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(Solution::first_missing_positive(vec![0, 2, 2, 1, 1]), 3);
    }
}
