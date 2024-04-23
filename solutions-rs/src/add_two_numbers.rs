use crate::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        add(l1, l2, 0)
    }
}

fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }
    let val1 = l1.as_ref().map_or(0, |l| l.val);
    let val2 = l2.as_ref().map_or(0, |l| l.val);
    let sum = val1 + val2 + carry;
    let carry = sum / 10;
    let sum = sum % 10;
    Some(Box::new(ListNode {
        val: sum,
        next: add(
            l1.map_or(None, |l| l.next),
            l2.map_or(None, |l| l.next),
            carry,
        ),
    }))
}
