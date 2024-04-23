use crate::Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut i, mut j, mut k) = (m - 1, n - 1, m + n - 1);
        while i >= 0 && j >= 0 {
            nums1[k as usize] = if nums1[i as usize] > nums2[j as usize] {
                let num = nums1[i as usize];
                i -= 1;
                num
            } else {
                let num = nums2[j as usize];
                j -= 1;
                num
            };
            k -= 1;
        }

        while i >= 0 {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
            k -= 1;
        }

        while j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}
