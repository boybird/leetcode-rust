/*
 * @lc app=leetcode.cn id=502 lang=rust
 *
 * [502] IPO
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut w = w;
        let mut np_c: Vec<_> = profits
            .iter()
            .zip(capital.iter())
            .collect();
        // 按利润从小到大排
        np_c.sort_by(|a, b| b.0.cmp(&a.0));
        for _i in 0..k {
            match np_c.iter().position(|&x| w >= *x.1) {
                Some(idx) => {
                    w += np_c[idx].0;
                    np_c.remove(idx);
                }
                None => {}
            }
        }

        w
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;
        let k = 2;
        let W = 0;
        let Profits = vec![1, 2, 3];
        let Capital = vec![0, 1, 1];

        dbg!(Solution::find_maximized_capital(k, W, Profits, Capital));
    }
}
