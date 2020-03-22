/*
 * @lc app=leetcode.cn id=552 lang=rust
 *
 * [552] 学生出勤记录 II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let (mut dp00, mut dp01, mut dp02, mut dp10, mut dp11, mut dp12) =
            (1i64, 0i64, 0i64, 0i64, 0i64, 0i64);
        let (mut dp00_, mut dp01_, mut dp02_, mut dp10_, mut dp11_, mut dp12_);

        let M = 1_000_000_000 + 7;
        for _i in 0..n {
            dp00_ = (dp00 + dp01 + dp02) % M;
            dp01_ = dp00;
            dp02_ = dp01;
            dp10_ = (dp00 + dp01 + dp02 + dp10 + dp11 + dp12) % M;
            dp11_ = dp10;
            dp12_ = dp11;

            dp00 = dp00_;
            dp01 = dp01_;
            dp02 = dp02_;
            dp10 = dp10_;
            dp11 = dp11_;
            dp12 = dp12_;
        }
        ((dp00 + dp01 + dp02 + dp10 + dp11 + dp12) % M) as i32
    }
}
// @lc code=end
