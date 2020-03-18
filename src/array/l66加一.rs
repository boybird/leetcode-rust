/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut step = 1;
        for n in digits.iter_mut().rev() {
            *n = *n + step;
            if *n >= 10 {
                *n = *n - 10;
                step = 1;
            } else {
                step = 0;
                break;
            }
        }
        if step > 0 {
            digits.insert(0, step);
        }
        digits
    }
}
// @lc code=end

mod tests {

    #[test]
    fn test_it() {
        use super::*;
        dbg!(Solution::plus_one(vec![1, 2, 3]));
        dbg!(Solution::plus_one(vec![4, 3, 2, 1]));
    }
}
