/*
 * @lc app=leetcode.cn id=210 lang=rust
 *
 * [210] 课程表 II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let num_courses = num_courses as usize;
        let mut map = std::collections::HashMap::new();
        let mut degree = vec![0; num_courses];
        for side in prerequisites.iter() {
            map.entry(side[1])
                .and_modify(|e: &mut Vec<i32>| e.push(side[0]))
                .or_insert(vec![side[0]]);
            degree[side[0] as usize] += 1;
        }

        let mut q = std::collections::VecDeque::new();
        for i in 0..num_courses {
            if degree[i] == 0 {
                q.push_back(i as i32);
            }
        }
        loop {
            if q.is_empty() {
                break;
            }
            let cur = q.pop_front().unwrap();
            result.push(cur);

            match map.get(&cur) {
                Some(v) => {
                    for next in v {
                        let n = *next as usize;
                        degree[n] -= 1;
                        if degree[n] == 0 {
                            q.push_back(*next)
                        }
                    }
                }
                None => {}
            }
        }
        if result.len() == num_courses {
            result
        } else {
            vec![]
        }
    }
}
// @lc code=end
