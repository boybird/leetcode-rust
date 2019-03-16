struct Solution {}
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let y_len = matrix.len();
        let x_len = matrix[0].len();
        for y in 0..y_len {
            for x in 0..x_len {
                if matrix[y][x] == '1' {}
            }
        }
        0
    }
}
#[test]
fn test_maximal_square() {
    assert_eq!(
        Solution::maximal_square(vec![
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ]),
        4
    );
}
