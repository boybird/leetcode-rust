///! https://leetcode-cn.com/problems/median-of-two-sorted-arrays/

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut i, mut j,  k) = (0, 0, 0);
        let n = nums1.len();
        let m = nums2.len();



        let mut num = Vec::with_capacity(n + m);

        while i < n && j < m {
            if nums1[i] < nums2[j] {
                num.push(nums1[i]);
                i = i + 1;
            } else {
                num.push(nums2[j]);
                j = j + 1;
            }
        }
        while i < n {
            num.push(nums1[i]);
            i = i + 1;
        }
        while j < m {
            num.push(nums2[j]);
            j = j + 1;
        }
        let l = num.len() - 1;

        if l % 2 == 0 {
            num[l / 2] as f64
        } else {
            (num[l / 2] + num[l / 2 + 1]) as f64 / 2.0f64
        }
    }
}

#[test]
fn test_find_median_sorted_arrays() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.0f64
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5f64
    );
}
