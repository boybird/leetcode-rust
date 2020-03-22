/*
 * @lc app=leetcode.cn id=29 lang=rust
 *
 * [29] 两数相除
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }
        if divisor == 1 {
            return dividend;
        }
        if divisor == -1 {
            return if dividend > std::i32::MIN {
                -dividend
            } else {
                std::i32::MAX
            };
        }
        let mut a = dividend as i64;
        let mut b = divisor as i64;
        let mut sign: i32 = 1;
        if (a > 0 && b < 0) || (a < 0 && b > 0) {
            sign = -1;
        }
        a = if a > 0 { a } else { -a };
        b = if b > 0 { b } else { -b };
        let res = Self::div(a, b);
        if sign > 0 {
            return if res > (std::i32::MAX as i64) {
                std::i32::MAX
            } else {
                res as i32
            };
        }
        -res as i32
    }

    fn div(a: i64, b: i64) -> i64 {
        if a < b {
            return 0;
        }
        let mut count: i64 = 1;
        let mut tb: i64 = b;
        while (tb + tb) <= a {
            count = count + count;
            tb = tb + tb;
        }
        count + Self::div(a - tb, b)
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;
        dbg!(Solution::divide(10, 3));
        dbg!(Solution::divide(7, -3));
    }
}
