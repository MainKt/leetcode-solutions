use solutions_rs::Solution;

#[test]
fn two_sum_case_1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let indices = Solution::two_sum(nums, target);
    assert_eq!(indices, [0, 1]);
}
