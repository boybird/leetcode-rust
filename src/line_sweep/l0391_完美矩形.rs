/*
 * @lc app=leetcode.cn id=391 lang=rust
 *
 * vec![391] 完美矩形
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut x1 = std::i32::MAX;
        let mut y1 = std::i32::MAX;
        let mut x2 = std::i32::MIN;
        let mut y2 = std::i32::MIN;
        let mut area = 0;
        let mut nodes = std::collections::HashSet::new();
        for p in rectangles.iter() {
            x1 = std::cmp::min(x1, p[0]);
            y1 = std::cmp::min(y1, p[1]);
            x2 = std::cmp::max(x2, p[2]);
            y2 = std::cmp::max(y2, p[3]);

            area += (p[2] - p[0]) * (p[3] - p[1]).abs();

            let p_0 = (p[0], p[1]);
            if nodes.contains(&p_0) {
                nodes.remove(&p_0);
            } else {
                nodes.insert(p_0);
            }
            let p_1 = (p[0], p[3]);
            if nodes.contains(&p_1) {
                nodes.remove(&p_1);
            } else {
                nodes.insert(p_1);
            }
            let p_2 = (p[2], p[1]);
            if nodes.contains(&p_2) {
                nodes.remove(&p_2);
            } else {
                nodes.insert(p_2);
            }
            let p_3 = (p[2], p[3]);
            if nodes.contains(&p_3) {
                nodes.remove(&p_3);
            } else {
                nodes.insert(p_3);
            }
        }

        if area != ((x2 - x1) * (y2 - y1)).abs() {
            return false;
        }

        nodes.contains(&(x1, y1))
            && nodes.contains(&(x1, y2))
            && nodes.contains(&(x2, y1))
            && nodes.contains(&(x2, y2))
            && nodes.len() == 4
    }
}
// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;
        let r0 = vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4],
            vec![1, 3, 2, 4],
            vec![2, 3, 3, 4],
        ];
        assert_eq!(dbg!(Solution::is_rectangle_cover(r0)), true);

        let r1 = vec![
            vec![1, 1, 2, 3],
            vec![1, 3, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4],
        ];
        assert_eq!(dbg!(Solution::is_rectangle_cover(r1)), false);

        let r2 = vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![1, 3, 2, 4],
            vec![3, 2, 4, 4],
        ];
        assert_eq!(dbg!(Solution::is_rectangle_cover(r2)), false);

        let r3 = vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![1, 3, 2, 4],
            vec![2, 2, 4, 4],
        ];
        assert_eq!(dbg!(Solution::is_rectangle_cover(r3)), false);
    }
}
