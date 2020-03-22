/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        if len <= 1 {
            return 0;
        };
        let mut i: usize = 0;
        let mut j = len - 1;
        let mut res = 0;
        let mut h: i32;
        while i < j {
            h = std::cmp::min(height[i], height[j]);
            res = std::cmp::max(res, h * (j - i) as i32);
            if height[i] < height[j] {
                i = i + 1;
            } else {
                j = j - 1;
            }
        }
        res
    }

    pub fn max_area2(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut tmp: i32;
        let mut h: i32;
        let len = height.len();
        for i in 0..(len - 1) {
            for j in (i + 1)..len {
                h = if height[i] > height[j] {
                    height[j]
                } else {
                    height[i]
                };
                tmp = h * (j - i) as i32;
                if tmp > max {
                    max = tmp;
                }
            }
        }
        max
    }
}


// @lc code=end

