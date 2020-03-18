/*
 * @lc app=leetcode.cn id=67 lang=rust
 *
 * [67] 二进制求和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut step = false;

        let mut pa = a.len();
        let mut pb = b.len();
        let mut c = Vec::with_capacity(std::cmp::max(pa, pb) + 1);
        let mut A;
        let mut B;
        loop {
            if pa == 0 && pb == 0 {
                break;
            }
            A = if pa == 0 {
                '0'
            } else {
                pa = pa - 1;
                a.chars().nth(pa).unwrap()
            };
            B = if pb == 0 {
                '0'
            } else {
                pb = pb - 1;
                b.chars().nth(pb).unwrap()
            };
            match (A, B, step) {
                ('0', '0', false) => c.push('0'),
                ('0', '0', true) | ('0', '1', false) | ('1', '0', false) => {
                    step = false;
                    c.push('1')
                }
                ('1', '1', false) | ('1', '0', true) | ('0', '1', true) => {
                    step = true;
                    c.push('0')
                }
                _ => c.push('1'),
            }
        }
        if step {
            c.push('1');
        }

        c.iter().rev().collect()
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;
        let c = Solution::add_binary("11".to_string(), "1".to_string());
        dbg!(c);
        let c = Solution::add_binary("1010".to_string(), "1010".to_string());
        dbg!(c);
    }
}
