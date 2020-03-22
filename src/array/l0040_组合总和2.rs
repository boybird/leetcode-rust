/*
 * @lc app=leetcode.cn id=40 lang=rust
 *
 * [40] 组合总和 II
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let len = candidates.len();
        let mut res = vec![];
        let mut path = VecDeque::new();
        Self::dfs(&candidates, len, target, 0, &mut path, &mut res);
        res
    }

    pub fn dfs(
        candidates: &Vec<i32>,
        len: usize,
        residue: i32,
        begin: usize,
        path: &mut VecDeque<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if residue == 0 {
            res.push(path.iter().map(|i| *i).collect());
        }
        for (idx, item) in candidates.iter().skip(begin).enumerate() {
            let i = begin + idx;
            if idx > 0 && *item == candidates[i - 1] {
                continue;
            }
            if residue - candidates[i] < 0 {
                break;
            }
            path.push_back(candidates[i]);
            Self::dfs(candidates, len, residue - candidates[i], i + 1, path, res);
            path.pop_back();
        }
    }
}
// @lc code=end

mod tests {
    #[test]
    fn test_it() {
        use super::*;

        
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let rs = Solution::combination_sum2(candidates, target);
        println!("{:?}", rs);
    }
}
