/*
 * @lc app=leetcode.cn id=38 lang=rust
 *
 * [38] 外观数列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut t = "1".to_string();
        // if n == 1 {
        //     return s;
        // }
        for _i in 2..=n {
            let mut c0 = 'c';
            let mut p0 = 1;
            let t0 = t.clone();
            t.clear();

            for c in t0.chars() {
                if c == c0 {
                    p0 = p0 + 1;
                } else {
                    if c0 != 'c' {
                        t.push_str(&format!("{}{}", p0, c0));
                    }
                    c0 = c;
                    p0 = 1;
                }
            }
            t.push_str(&format!("{}{}", p0, c0));            
        }
        t
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;
        // dbg!(Solution::count_and_say(1));
        // dbg!(Solution::count_and_say(2));
        // dbg!(Solution::count_and_say(3));
        // dbg!(Solution::count_and_say(4));
        dbg!(Solution::count_and_say(5));
    }
}
