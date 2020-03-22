/*
 * @lc app=leetcode.cn id=48 lang=rust
 *
 * [48] 旋转图像
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for i in 0..(n / 2) {
            for j in i..(n - i - 1) {
                let mut p = i;
                let mut q = j;
                let mut prev = matrix[p][q];
                for _ in 0..3 {
                    let next = (((p * n + q + 1) * n) % (n * n + 1)) - 1;
                    let tmp = matrix[next / n][next % n];
                    matrix[next / n][next % n] = prev;
                    prev = tmp;
                    p = next / n;
                    q = next % n;
                }
                matrix[i][j] = prev;
            }
        }
    }
}
// @lc code=end

mod tests {

    #[test]
    fn test_it() {
        use super::*;
        let mut v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        println!("{:?}", v);
        Solution::rotate(&mut v);
        println!("{:?}", v);

        let mut v = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];

        println!("{:?}", v);
        Solution::rotate(&mut v);
        println!("{:?}", v);
    }
}
