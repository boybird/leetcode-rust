/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除排序数组中的重复项
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 1;
        while i < nums.len() {
            if nums[i] == nums[i - 1] {
                nums.remove(i);
            } else {
                i = i + 1;
            }
        }
        nums.len() as i32
    }
}
// @lc code=end

mod test {
    #[test]
    fn test_it() {
        use super::*;
        let mut s = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let n = Solution::remove_duplicates(&mut s);
        println!("n = {}\n{:?}", n, s);
    }
}
