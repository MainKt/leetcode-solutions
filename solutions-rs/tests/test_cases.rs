use solutions_rs::Solution;

#[test]
fn merge_case_1() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;

    let mut nums2 = vec![2, 5, 6];
    let n = 3;

    Solution::merge(&mut nums1, m, &mut nums2, n);

    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6])
}

#[test]
fn two_sum_case_1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let indices = Solution::two_sum(nums, target);
    assert_eq!(indices, [0, 1]);
}
