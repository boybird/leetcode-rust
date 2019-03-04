struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut r = vec![];
        Self::search(n, n, "".to_owned(), &mut r);
        r
    }
    fn search(left: i32, right: i32, path: String, r: &mut Vec<String>) {
        if left == 0 && right == 0 {
            r.push(path.clone());
        }
        if left > 0 {
            let mut p = path.clone();
            p.push('(');
            Self::search(left - 1, right, p, r);
        }
        if left < right {
            let mut p = path.clone();
            p.push(')');
            Self::search(left, right - 1, p, r);
        }
    }
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec![
            "((()))".to_owned(),
            "(()())".to_owned(),
            "(())()".to_owned(),
            "()(())".to_owned(),
            "()()()".to_owned(),
        ]
    );
}
