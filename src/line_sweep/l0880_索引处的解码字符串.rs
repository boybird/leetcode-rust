/*
 * @lc app=leetcode.cn id=880 lang=rust
 *
 * [880] 索引处的解码字符串
 */

struct Solution;
// @lc code=start
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut k = k as usize;
        let mut size = 0usize;
        let N = s.len();
        for c in s.chars() {
            if c.is_numeric() {
                size *= c as usize - '0' as usize;
            } else {
                size += 1;
            }
        }
        for i in (0..N).rev() {
            k = k % size;
            if k == 0 && !s.chars().nth(i).unwrap().is_numeric() {
                return String::from(&s[i..=i]);
            }
            if s.chars().nth(i).unwrap().is_numeric() {
                size /= s.chars().nth(i).unwrap() as usize - '0' as usize;
            } else {
                size -= 1;
            }
        }

        "".to_string()
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;
        let s = "leet2code3".to_string();
        assert_eq!(Solution::decode_at_index(s, 14), "e".to_string());
    }
}
