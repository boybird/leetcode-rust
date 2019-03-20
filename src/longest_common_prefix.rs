struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_owned();
        }
        let str0 = &strs[0];
        let str0bytes = str0.as_bytes();
        let mut len = str0.len();
        for str in &strs[1..] {
            len = std::cmp::min(
                len,
                str.as_bytes()
                    .iter()
                    .zip(str0bytes)
                    .take_while(|&(a, b)| a == b)
                    .count(),
            );
        }
        String::from(&strs[0][..len])
    }
}

#[test]
fn test_longest_common_prefix() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "flower".to_owned(),
            "flow".to_owned(),
            "flight".to_owned(),
        ]),
        "fl".to_owned()
    )
}
