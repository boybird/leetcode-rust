/*
 * @lc app=leetcode.cn id=44 lang=rust
 *
 * [44] 通配符匹配
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.eq(&s) || p.eq("*") {
            return true;
        }
        if p.is_empty() || s.is_empty() {
            return false;
        }

        let s_len = s.len();
        let p_len = p.len();
        let mut res = vec![vec![false; s_len + 1]; p_len + 1];
        res[0][0] = true;
        for pi in 1..=p_len {
            if p.chars().nth(pi - 1) == Some('*') {
                let mut si = 1;
                while !res[pi - 1][si - 1] && si < s_len + 1 {
                    si += 1;
                }
                res[pi][si - 1] = res[pi - 1][si - 1];
                while si < s_len + 1 {
                    res[pi][si] = true;
                    si += 1;
                }
            } else if p.chars().nth(pi - 1) == Some('?') {
                for si in 1..=s_len {
                    res[pi][si] = res[pi - 1][si - 1];
                }
            } else {
                for si in 1..=s_len {
                    res[pi][si] = res[pi - 1][si - 1]
                        && (p.chars().nth(pi - 1) == s.chars().nth(si - 1))
                }
            }
        }

        res[p_len][s_len]
    }
}
// @lc code=end

mod tests {

    #[test]
    fn test_it() {
        use super::*;
        assert_eq!(
            Solution::is_match(String::from("cb"), String::from("?a")),
            false
        );

        assert_eq!(
            Solution::is_match(String::from("adceb"), String::from("*a*b")),
            true
        );

        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("a"),),
            false
        );
        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("*"),),
            true
        );
        assert_eq!(
            Solution::is_match(String::from("aaacdcb"), String::from("a*c?b"),),
            false
        );
    }
}
