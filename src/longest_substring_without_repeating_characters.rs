struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut cur_sub = String::with_capacity(26);
        let (mut start, mut end, mut max, mut cur_result) = (0, 0, 0, 0);
        for c in s.chars() {
            if let Some(i) = cur_sub.find(|ch| ch == c) {
                start = start + i + 1;
                cur_result = cur_result - i;
                cur_sub = s.chars().skip(start).take(cur_result).collect();
            } else {
                cur_sub.push(c);
                cur_result = cur_result + 1;
                if cur_result > max {
                    max = cur_result;
                }
            }
            end = end + 1;
        }
        max as i32
    }
    pub fn length_of_longest_substring2(s: String) -> i32 {
        let mut hash = std::collections::HashMap::with_capacity(26);
        let (mut max, mut start, mut end, mut curr) = (0, 0, 0, 0);
        for (i, item) in s.chars().enumerate() {
            if let Some(j) = hash.get(&item) {
                if *j >= start {
                    curr = end - start;
                    if max < curr {
                        max = curr;
                    }
                    start = *j + 1;
                }
            }
            end += 1;
            hash.insert(item, i);
        }
        let curr = end - start;
        if max < curr {
            max = curr;
        }
        max as i32
    }
}

#[test]
fn test_lenght_of_longest_substring() {
    assert_eq!(
        Solution::length_of_longest_substring2("abcabcbb".to_owned()),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring2("pwwkew".to_owned()),
        3
    );
}
