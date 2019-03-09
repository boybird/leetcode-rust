struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".to_owned();
        }
        if s.len() == 1 {
            return s;
        }
        let (mut min_start, mut max_len) = (0, 1);
        let mut i = 0;
        while i < s.len() {
            if s.len() - i < max_len / 2 {
                break;
            }
            let (mut j, mut k) = (i, i);
            while k < s.len() - 1 && s.chars().nth(k) == s.chars().nth(k + 1) {
                k = k + 1;
            }
            i = k + 1;
            while k < s.len() - 1 && j > 0 && s.chars().nth(k + 1) == s.chars().nth(j - 1) {
                k = k + 1;
                j = j - 1;
            }
            let new_len = k - j + 1;
            if new_len > max_len {
                min_start = j;
                max_len = new_len;
            }
        }
        return String::from(&s[min_start..(min_start + max_len)]);
    }
}

#[test]
fn test_longest_palindromic_substringv() {
    assert_eq!(
        Solution::longest_palindrome("cbbd".to_owned()),
        "bb".to_owned()
    );
}
