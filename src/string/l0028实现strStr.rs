/*
 * @lc app=leetcode.cn id=28 lang=rust
 *
 * [28] 实现 strStr()
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|c| c as i32).unwrap_or(-1)
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;
        dbg!(Solution::str_str("hello".to_string(), "ll".to_string()));
    }
}
