/*
 * @lc app=leetcode.cn id=31 lang=rust
 *
 * [31] 下一个排列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut j0;
        let n = nums.len();
        for i in (0..n - 1).rev() {
            if nums[i + 1] > nums[i] {
                j0 = n - 1;
                for j in (i + 1..=n - 1).rev() {
                    j0 = j;
                    if nums[j] > nums[i] {
                        break;
                    }
                }
                nums.swap(i, j0);
                nums[i + 1..n].reverse();
                return;
            }
        }
        nums.reverse();
    }
}
// @lc code=end

mod test {
    #[test]
    fn test_it() {
        use super::*;
        let mut a = vec![1, 2, 7, 4, 3, 1];
        Solution::next_permutation(&mut a);
        println!("{:?}", a);
    }
}
