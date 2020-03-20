/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows <= 1 {
            return s;
        }
        let chunk = 2 * num_rows - 2;
        let mut r = String::with_capacity(s.len());
        for i in 0..num_rows {
            if i == 0 || i == (num_rows - 1) {
                for c in s.chars().skip(i).step_by(chunk) {
                    r.push(c);
                }
            } else {
                let mut iter0 = s.chars().skip(i).step_by(chunk);
                let mut iter1 = s.chars().skip(chunk - i).step_by(chunk);
                loop {
                    match (iter0.next(), iter1.next()) {
                        (Some(c0), Some(c1)) => {
                            r.push(c0);
                            r.push(c1);
                        }
                        (Some(c0), None) => {
                            r.push(c0);
                        }
                        (None, Some(c0)) => {
                            r.push(c0);
                        }

                        (None, None) => {
                            break;
                        }
                    }
                }
            }
        }
        r
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;

        dbg!(Solution::convert("LEETCODEISHIRING".to_owned(), 3));
        dbg!(Solution::convert("PAYPALISHIRING".to_owned(), 3));
    }
}
