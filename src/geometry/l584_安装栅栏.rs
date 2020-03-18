/*
 * @lc app=leetcode.cn id=587 lang=rust
 *
 * vec![587] 安装栅栏
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn outer_trees(points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut hul = std::collections::HashSet::new();
        if points.len() < 4 {
            return points;
        }
        let mut left_most = 0;
        for i in 0..points.len() {
            if points[i][0] < points[left_most][0] {
                left_most = i;
            }
        }
        let mut p = left_most;

        loop {
            let mut q = (p + 1) % points.len();
            for i in 0..points.len() {
                if Self::orientation(&points[p], &points[i], &points[q]) < 0 {
                    q = i;
                }
            }
            for i in 0..points.len() {
                if i != p
                    && i != q
                    && Self::orientation(&points[p], &points[i], &points[q]) == 0
                    && Self::inBetween(&points[p], &points[i], &points[q])
                {
                    hul.insert(points[i].clone());
                }
            }
            hul.insert(points[q].clone());
            p = q;
            if p == left_most {
                break;
            }
        }
        hul.into_iter().collect()
    }

    fn orientation(p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>) -> i32 {
        (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] - q[1])
    }
    fn inBetween(p: &Vec<i32>, i: &Vec<i32>, q: &Vec<i32>) -> bool {
        let a = i[0] >= p[0] && i[0] <= q[0] || i[0] <= p[0] && i[0] >= q[0];
        let b = i[1] >= p[1] && i[1] <= q[1] || i[1] <= p[1] && i[1] >= q[1];
        a && b
    }
}

// @lc code=end

mod tests {
    #[test]
    fn testit() {
        use super::*;
        let v = vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2],
        ];
        let vs = vec![vec![1, 1], vec![2, 0], vec![4, 2], vec![3, 3], vec![2, 4]];
        assert_eq!(Solution::outer_trees(v), vs);

        let v1 = vec![vec![1, 2], vec![2, 2], vec![4, 2]];
        let vs1 = vec![vec![1, 2], vec![2, 2], vec![4, 2]];
        assert_eq!(Solution::outer_trees(v1), vs1);
    }
}
