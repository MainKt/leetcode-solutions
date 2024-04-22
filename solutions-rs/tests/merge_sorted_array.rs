#[test]
fn case1() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;

    let mut nums2 = vec![2, 5, 6];
    let n = 3;

    solutions_rs::Solution::merge(&mut nums1, m, &mut nums2, n);

    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6])
}
