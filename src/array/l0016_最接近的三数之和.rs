/*
 * @lc app=leetcode.cn id=16 lang=rust
 *
 * [16] 最接近的三数之和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut j;
        let mut k;
        let mut f;
        let mut sum;
        let mut d;
        let mut dff = std::i32::MAX;
        for i in 0..nums.len() - 2 {
            f = nums[i];
            j = i + 1;
            k = nums.len() - 1;
            while j < k {
                sum = f + nums[j] + nums[k];
                d = sum - target;
                if d.abs() < dff.abs() {
                    dff = d;
                }
                if sum > target {
                    k = k - 1;
                } else {
                    j = j + 1;
                }
            }
        }
        target + dff
    }

    pub fn three_sum_closest_bad(nums: Vec<i32>, target: i32) -> i32 {
        let mut res;
        let mut diff = std::i32::MAX;
        for i in 0..(nums.len() - 2) {
            for j in i + 1..(nums.len() - 1) {
                for k in j + 1..(nums.len()) {
                    res = nums[i] + nums[j] + nums[k] - target;
                    if res.abs() < diff.abs() {
                        // println!("{} + {} + {}", nums[i], nums[j], nums[k]);
                        diff = res;
                    }
                }
            }
        }
        target + diff
    }
}
// @lc code=end

mod test {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2)
    }
}
