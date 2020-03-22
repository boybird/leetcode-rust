/*
 * @lc app=leetcode.cn id=58 lang=rust
 *
 * [58] 最后一个单词的长度
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split(' ')
            .rev()
            .find(|s| s.len() > 0)
            .map(|s| s.len() as i32)
            .unwrap_or(0)
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;
        dbg!(Solution::length_of_last_word("a ".to_string()));
        dbg!(Solution::length_of_last_word("hello world".to_string()));
    }
}
