/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = vec![];
        if nums.len() < 4 {
            return res;
        }
        let len = nums.len();
        for k in 0..len - 3 {
            /* 当k的值与前面的值相等时跳过 */
            if k > 0 && nums[k] == nums[k - 1] {
                continue;
            }
            /*获取当前最小值，如果最小值比目标值大，说明后面越来越大的值根本没戏*/
            if nums[k] + nums[k + 1] + nums[k + 2] + nums[k + 3] > target {
                break;
            }
            /*获取当前最大值，如果最大值比目标值小，说明后面越来越小的值根本没戏，忽略*/
            if nums[k] + nums[len - 1] + nums[len - 2] + nums[len - 3] < target {
                continue;
            }
            /*第二层循环j，初始值指向i+1*/
            for i in k + 1..len - 2 {
                /*当i的值与前面的值相等时忽略*/
                if i > k + 1 && nums[i] == nums[i - 1] {
                    continue;
                }
                let mut j = i + 1;
                let mut h = len - 1;
                /*获取当前最小值，如果最小值比目标值大，说明后面越来越大的值根本没戏，忽略*/
                if nums[k] + nums[i] + nums[j] + nums[j + 1] > target {
                    continue;
                }
                /*获取当前最大值，如果最大值比目标值小，说明后面越来越小的值根本没戏，忽略*/
                if nums[k] + nums[i] + nums[h] + nums[h - 1] < target {
                    continue;
                }
                /*开始j指针和h指针的表演，计算当前和，如果等于目标值，j++并去重，h--并去重，当当前和大于目标值时h--，当当前和小于目标值时j++*/
                while j < h {
                    let curr = nums[k] + nums[i] + nums[j] + nums[h];
                    match curr.cmp(&target) {
                        std::cmp::Ordering::Equal => {
                            res.push(vec![nums[k], nums[i], nums[j], nums[h]]);
                            j = j + 1;
                            while j < h && nums[j] == nums[j - 1] {
                                j = j + 1;
                            }
                            h = h - 1;
                            while j < h && nums[h] == nums[h + 1] {
                                h = h - 1;
                            }
                        }
                        std::cmp::Ordering::Greater => {
                            h = h - 1;
                        }
                        std::cmp::Ordering::Less => {
                            j = j + 1;
                        }
                    }
                }
            }
        }

        res
    }
}
// @lc code=end

mod test {
    #[test]
    fn test_it() {
        use super::*;
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        )
    }
}
