/*
 * @lc app=leetcode.cn id=39 lang=rust
 *
 * [39] 组合总和
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
        for i in begin..len {
            if residue - candidates[i] < 0 {
                break;
            }
            path.push_back(candidates[i]);
            Self::dfs(candidates, len, residue - candidates[i], i, path, res);
            path.pop_back();
        }
    }

    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut all_ans: Vec<HashSet<Vec<i32>>> = vec![HashSet::new(); target as usize + 1];
        let mut candidates = candidates;
        candidates.sort();
        for i in 1..(target as usize + 1) {
            for j in 0..candidates.len() {
                let val = candidates[j];
                if val > i as i32 {
                    break;
                }
                let dif = i as i32 - val;
                if dif == 0 {
                    all_ans[i].insert(vec![val]);
                } else {
                    let cl = all_ans[dif as usize].clone();
                    for mut rec in cl {
                        rec.push(val);
                        rec.sort();
                        all_ans[i].insert(rec);
                    }
                }
            }
        }

        all_ans.remove(target as usize).into_iter().collect()
    }
}
// @lc code=end

mod tests {
    #[test]
    fn test_it() {
        use super::*;

        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let rs = Solution::combination_sum(candidates, target);
        println!("{:?}", rs);
        //assert_eq!(rs, vec![vec![7], vec![2, 2, 3]]);
    }
}
