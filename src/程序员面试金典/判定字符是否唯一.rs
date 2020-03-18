struct Solution;

impl Solution {
    pub fn is_unique(astr: String) -> bool {
        let mut set = std::collections::HashSet::<char>::new();

        for c in astr.chars() {
            if set.contains(&c) {
                return false;
            }
            set.insert(c);
        }
        true
    }
}
