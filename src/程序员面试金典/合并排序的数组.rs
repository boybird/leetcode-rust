struct Solution;

impl Solution {
    pub fn merge(a: &mut Vec<i32>, m: i32, b: &mut Vec<i32>, n: i32) {
        let mut total = (n + m) as usize;
        if total > a.len() {
            a.extend(vec![0; total - a.len()].iter());
        }

        let mut pa = (m) as usize;
        let mut pb = (n) as usize;
        while pa > 0 || pb > 0 {
            let cur;
            if pa == 0 {
                pb = pb - 1;
                cur = b[pb];
            } else if pb == 0 {
                pa = pa - 1;
                cur = a[pa];
            } else if a[pa - 1] > b[pb - 1] {
                pa = pa - 1;
                cur = a[pa];
            } else {
                pb = pb - 1;
                cur = b[pb];
            }
            total = total - 1;
            a[total] = cur;
        }
    }
    pub fn merge2(a: &mut Vec<i32>, m: i32, b: &mut Vec<i32>, n: i32) {
        let s = m as usize;
        let mut pb: usize = 0;
        while pb < n as usize {
            if a.len() <= s + pb {
                a.push(b[pb])
            } else {
                a[s + pb] = b[pb];
            }
            pb += 1;
        }
        a.sort()
    }
}

mod tests {
    #[test]
    fn testit() {
        use super::*;
        let mut a = vec![0];
        let mut b = vec![1];
        Solution::merge(&mut a, 0, &mut b, 1);
        println!("{:?}", a);
    }
}
