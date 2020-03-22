/*
 * @lc app=leetcode.cn id=218 lang=rust
 *
 * [218] 天际线问题
 */
struct Solution;
// @lc code=start
use std::collections::BTreeMap;
use std::collections::HashMap;
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if buildings.is_empty() {
            return vec![];
        }

        //横坐标排序    (x,LR,bt)
        let mut b: Vec<(i32, i32, i32)> = vec![];
        for e in buildings.iter() {
            b.push((e[0], 0, e[2]));
            b.push((e[1], 1, e[2]));
        }
        b.sort();

        let mut z = HashMap::new(); //存 关键点
        let mut bt = BTreeMap::new(); //当作MultiSet, 还能快速找到最大值
        bt.insert(0, 1); // 地平线
        for &(x, lr, h) in b.iter() {
            if lr == 0 {
                // 左边线 插入
                bt.entry(h).and_modify(|t| *t += 1).or_insert(1);
            } else {
                // 右边线 删除
                *bt.get_mut(&h).unwrap() -= 1;
                if bt[&h] == 0 {
                    bt.remove(&h);
                }
            }

            // 可以的关键点 (x, maxh)
            // 条件是 插入的或删除的 那个原来的 h 要比 maxh 大(或等)
            // 如果之前有相同高度的关键点就直接跳过
            let maxh = *bt.keys().rev().next().unwrap();
            if h >= maxh {
                z.insert(x, maxh);
            }
        }

        // 排序
        let mut z1: Vec<_> = z.into_iter().map(|(x, h)| vec![x, h]).collect();
        z1.sort();
        // 去重
        let mut pre = -1;
        z1.into_iter()
            .filter(|x| {
                if x[1] != pre {
                    pre = x[1];
                    true
                } else {
                    false
                }
            })
            .collect()
    }
}
// @lc code=end

mod tests {

    #[test]
    fn testit() {
        use super::*;
        let buildings = vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8],
        ];
        dbg!(Solution::get_skyline(buildings));
    }
}
