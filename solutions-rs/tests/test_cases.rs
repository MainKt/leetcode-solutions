use solutions_rs::{add_two_numbers::ListNode, Solution};

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

#[test]
fn add_two_number_case_1() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));

    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let l3 = Solution::add_two_numbers(l1, l2);

    assert_eq!(
        l3,
        Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }))
    )
}

#[test]
fn coin_change_case_1() {
    let coins = vec![1, 2, 5];
    let amount = 11;
    let count = Solution::coin_change(coins, amount);
    assert_eq!(count, 3);
}

#[test]
fn maximum_subarray_case_1() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = 6;
    assert_eq!(Solution::max_sub_array(nums), max_sum);
}

#[test]
fn valid_sudoku_case_1() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert!(Solution::is_valid_sudoku(board));
}

#[test]
fn valid_parentheses_case_1() {
    let parentheses = "()".to_string();
    assert!(Solution::is_valid(parentheses));
}

#[test]
fn longest_increasing_subsequence_case_1() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(Solution::length_of_lis(nums), 4);
}

#[test]
fn longest_common_subsequence_case_1() {
    let text1 = "abcde";
    let text2 = "ace";

    assert_eq!(
        Solution::longest_common_subsequence(text1.into(), text2.into()),
        3
    );
}

#[test]
fn longest_common_subsequence_case_2() {
    let text1 = "bl";
    let text2 = "yby";

    assert_eq!(
        Solution::longest_common_subsequence(text1.into(), text2.into()),
        1
    );
}

#[test]
fn longest_common_subsequence_case_3() {
    let text1 = "pmjghexybyrgzczy";
    let text2 = "hafcdqbgncrcbihkd";

    assert_eq!(
        Solution::longest_common_subsequence(text1.into(), text2.into()),
        4
    );
}

#[test]
fn roman_to_integer_case_1() {
    let roman = "LVIII".to_string();
    assert_eq!(Solution::roman_to_int(roman), 58);
}

#[test]
fn longest_common_prefix_case_1() {
    let strs: Vec<_> = vec!["flower", "flow", "flight"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string());
}
