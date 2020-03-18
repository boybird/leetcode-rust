/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|v| *v != val);
        nums.len() as i32
    }
}
// @lc code=end

mod test {
    #[test]
    fn test_it() {
        use super::*;
        let mut s = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let n = Solution::remove_element(&mut s, 2);
        println!("n = {}\n{:?}", n, s);
    }
}
