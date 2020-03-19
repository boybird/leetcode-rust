/*
 * @lc app=leetcode.cn id=207 lang=rust
 *
 * [207] 课程表
 */
// depth-first-search | breadth-first-search | graph | topological-sort
struct Solution;

// @lc code=start
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut map = std::collections::HashMap::new();
        let mut degree = vec![0; num_courses];
        for side in prerequisites.iter() {
            map.entry(side[1])
                .and_modify(|e: &mut Vec<i32>| e.push(side[0]))
                .or_insert(vec![side[0]]);
            degree[side[0] as usize] += 1;
        }
        let mut count = 0;
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
            count += 1;

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
        count == num_courses
    }
}
// @lc code=end

mod tests {

    #[test]
    fn test_it() {
        use super::*;
        dbg!(Solution::can_finish(2, vec![vec![1, 0]]));
    }
}
